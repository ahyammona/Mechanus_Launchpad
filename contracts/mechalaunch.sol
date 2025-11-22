// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Interface for Uniswap V2
interface IUniswapV2Router02 {
    function factory() external pure returns (address);
    function WETH() external pure returns (address);
    function addLiquidityETH(
        address token,
        uint amountTokenDesired,
        uint amountTokenMin,
        uint amountETHMin,
        address to,
        uint deadline
    ) external payable returns (uint amountToken, uint amountETH, uint liquidity);
}

interface IUniswapV2Factory {
    function getPair(address tokenA, address tokenB) external view returns (address pair);
}

contract MechaToken is ERC20, Ownable {
    uint256 public constant MAX_SUPPLY = 100_000_000 * 10**18;

    constructor(
        string memory name,
        string memory symbol,
        uint256 initialSupply,
        address projectOwner
    ) ERC20(name, symbol) Ownable(projectOwner) {
        _mint(projectOwner, initialSupply);
    }

    function mint(address to, uint256 amount) external onlyOwner {
        require(totalSupply() + amount <= MAX_SUPPLY, "Exceeds max supply");
        _mint(to, amount);
    }
}

contract MechaLaunchpad is ReentrancyGuard, Ownable {
    using SafeERC20 for IERC20;

    enum ProjectStatus { PENDING, APPROVED, REJECTED, ACTIVE, COMPLETED, CANCELLED, LIQUIDITY_ADDED }
    enum Tier { NONE, BRONZE, SILVER, GOLD, PLATINUM }

    struct Project {
        address id;                // Token address as project ID
        address creator;
        string name;
        address offeringToken;     // Same as id
        uint256 softCap;
        uint256 hardCap;
        uint256 minInvestment;
        uint256 maxInvestment;
        uint256 startTime;
        uint256 endTime;
        uint256 totalRaised;
        uint256 totalInvestors;
        uint256 tokenPrice;        // wei per token (token has 18 decimals)
        uint256 tokensForSale;     // remaining tokens available to buyers
        uint256 liquidityPercent;  // basis points (5000 = 50%)
        uint256 liquidityLockTime;
        address lpTokenAddress;
        uint256 lpTokensLocked;
        ProjectStatus status;
        bool fundsClaimed;
        bool liquidityAdded;
    }

    struct ProjectDetails {
        string description;
        string website;
        string whitepaper;
    }

    struct Investment {
        uint256 amount;        // ETH invested (wei)
        uint256 tokenAmount;   // tokens allocated (with 18 decimals)
        uint256 timestamp;
        bool refunded;
        bool tokensClaimed;
    }

    struct UserInfo {
        Tier tier;
        uint256 totalInvested;
        uint256 totalProjectsInvested;
        uint256 joinDate;
    }

    struct LiquidityLock {
        address lpToken;
        uint256 amount;
        uint256 unlockTime;
        bool claimed;
        address projectId;     // Changed to address to match project ID
    }

    // State variables - using address as key for projects
    mapping(address => Project) public projects;
    mapping(address => ProjectDetails) public projectDetails;
    mapping(address => mapping(address => Investment)) public investments; // projectId -> investor -> investment
    mapping(address => UserInfo) public users;
    mapping(address => bool) public whitelistedTokens;
    mapping(uint256 => LiquidityLock) public liquidityLocks; // Keep uint256 for lock IDs
    mapping(address => uint256[]) public userLocks;

    // Uniswap V2
    address public constant UNISWAP_ROUTER = 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;
    address public constant UNISWAP_FACTORY = 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f;
    
    uint256 public projectCount;
    uint256 public totalRaisedAllProjects;
    uint256 public totalInvestors;
    uint256 public lockCounter;
    
    uint256 public platformFee = 200; // 2%
    uint256 public liquidityFee = 100; // 1%
    address public feeWallet;
    uint256 public constant FEE_DENOMINATOR = 10000;

    // Events - using address for projectId
    event ProjectCreated(address indexed projectId, address indexed creator, string name);
    event ProjectStatusChanged(address indexed projectId, ProjectStatus status);
    event Invested(address indexed projectId, address indexed investor, uint256 amount, Tier tier);
    event Refunded(address indexed projectId, address indexed investor, uint256 amount);
    event FundsClaimed(address indexed projectId, address indexed creator, uint256 amount);
    event LiquidityAdded(address indexed projectId, uint256 ethAmount, uint256 tokenAmount, address lpToken);
    event TokensClaimed(address indexed projectId, address indexed investor, uint256 tokenAmount);
    event UserTierUpdated(address indexed user, Tier tier);
    event PlatformFeeUpdated(uint256 newFee);
    event WhitelistTokenUpdated(address token, bool whitelisted);
    event LiquidityLocked(uint256 indexed lockId, address indexed lpToken, uint256 amount, uint256 unlockTime);
    event CreatorAllocated(address indexed projectId, address indexed creator, uint256 tokenAmount);

    modifier onlyProjectCreator(address _projectId) {
        require(projects[_projectId].creator == msg.sender, "Not project creator");
        _;
    }

    modifier validProject(address _projectId) {
        require(_projectId != address(0) && projects[_projectId].creator != address(0), "Invalid project");
        _;
    }

    modifier onlyActiveProject(address _projectId) {
        require(projects[_projectId].status == ProjectStatus.ACTIVE, "Project not active");
        _;
    }

    constructor(address _feeWallet) Ownable(msg.sender) {
        require(_feeWallet != address(0), "Invalid fee wallet");
        feeWallet = _feeWallet;
    }

    /**
     * @dev Create a new project using token address as project ID
     */
    function createProject(
        string memory _name,
        string memory _description,
        string memory _website,
        string memory _whitepaper,
        address _offeringToken,
        uint256 _softCap,
        uint256 _hardCap,
        uint256 _minInvestment,
        uint256 _maxInvestment,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _tokenPrice,
        uint256 _tokensForSale,
        uint256 _liquidityPercent,
        uint256 _liquidityLockTime,
        uint256 _creatorAllocationBP
    ) external returns (address) {
        require(bytes(_name).length > 0, "Name required");
        require(_softCap > 0 && _hardCap > _softCap, "Invalid caps");
        require(_minInvestment > 0 && _maxInvestment > _minInvestment, "Invalid investment limits");
        require(_startTime > block.timestamp && _endTime > _startTime, "Invalid timing");
        require(_offeringToken != address(0), "Invalid token address");
        require(_tokenPrice > 0, "Invalid token price");
        require(_tokensForSale > 0, "Invalid tokens for sale");
        require(_liquidityPercent >= 5000 && _liquidityPercent <= 8000, "Liquidity percent 50-80%");
        require(_liquidityLockTime >= 30 days, "Lock time too short");
        require(_creatorAllocationBP <= 5000, "Creator allocation too high");
        require(projects[_offeringToken].creator == address(0), "Project already exists");

        // Calculate required tokens
        uint256 tokensForLiquidity = (_tokensForSale * _liquidityPercent) / FEE_DENOMINATOR;
        uint256 requiredTokens = _tokensForSale + tokensForLiquidity;

        // Ensure creator has enough tokens and transfer to contract
        IERC20 offering = IERC20(_offeringToken);
        require(offering.balanceOf(msg.sender) >= requiredTokens, "Insufficient tokens");
        offering.safeTransferFrom(msg.sender, address(this), requiredTokens);

        projectCount++;
        address projectId = _offeringToken; // Use token address as project ID

        // Create project
        projects[projectId] = Project({
            id: projectId,
            creator: msg.sender,
            name: _name,
            offeringToken: _offeringToken,
            softCap: _softCap,
            hardCap: _hardCap,
            minInvestment: _minInvestment,
            maxInvestment: _maxInvestment,
            startTime: _startTime,
            endTime: _endTime,
            totalRaised: 0,
            totalInvestors: 0,
            tokenPrice: _tokenPrice,
            tokensForSale: _tokensForSale,
            liquidityPercent: _liquidityPercent,
            liquidityLockTime: _liquidityLockTime,
            lpTokenAddress: address(0),
            lpTokensLocked: 0,
            status: ProjectStatus.PENDING,
            fundsClaimed: false,
            liquidityAdded: false
        });

        // Store project details
        projectDetails[projectId] = ProjectDetails({
            description: _description,
            website: _website,
            whitepaper: _whitepaper
        });

        // Initialize user
        _initializeUser(msg.sender);

        // Handle creator allocation
        if (_creatorAllocationBP > 0) {
            uint256 creatorTokens = (_tokensForSale * _creatorAllocationBP) / FEE_DENOMINATOR;
            projects[projectId].tokensForSale -= creatorTokens;

            // Set up creator investment record
            investments[projectId][msg.sender] = Investment({
                amount: 0,
                tokenAmount: creatorTokens,
                timestamp: block.timestamp,
                refunded: false,
                tokensClaimed: false
            });

            // Transfer creator tokens immediately
            offering.safeTransfer(msg.sender, creatorTokens);

            // Update stats
            projects[projectId].totalInvestors += 1;
            users[msg.sender].totalProjectsInvested += 1;

            emit CreatorAllocated(projectId, msg.sender, creatorTokens);
        }

        emit ProjectCreated(projectId, msg.sender, _name);
        return projectId;
    }

    /**
     * @dev Invest in a project
     */
    function invest(address _projectId) external payable nonReentrant validProject(_projectId) onlyActiveProject(_projectId) {
        Project storage project = projects[_projectId];
        require(block.timestamp >= project.startTime && block.timestamp <= project.endTime, "Not in investment period");
        require(msg.value >= project.minInvestment, "Below min investment");

        Investment storage userInvestment = investments[_projectId][msg.sender];
        uint256 totalInvestment = userInvestment.amount + msg.value;
        require(totalInvestment <= project.maxInvestment, "Exceeds max investment");

        // Calculate token amount
        uint256 tokenAmount = (msg.value * 10**18) / project.tokenPrice;
        require(tokenAmount > 0, "Invalid token amount");
        require(project.tokensForSale >= tokenAmount, "Insufficient tokens for sale");

        // Update investment
        if (userInvestment.amount == 0) {
            project.totalInvestors++;
        }
        
        userInvestment.amount = totalInvestment;
        userInvestment.tokenAmount += tokenAmount;
        userInvestment.timestamp = block.timestamp;

        // Update project
        project.totalRaised += msg.value;
        project.tokensForSale -= tokenAmount;
        require(project.totalRaised <= project.hardCap, "Exceeds hard cap");

        // Update user stats
        _updateUserStats(msg.sender, msg.value, userInvestment.amount == msg.value);

        emit Invested(_projectId, msg.sender, msg.value, users[msg.sender].tier);

        // Auto-complete if hard cap reached
        if (project.totalRaised >= project.hardCap) {
            _completeProject(_projectId);
        }
    }

    function _updateUserStats(address investor, uint256 value, bool isFirstInvestment) internal {
        UserInfo storage user = users[investor];
        if (user.joinDate == 0) {
            _initializeUser(investor);
        }

        user.totalInvested += value;

        if (isFirstInvestment) {
            user.totalProjectsInvested++;
        }

        totalRaisedAllProjects += value;
        if (user.totalInvested == value) {
            totalInvestors++;
        }

        _updateUserTier(investor);
    }

    function _initializeUser(address _user) internal {
        users[_user] = UserInfo({
            tier: Tier.NONE,
            totalInvested: 0,
            totalProjectsInvested: 0,
            joinDate: block.timestamp
        });
    }

    /**
     * @dev Complete project and add liquidity
     */
    function _completeProject(address _projectId) internal {
        Project storage project = projects[_projectId];
        project.status = ProjectStatus.COMPLETED;

        uint256 liquidityETH = (project.totalRaised * project.liquidityPercent) / FEE_DENOMINATOR;
        uint256 liquidityTokens = (liquidityETH * 10**18) / project.tokenPrice;

        _addLiquidity(_projectId, liquidityETH, liquidityTokens);
        _distributeFunds(_projectId);
    }

    function _addLiquidity(address _projectId, uint256 _ethAmount, uint256 _tokenAmount) internal {
        Project storage project = projects[_projectId];
        
        IERC20(project.offeringToken).approve(UNISWAP_ROUTER, _tokenAmount);
        
        IUniswapV2Router02 router = IUniswapV2Router02(UNISWAP_ROUTER);
        
        (uint256 amountToken, uint256 amountETH, uint256 liquidity) = router.addLiquidityETH{value: _ethAmount}(
            project.offeringToken,
            _tokenAmount,
            _tokenAmount,
            _ethAmount,
            address(this),
            block.timestamp + 1 hours
        );

        address lpToken = IUniswapV2Factory(UNISWAP_FACTORY).getPair(project.offeringToken, router.WETH());

        project.lpTokenAddress = lpToken;
        project.liquidityAdded = true;
        project.status = ProjectStatus.LIQUIDITY_ADDED;

        _lockLPTokens(_projectId, lpToken, liquidity);

        emit LiquidityAdded(_projectId, amountETH, amountToken, lpToken);
    }

    function _lockLPTokens(address _projectId, address _lpToken, uint256 _amount) internal {
        Project storage project = projects[_projectId];
        
        lockCounter++;
        liquidityLocks[lockCounter] = LiquidityLock({
            lpToken: _lpToken,
            amount: _amount,
            unlockTime: block.timestamp + project.liquidityLockTime,
            claimed: false,
            projectId: _projectId
        });

        project.lpTokensLocked = _amount;
        emit LiquidityLocked(lockCounter, _lpToken, _amount, block.timestamp + project.liquidityLockTime);
    }

    function _distributeFunds(address _projectId) internal {
        Project storage project = projects[_projectId];
        
        uint256 totalFees = (project.totalRaised * (platformFee + liquidityFee)) / FEE_DENOMINATOR;
        uint256 creatorShare = project.totalRaised - totalFees;

        payable(project.creator).transfer(creatorShare);
        payable(feeWallet).transfer(totalFees);

        project.fundsClaimed = true;
        emit FundsClaimed(_projectId, project.creator, creatorShare);
    }

    /**
     * @dev Claim purchased tokens
     */
    function claimTokens(address _projectId) external nonReentrant validProject(_projectId) {
        Project storage project = projects[_projectId];
        require(project.status == ProjectStatus.LIQUIDITY_ADDED, "Liquidity not added");

        Investment storage userInvestment = investments[_projectId][msg.sender];
        require(userInvestment.tokenAmount > 0, "No tokens to claim");
        require(!userInvestment.tokensClaimed, "Tokens already claimed");

        userInvestment.tokensClaimed = true;
        IERC20(project.offeringToken).safeTransfer(msg.sender, userInvestment.tokenAmount);

        emit TokensClaimed(_projectId, msg.sender, userInvestment.tokenAmount);
    }

    /**
     * @dev Claim locked LP tokens
     */
    function claimLockedLPTokens(uint256 _lockId) external nonReentrant {
        LiquidityLock storage lock = liquidityLocks[_lockId];
        require(lock.amount > 0, "Invalid lock");
        require(block.timestamp >= lock.unlockTime, "Still locked");
        require(!lock.claimed, "Already claimed");
        require(projects[lock.projectId].creator == msg.sender, "Not project creator");

        lock.claimed = true;
        IERC20(lock.lpToken).safeTransfer(msg.sender, lock.amount);
    }

    /**
     * @dev Manual project completion
     */
    function completeProject(address _projectId) external onlyProjectCreator(_projectId) validProject(_projectId) {
        Project storage project = projects[_projectId];
        require(block.timestamp > project.endTime, "Funding not ended");
        require(project.status == ProjectStatus.ACTIVE, "Project not active");
        require(project.totalRaised >= project.softCap, "Soft cap not reached");

        _completeProject(_projectId);
    }

    /**
     * @dev Get refund if project fails
     */
    function claimRefund(address _projectId) external nonReentrant validProject(_projectId) {
        Project storage project = projects[_projectId];
        require(block.timestamp > project.endTime, "Funding not ended");
        require(project.totalRaised < project.softCap, "Soft cap reached");
        require(project.status != ProjectStatus.COMPLETED, "Project completed");
        
        Investment storage userInvestment = investments[_projectId][msg.sender];
        require(userInvestment.amount > 0, "No investment");
        require(!userInvestment.refunded, "Already refunded");

        userInvestment.refunded = true;
        payable(msg.sender).transfer(userInvestment.amount);

        // Return tokens to pool
        project.tokensForSale += userInvestment.tokenAmount;

        emit Refunded(_projectId, msg.sender, userInvestment.amount);
    }

    function _updateUserTier(address _user) internal {
        UserInfo storage user = users[_user];
        Tier newTier = _calculateTier(user.totalInvested);

        if (user.tier != newTier) {
            user.tier = newTier;
            emit UserTierUpdated(_user, newTier);
        }
    }

    function _calculateTier(uint256 totalInvested) internal pure returns (Tier) {
        if (totalInvested >= 25 ether) return Tier.PLATINUM;
        if (totalInvested >= 10 ether) return Tier.GOLD;
        if (totalInvested >= 5 ether) return Tier.SILVER;
        if (totalInvested >= 1 ether) return Tier.BRONZE;
        return Tier.NONE;
    }

    // View functions
    function getProjectFullDetails(address _projectId) external view validProject(_projectId) returns (Project memory, ProjectDetails memory) {
        return (projects[_projectId], projectDetails[_projectId]);
    }

    function getProjectProgress(address _projectId) external view validProject(_projectId) returns (uint256 remaining, uint256 progress) {
        Project memory project = projects[_projectId];
        remaining = project.hardCap - project.totalRaised;
        progress = (project.totalRaised * 100) / project.hardCap;
        return (remaining, progress);
    }

    function getUserAllocation(address _projectId, address _user) external view returns (uint256 investmentAmount, uint256 tokenAmount, bool claimed) {
        Investment memory investment = investments[_projectId][_user];
        return (investment.amount, investment.tokenAmount, investment.tokensClaimed);
    }

    // Admin functions
    function setFees(uint256 _platformFee, uint256 _liquidityFee) external onlyOwner {
        require(_platformFee <= 500, "Platform fee too high");
        require(_liquidityFee <= 200, "Liquidity fee too high");
        platformFee = _platformFee;
        liquidityFee = _liquidityFee;
        emit PlatformFeeUpdated(_platformFee);
    }

    function updateProjectStatus(address _projectId, ProjectStatus _status) external onlyOwner validProject(_projectId) {
        projects[_projectId].status = _status;
        emit ProjectStatusChanged(_projectId, _status);
    }

    function whitelistToken(address _token, bool _whitelisted) external onlyOwner {
        whitelistedTokens[_token] = _whitelisted;
        emit WhitelistTokenUpdated(_token, _whitelisted);
    }

    receive() external payable {}
}