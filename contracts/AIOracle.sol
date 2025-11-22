// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

/**
 * AI Agent Wallet (minimal, auditable, signature-authorized)
 * - Holds ERC20 tokens and native funds
 * - Can execute arbitrary calls when authorized by the AI "brainbox" via ECDSA signature or direct brainbox address
 * - Enforces a configurable daily token spend limit
 * - Keeps on-chain transparency: all actions emit events
 *
 * NOTES:
 * - This contract is intentionally small and observable. For production use you should add
 *   governance, timelocks, multisig fallback, and thorough security/audit.
 */

interface IERC20Minimal {
    function balanceOf(address account) external view returns (uint256);
    function transfer(address recipient, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    function approve(address spender, uint256 amount) external returns (bool);
}

contract AIAgentWallet {
    address public owner;         // deployer / project owner
    address public brainbox;      // off-chain AI signer (or an onchain oracle)
    IERC20Minimal public token;   // AICat token (or any ERC20)

    // daily spend control per-token (keyed by token address). value is limit in token wei
    mapping(address => uint256) public dailyLimit;
    // tracking spent per-day (token => day => amount)
    mapping(address => mapping(uint256 => uint256)) public spentToday;

    // replay protection for ECDSA-signed instructions
    mapping(address => uint256) public nonces;

    event OwnerChanged(address indexed oldOwner, address indexed newOwner);
    event BrainboxChanged(address indexed oldBrainbox, address indexed newBrainbox);
    event DailyLimitSet(address indexed token, uint256 limit);
    event Executed(address indexed executor, address indexed to, uint256 value, bytes data);
    event TokenTransferred(address indexed token, address indexed to, uint256 amount);
    event NativeWithdraw(address indexed to, uint256 amount);

    modifier onlyOwner() {
        require(msg.sender == owner, "AIAgent: only owner");
        _;
    }

    modifier onlyBrainboxOrOwner() {
        require(msg.sender == brainbox || msg.sender == owner, "AIAgent: only brainbox or owner");
        _;
    }

    constructor(address _token, address _brainbox) {
        owner = msg.sender;
        token = IERC20Minimal(_token);
        brainbox = _brainbox;
    }

    // owner controls
    function setOwner(address _new) external onlyOwner {
        require(_new != address(0), "zero");
        emit OwnerChanged(owner, _new);
        owner = _new;
    }

    function setBrainbox(address _brainbox) external onlyOwner {
        emit BrainboxChanged(brainbox, _brainbox);
        brainbox = _brainbox;
    }

    function setDailyLimit(address _token, uint256 _limit) external onlyOwner {
        dailyLimit[_token] = _limit;
        emit DailyLimitSet(_token, _limit);
    }

    receive() external payable {}

    // helper: current day (UTC seconds / 1 day)
    function _currentDay() internal view returns (uint256) {
        return block.timestamp / 1 days;
    }

    // increase spent amount for token today (reverts if exceeds limit)
    function _chargeDaily(address _token, uint256 _amount) internal {
        uint256 day = _currentDay();
        uint256 limit = dailyLimit[_token];
        if (limit == 0) return; // no limit set
        uint256 already = spentToday[_token][day];
        require(already + _amount <= limit, "AIAgent: daily limit exceeded");
        spentToday[_token][day] = already + _amount;
    }

    function execute(address to, uint256 value, bytes calldata data) external onlyBrainboxOrOwner returns (bytes memory) {
        // if the call will move ERC20 tokens from this contract to somewhere, and token is the configured token,
        // caller should be careful: enforce daily limit when moving token using helper functions below.

        (bool ok, bytes memory res) = to.call{value: value}(data);
        require(ok, "AIAgent: exec failed");
        emit Executed(msg.sender, to, value, data);
        return res;
    }

    function executeWithSignature(
        address to,
        uint256 value,
        bytes calldata data,
        uint256 nonce,
        uint256 deadline,
        bytes calldata signature
    ) external returns (bytes memory) {
        require(deadline == 0 || block.timestamp <= deadline, "AIAgent: signature expired");

        // build the prefixed message hash
        bytes32 hash = keccak256(abi.encodePacked(address(this), to, value, keccak256(data), nonce, deadline));
        bytes32 message = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", hash));

        // recover signer
        address signer = _recoverSigner(message, signature);
        require(signer == brainbox, "AIAgent: invalid brainbox signature");

        // replay protection
        require(nonce == nonces[signer], "AIAgent: bad nonce");
        nonces[signer]++;

        (bool ok, bytes memory res) = to.call{value: value}(data);
        require(ok, "AIAgent: exec failed");
        emit Executed(signer, to, value, data);
        return res;
    }

    // helper to transfer ERC20 token from this contract to a recipient (enforces daily limit)
    function transferToken(address _token, address to, uint256 amount) public onlyBrainboxOrOwner {
        _chargeDaily(_token, amount);
        require(IERC20Minimal(_token).transfer(to, amount), "AIAgent: token transfer failed");
        emit TokenTransferred(_token, to, amount);
    }

    // transfer the primary configured token (convenience) - enforces daily limit for configured token
    function transferPrimaryToken(address to, uint256 amount) external onlyBrainboxOrOwner {
        _chargeDaily(address(token), amount);
        require(token.transfer(to, amount), "AIAgent: primary transfer failed");
        emit TokenTransferred(address(token), to, amount);
    }

    // withdraw native currency (ETH/BNB) to owner or specified address
    function withdrawNative(address payable to, uint256 amount) external onlyOwner {
        require(address(this).balance >= amount, "AIAgent: insufficient native balance");
        to.transfer(amount);
        emit NativeWithdraw(to, amount);
    }

    // owner can rescue ERC20 tokens (emergency)
    function rescueERC20(address _token, address to, uint256 amount) external onlyOwner {
        require(IERC20Minimal(_token).transfer(to, amount), "AIAgent: rescue failed");
    }

    // recover signer from signature (v,r,s compact signature expected)
    function _recoverSigner(bytes32 message, bytes memory sig) internal pure returns (address) {
        // signature must be 65 bytes
        require(sig.length == 65, "AIAgent: invalid sig length");
        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := mload(add(sig, 32))
            s := mload(add(sig, 64))
            v := byte(0, mload(add(sig, 96)))
        }
        // EIP-2 still allows v = 27 or 28
        if (v < 27) {
            v += 27;
        }
        require(v == 27 || v == 28, "AIAgent: invalid v");
        return ecrecover(message, v, r, s);
    }

    // view helpers
    function primaryBalance() external view returns (uint256) {
        return token.balanceOf(address(this));
    }

    function tokenBalance(address _token) external view returns (uint256) {
        return IERC20Minimal(_token).balanceOf(address(this));
    }
}
