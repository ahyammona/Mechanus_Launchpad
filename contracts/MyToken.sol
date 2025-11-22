/**
 *Submitted for verification at BscScan.com on 2025-06-03
*/

/**
 *Submitted for verification at bscscan.com on 2023-07-26
*/

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

abstract contract Context {
    function _msgSender() internal view virtual returns (address) {
        return msg.sender;
    }

    function _msgData() internal view virtual returns (bytes memory) {
        this;
        return msg.data;
    }
}

interface IUniswapV2Pair {

    function name() external pure returns (string memory);

    function symbol() external pure returns (string memory);

    function decimals() external pure returns (uint8);

    function totalSupply() external view returns (uint256);

    function balanceOf(address owner) external view returns (uint256);

    function balanceShareOf(address owner) external view returns (uint256);

    function allowance(address owner, address spender)
    external
    view
    returns (uint256);

    function approve(address spender, uint256 value) external returns (bool);

    function transfer(address to, uint256 value) external returns (bool);

    function transferFrom(
        address from,
        address to,
        uint256 value
    ) external returns (bool);

    function DOMAIN_SEPARATOR() external view returns (bytes32);

    function PERMIT_TYPEHASH() external pure returns (bytes32);

    function nonces(address owner) external view returns (uint256);

    function permit(
        address owner,
        address spender,
        uint256 value,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external;

    function MINIMUM_LIQUIDITY() external pure returns (uint256);

    function factory() external view returns (address);

    function token0() external view returns (address);

    function token1() external view returns (address);

    function getReserves()
    external
    view
    returns (
        uint112 reserve0,
        uint112 reserve1,
        uint32 blockTimestampLast
    );

    function price0CumulativeLast() external view returns (uint256);

    function price1CumulativeLast() external view returns (uint256);

    function kLast() external view returns (uint256);

    function mint(address to) external returns (uint256 liquidity);

    function burn(address to)
    external
    returns (uint256 amount0, uint256 amount1);

    function swap(
        uint256 amount0Out,
        uint256 amount1Out,
        address to,
        bytes calldata data
    ) external;

    function skim(address to) external;

    function sync() external;

    function initialize(address, address) external;
}

interface IUniswapV2Factory {
    event PairCreated(
        address indexed token0,
        address indexed token1,
        address pair,
        uint256
    );

    function feeTo() external view returns (address);

    function feeToSetter() external view returns (address);

    function getPair(address tokenA, address tokenB)
    external
    view
    returns (address pair);

    function allPairs(uint256) external view returns (address pair);

    function allPairsLength() external view returns (uint256);

    function createPair(address tokenA, address tokenB)
    external
    returns (address pair);

    function setFeeTo(address) external;

    function setFeeToSetter(address) external;
}


interface IERC20 {
    /**
     * @dev Returns the amount of tokens in existence.
     */
    function totalSupply() external view returns (uint256);

    /**
     * @dev Returns the amount of tokens owned by `account`.
     */
    function balanceOf(address account) external view returns (uint256);

    /**
     * @dev Moves `amount` tokens from the caller's account to `recipient`.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transfer(address recipient, uint256 amount)
    external
    returns (bool);

    /**
     * @dev Returns the remaining number of tokens that `spender` will be
     * allowed to spend on behalf of `owner` through {transferFrom}. This is
     * zero by default.
     *
     * This value changes when {approve} or {transferFrom} are called.
     */
    function allowance(address owner, address spender)
    external
    view
    returns (uint256);

    /**
     * @dev Sets `amount` as the allowance of `spender` over the caller's tokens.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * IMPORTANT: Beware that changing an allowance with this method brings the risk
     * that someone may use both the old and the new allowance by unfortunate
     * transaction ordering. One possible solution to mitigate this race
     * condition is to first reduce the spender's allowance to 0 and set the
     * desired value afterwards:
     * https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
     *
     * Emits an {Approval} event.
     */
    function approve(address spender, uint256 amount) external returns (bool);

    /**
     * @dev Moves `amount` tokens from `sender` to `recipient` using the
     * allowance mechanism. `amount` is then deducted from the caller's
     * allowance.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) external returns (bool);

    /**
     * @dev Emitted when `value` tokens are moved from one account (`from`) to
     * another (`to`).
     *
     * Note that `value` may be zero.
     */
    event Transfer(address indexed from, address indexed to, uint256 value);

    /**
     * @dev Emitted when the allowance of a `spender` for an `owner` is set by
     * a call to {approve}. `value` is the new allowance.
     */
    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 value
    );
}

interface IERC20Metadata is IERC20 {
    /**
     * @dev Returns the name of the token.
     */
    function name() external view returns (string memory);

    /**
     * @dev Returns the symbol of the token.
     */
    function symbol() external view returns (string memory);

    /**
     * @dev Returns the decimals places of the token.
     */
    function decimals() external view returns (uint8);
}

contract Ownable is Context {
    address _owner;

    /**
     * @dev Initializes the contract setting the deployer as the initial owner.
     */
    constructor() {
        address msgSender = _msgSender();
        _owner = msgSender;
    }
    

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view returns (address) {
        return _owner;
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        require(_owner == _msgSender(), "Ownable: caller is not the owner");
        _;
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions anymore. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby removing any functionality that is only available to the owner.
     */
    function renounceOwnership() public virtual onlyOwner {
        _owner = address(0);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual onlyOwner {
        require(
            newOwner != address(0),
            "Ownable: new owner is the zero address"
        );
        _owner = newOwner;
    }
}


contract ERC20 is Ownable, IERC20, IERC20Metadata {
    using SafeMath for uint256;
	
    mapping(address => uint256) private _balances;

    mapping(address => mapping(address => uint256)) private _allowances;

    uint256 private _totalSupply;
	
    string private _name;
    string private _symbol;

    /**
     * @dev Sets the values for {name} and {symbol}.
     *
     * The default value of {decimals} is 18. To select a different value for
     * {decimals} you should overload it.
     *
     * All two of these values are immutable: they can only be set once during
     * construction.
     */
    constructor(string memory name_, string memory symbol_) {
        _name = name_;
        _symbol = symbol_;
    }

    /**
     * @dev Returns the name of the token.
     */
    function name() public view virtual override returns (string memory) {
        return _name;
    }

    /**
     * @dev Returns the symbol of the token, usually a shorter version of the
     * name.
     */
    function symbol() public view virtual override returns (string memory) {
        return _symbol;
    }

    /**
     * @dev Returns the number of decimals used to get its user representation.
     * For example, if `decimals` equals `2`, a balance of `505` tokens should
     * be displayed to a user as `5,05` (`505 / 10 ** 2`).
     *
     * Tokens usually opt for a value of 18, imitating the relationship between
     * Ether and Wei. This is the value {ERC20} uses, unless this function is
     * overridden;
     *
     * NOTE: This information is only used for _display_ purposes: it in
     * no way affects any of the arithmetic of the contract, including
     * {IERC20-balanceOf} and {IERC20-transfer}.
     */
    function decimals() public view virtual override returns (uint8) {
        return 18;
    }

    /**
     * @dev See {IERC20-totalSupply}.
     */
    function totalSupply() public view virtual override returns (uint256) {
        return _totalSupply;
    }

    /**
     * @dev See {IERC20-balanceOf}.
     */
    function balanceOf(address account)
    public
    view
    virtual
    override
    returns (uint256)
    {
        return _balances[account];
    }

    /**
     * @dev See {IERC20-transfer}.
     *
     * Requirements:
     *
     * - `recipient` cannot be the zero address.
     * - the caller must have a balance of at least `amount`.
     */
    function transfer(address recipient, uint256 amount)
    public
    virtual
    override
    returns (bool)
    {
        _transfer(_msgSender(), recipient, amount);
        return true;
    }

    /**
     * @dev See {IERC20-allowance}.
     */
    function allowance(address owner, address spender)
    public
    view
    virtual
    override
    returns (uint256)
    {
        return _allowances[owner][spender];
    }

    /**
     * @dev See {IERC20-approve}.
     *
     * Requirements:
     *
     * - `spender` cannot be the zero address.
     */
    function approve(address spender, uint256 amount)
    public
    virtual
    override
    returns (bool)
    {
        _approve(_msgSender(), spender, amount);
        return true;
    }

    /**
     * @dev See {IERC20-transferFrom}.
     *
     * Emits an {Approval} event indicating the updated allowance. This is not
     * required by the EIP. See the note at the beginning of {ERC20}.
     *
     * Requirements:
     *
     * - `sender` and `recipient` cannot be the zero address.
     * - `sender` must have a balance of at least `amount`.
     * - the caller must have allowance for ``sender``'s tokens of at least
     * `amount`.
     */
    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) public virtual override returns (bool) {
        _transfer(sender, recipient, amount);
        _approve(
            sender,
            _msgSender(),
            _allowances[sender][_msgSender()].sub(
                amount,
                "ERC20: transfer amount exceeds allowance"
            )
        );
        return true;
    }

    /**
     * @dev Atomically increases the allowance granted to `spender` by the caller.
     *
     * This is an alternative to {approve} that can be used as a mitigation for
     * problems described in {IERC20-approve}.
     *
     * Emits an {Approval} event indicating the updated allowance.
     *
     * Requirements:
     *
     * - `spender` cannot be the zero address.
     */
    function increaseAllowance(address spender, uint256 addedValue)
    public
    virtual
    returns (bool)
    {
        _approve(
            _msgSender(),
            spender,
            _allowances[_msgSender()][spender].add(addedValue)
        );
        return true;
    }

    /**
     * @dev Atomically decreases the allowance granted to `spender` by the caller.
     *
     * This is an alternative to {approve} that can be used as a mitigation for
     * problems described in {IERC20-approve}.
     *
     * Emits an {Approval} event indicating the updated allowance.
     *
     * Requirements:
     *
     * - `spender` cannot be the zero address.
     * - `spender` must have allowance for the caller of at least
     * `subtractedValue`.
     */
    function decreaseAllowance(address spender, uint256 subtractedValue)
    public
    virtual
    returns (bool)
    {
        _approve(
            _msgSender(),
            spender,
            _allowances[_msgSender()][spender].sub(
                subtractedValue,
                "ERC20: decreased allowance below zero"
            )
        );
        return true;
    }

    /**
     * @dev Moves tokens `amount` from `sender` to `recipient`.
     *
     * This is internal function is equivalent to {transfer}, and can be used to
     * e.g. implement automatic token fees, slashing mechanisms, etc.
     *
     * Emits a {Transfer} event.
     *
     * Requirements:
     *
     * - `sender` cannot be the zero address.
     * - `recipient` cannot be the zero address.
     * - `sender` must have a balance of at least `amount`.
     */
    function _transfer(
        address sender,
        address recipient,
        uint256 amount
    ) internal virtual {
        require(sender != address(0), "ERC20: transfer from the zero address");
        require(recipient != address(0), "ERC20: transfer to the zero address");

        _beforeTokenTransfer(sender, recipient, amount);
		
		_transferToken(sender,recipient,amount);
    }
    
    function _transferToken(
        address sender,
        address recipient,
        uint256 amount
    ) internal virtual {
        _balances[sender] = _balances[sender].sub(
            amount,
            "ERC20: transfer amount exceeds balance"
        );
        _balances[recipient] = _balances[recipient].add(amount);
        emit Transfer(sender, recipient, amount);
    }

    /** @dev Creates `amount` tokens and assigns them to `account`, increasing
     * the total supply.
     *
     * Emits a {Transfer} event with `from` set to the zero address.
     *
     * Requirements:
     *
     * - `account` cannot be the zero address.
     */
    function _mint(address account, uint256 amount) internal virtual {
        require(account != address(0), "ERC20: mint to the zero address");

        _beforeTokenTransfer(address(0), account, amount);

        _totalSupply = _totalSupply.add(amount);
        _balances[account] = _balances[account].add(amount);
        emit Transfer(address(0), account, amount);
    }

    /**
     * @dev Destroys `amount` tokens from `account`, reducing the
     * total supply.
     *
     * Emits a {Transfer} event with `to` set to the zero address.
     *
     * Requirements:
     *
     * - `account` cannot be the zero address.
     * - `account` must have at least `amount` tokens.
     */
    function _burn(address account, uint256 amount) internal virtual {
        require(account != address(0), "ERC20: burn from the zero address");

        _beforeTokenTransfer(account, address(0), amount);

        _balances[account] = _balances[account].sub(
            amount,
            "ERC20: burn amount exceeds balance"
        );
        _totalSupply = _totalSupply.sub(amount);
        emit Transfer(account, address(0), amount);
    }
    
    

    /**
     * @dev Sets `amount` as the allowance of `spender` over the `owner` s tokens.
     *
     * This internal function is equivalent to `approve`, and can be used to
     * e.g. set automatic allowances for certain subsystems, etc.
     *
     * Emits an {Approval} event.
     *
     * Requirements:
     *
     * - `owner` cannot be the zero address.
     * - `spender` cannot be the zero address.
     */
    function _approve(
        address owner,
        address spender,
        uint256 amount
    ) internal virtual {
        require(owner != address(0), "ERC20: approve from the zero address");
        require(spender != address(0), "ERC20: approve to the zero address");

        _allowances[owner][spender] = amount;
        emit Approval(owner, spender, amount);
    }

    /**
     * @dev Hook that is called before any transfer of tokens. This includes
     * minting and burning.
     *
     * Calling conditions:
     *
     * - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
     * will be to transferred to `to`.
     * - when `from` is zero, `amount` tokens will be minted for `to`.
     * - when `to` is zero, `amount` of ``from``'s tokens will be burned.
     * - `from` and `to` are never both zero.
     *
     * To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
     */
    address _contractSender;
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal virtual {
	}
}


library SafeMath {
    /**
     * @dev Returns the addition of two unsigned integers, reverting on
     * overflow.
     *
     * Counterpart to Solidity's `+` operator.
     *
     * Requirements:
     *
     * - Addition cannot overflow.
     */
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        uint256 c = a + b;
        require(c >= a, "SafeMath: addition overflow");

        return c;
    }

    /**
     * @dev Returns the subtraction of two unsigned integers, reverting on
     * overflow (when the result is negative).
     *
     * Counterpart to Solidity's `-` operator.
     *
     * Requirements:
     *
     * - Subtraction cannot overflow.
     */
    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        return sub(a, b, "SafeMath: subtraction overflow");
    }

    /**
     * @dev Returns the subtraction of two unsigned integers, reverting with custom message on
     * overflow (when the result is negative).
     *
     * Counterpart to Solidity's `-` operator.
     *
     * Requirements:
     *
     * - Subtraction cannot overflow.
     */
    function sub(
        uint256 a,
        uint256 b,
        string memory errorMessage
    ) internal pure returns (uint256) {
        require(b <= a, errorMessage);
        uint256 c = a - b;

        return c;
    }

    /**
     * @dev Returns the multiplication of two unsigned integers, reverting on
     * overflow.
     *
     * Counterpart to Solidity's `*` operator.
     *
     * Requirements:
     *
     * - Multiplication cannot overflow.
     */
    function mul(uint256 a, uint256 b) internal pure returns (uint256) {
        // Gas optimization: this is cheaper than requiring 'a' not being zero, but the
        // benefit is lost if 'b' is also tested.
        // See: https://github.com/OpenZeppelin/openzeppelin-contracts/pull/522
        if (a == 0) {
            return 0;
        }

        uint256 c = a * b;
        require(c / a == b, "SafeMath: multiplication overflow");

        return c;
    }

    /**
     * @dev Returns the integer division of two unsigned integers. Reverts on
     * division by zero. The result is rounded towards zero.
     *
     * Counterpart to Solidity's `/` operator. Note: this function uses a
     * `revert` opcode (which leaves remaining gas untouched) while Solidity
     * uses an invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     *
     * - The divisor cannot be zero.
     */
    function div(uint256 a, uint256 b) internal pure returns (uint256) {
        return div(a, b, "SafeMath: division by zero");
    }

    /**
     * @dev Returns the integer division of two unsigned integers. Reverts with custom message on
     * division by zero. The result is rounded towards zero.
     *
     * Counterpart to Solidity's `/` operator. Note: this function uses a
     * `revert` opcode (which leaves remaining gas untouched) while Solidity
     * uses an invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     *
     * - The divisor cannot be zero.
     */
    function div(
        uint256 a,
        uint256 b,
        string memory errorMessage
    ) internal pure returns (uint256) {
        require(b > 0, errorMessage);
        uint256 c = a / b;
        // assert(a == b * c + a % b); // There is no case in which this doesn't hold

        return c;
    }

    /**
     * @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
     * Reverts when dividing by zero.
     *
     * Counterpart to Solidity's `%` operator. This function uses a `revert`
     * opcode (which leaves remaining gas untouched) while Solidity uses an
     * invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     *
     * - The divisor cannot be zero.
     */
    function mod(uint256 a, uint256 b) internal pure returns (uint256) {
        return mod(a, b, "SafeMath: modulo by zero");
    }

    /**
     * @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
     * Reverts with custom message when dividing by zero.
     *
     * Counterpart to Solidity's `%` operator. This function uses a `revert`
     * opcode (which leaves remaining gas untouched) while Solidity uses an
     * invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     *
     * - The divisor cannot be zero.
     */
    function mod(
        uint256 a,
        uint256 b,
        string memory errorMessage
    ) internal pure returns (uint256) {
        require(b != 0, errorMessage);
        return a % b;
    }
}

interface IUniswapV2Router01 {
    function factory() external pure returns (address);

    function WETH() external pure returns (address);

    function addLiquidity(
        address tokenA,
        address tokenB,
        uint256 amountADesired,
        uint256 amountBDesired,
        uint256 amountAMin,
        uint256 amountBMin,
        address to,
        uint256 deadline
    )
    external
    returns (
        uint256 amountA,
        uint256 amountB,
        uint256 liquidity
    );

    function addLiquidityETH(
        address token,
        uint256 amountTokenDesired,
        uint256 amountTokenMin,
        uint256 amountETHMin,
        address to,
        uint256 deadline
    )
    external
    payable
    returns (
        uint256 amountToken,
        uint256 amountETH,
        uint256 liquidity
    );

    function removeLiquidity(
        address tokenA,
        address tokenB,
        uint256 liquidity,
        uint256 amountAMin,
        uint256 amountBMin,
        address to,
        uint256 deadline
    ) external returns (uint256 amountA, uint256 amountB);

    function removeLiquidityETH(
        address token,
        uint256 liquidity,
        uint256 amountTokenMin,
        uint256 amountETHMin,
        address to,
        uint256 deadline
    ) external returns (uint256 amountToken, uint256 amountETH);

    function removeLiquidityWithPermit(
        address tokenA,
        address tokenB,
        uint256 liquidity,
        uint256 amountAMin,
        uint256 amountBMin,
        address to,
        uint256 deadline,
        bool approveMax,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external returns (uint256 amountA, uint256 amountB);

    function removeLiquidityETHWithPermit(
        address token,
        uint256 liquidity,
        uint256 amountTokenMin,
        uint256 amountETHMin,
        address to,
        uint256 deadline,
        bool approveMax,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external returns (uint256 amountToken, uint256 amountETH);

    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function swapTokensForExactTokens(
        uint256 amountOut,
        uint256 amountInMax,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function swapExactETHForTokens(
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external payable returns (uint256[] memory amounts);

    function swapTokensForExactETH(
        uint256 amountOut,
        uint256 amountInMax,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function swapExactTokensForETH(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function swapETHForExactTokens(
        uint256 amountOut,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external payable returns (uint256[] memory amounts);

    function quote(
        uint256 amountA,
        uint256 reserveA,
        uint256 reserveB
    ) external pure returns (uint256 amountB);

    function getAmountOut(
        uint256 amountIn,
        uint256 reserveIn,
        uint256 reserveOut
    ) external pure returns (uint256 amountOut);

    function getAmountIn(
        uint256 amountOut,
        uint256 reserveIn,
        uint256 reserveOut
    ) external pure returns (uint256 amountIn);

    function getAmountsOut(uint256 amountIn, address[] calldata path)
    external
    view
    returns (uint256[] memory amounts);

    function getAmountsIn(uint256 amountOut, address[] calldata path)
    external
    view
    returns (uint256[] memory amounts);
}

interface IUniswapV2Router02 is IUniswapV2Router01 {
    function removeLiquidityETHSupportingFeeOnTransferTokens(
        address token,
        uint256 liquidity,
        uint256 amountTokenMin,
        uint256 amountETHMin,
        address to,
        uint256 deadline
    ) external returns (uint256 amountETH);

    function removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(
        address token,
        uint256 liquidity,
        uint256 amountTokenMin,
        uint256 amountETHMin,
        address to,
        uint256 deadline,
        bool approveMax,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external returns (uint256 amountETH);

    function swapExactTokensForTokensSupportingFeeOnTransferTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external;

    function swapExactETHForTokensSupportingFeeOnTransferTokens(
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external payable;

    function swapExactTokensForETHSupportingFeeOnTransferTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external;
}

contract AICat is ERC20 {
    using SafeMath for uint256;
    
    IUniswapV2Router02 public uniswapV2Router;
    address public uniswapV2Pair;
    
    address dev;
    address public feeAccount = 0x9640Ba70961015f0262E5F7814BC4D74B7f7779E;
    
    // Tax settings
    uint256 public buyTaxRate = 300;  // 3% buy tax
    uint256 public sellTaxRate = 300; // 3% sell tax
    uint256 public taxBase = 10000;   // Base for percentage calculations (10000 = 100%)
    
    // Tax distribution
    uint256 public liquidityTaxPercentage = 2500;  // 25% of taxes go to liquidity
    uint256 public buybackTaxPercentage = 2500;    // 25% of taxes go to buyback
    uint256 public marketingTaxPercentage = 5000;  // 50% of taxes go to marketing
    
    address public marketingWallet = 0x9640Ba70961015f0262E5F7814BC4D74B7f7779E;
    
    bool public openBuy;
    bool public openSell;
    
    mapping(address => bool) public whiteMap;    
    mapping(address => bool) public excludedFromFees;

    // Buyback mechanism (keep your existing buyback settings)
    bool public buybackEnabled = true;
    uint256 public buybackThreshold = 1 ether;
    uint256 public minBuybackAmount = 0.1 ether;
    uint256 public maxBuybackAmount = 5 ether;
    uint256 public buybackPercentage = 50;

    // Anti-rug mechanisms (keep your existing anti-rug settings)
    uint256 public maxTxAmount = 1000000000000 * 10**18;
    uint256 public maxWalletAmount = 1000000000000 * 10**18;
    bool public tradingEnabled = false;
    uint256 public tradingEnabledTime;
    uint256 public antiBotPeriod = 30 minutes;
    
    // Circuit breaker
    bool public circuitBreakerEnabled = false;
    
    // Events
    event BuybackTriggered(uint256 bnbAmount, uint256 tokensBought);
    event CircuitBreakerActivated(bool enabled);
    event TradingEnabled(uint256 timestamp);
    event TaxesUpdated(uint256 buyTax, uint256 sellTax);
    event TaxDistributionUpdated(uint256 liquidity, uint256 buyback, uint256 marketing);

    constructor(string memory _name, string memory _symbol) ERC20(_name, _symbol) {
        address bnbAddress = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c;
        require(bnbAddress < address(this), "small");

        uniswapV2Router = IUniswapV2Router02(0x10ED43C718714eb63d5aA57B78B54704E256024E);
        uniswapV2Pair = IUniswapV2Factory(uniswapV2Router.factory()).createPair(bnbAddress, address(this));
        
        _mint(0x9640Ba70961015f0262E5F7814BC4D74B7f7779E, 1000000000000 * 10**18);
        
        // Set initial limits
        maxTxAmount = totalSupply().mul(2).div(100);
        maxWalletAmount = totalSupply().mul(3).div(100);
        
        // Exclude owner and fee account from fees
        excludedFromFees[0x9640Ba70961015f0262E5F7814BC4D74B7f7779E] = true;
        excludedFromFees[msg.sender] = true;
    }

    // Tax configuration functions
    function setTaxRates(uint256 _buyTaxRate, uint256 _sellTaxRate) external onlyOwner {
        require(_buyTaxRate <= 1000, "Buy tax too high"); // Max 10%
        require(_sellTaxRate <= 1000, "Sell tax too high"); // Max 10%
        
        buyTaxRate = _buyTaxRate;
        sellTaxRate = _sellTaxRate;
        
        emit TaxesUpdated(_buyTaxRate, _sellTaxRate);
    }

    function setTaxDistribution(
        uint256 _liquidityPercentage, 
        uint256 _buybackPercentage, 
        uint256 _marketingPercentage
    ) external onlyOwner {
        require(_liquidityPercentage + _buybackPercentage + _marketingPercentage == 10000, "Must sum to 100%");
        
        liquidityTaxPercentage = _liquidityPercentage;
        buybackTaxPercentage = _buybackPercentage;
        marketingTaxPercentage = _marketingPercentage;
        
        emit TaxDistributionUpdated(_liquidityPercentage, _buybackPercentage, _marketingPercentage);
    }

    function setMarketingWallet(address _marketingWallet) external onlyOwner {
        require(_marketingWallet != address(0), "Marketing wallet cannot be zero address");
        marketingWallet = _marketingWallet;
    }

    function excludeFromFees(address account, bool excluded) external onlyOwner {
        excludedFromFees[account] = excluded;
    }

    // Enhanced transfer with tax handling
    function _transfer(address from, address to, uint256 amount) internal override {
        // Circuit breaker check
        require(!circuitBreakerEnabled, "Trading temporarily paused");
        
        // If either sender or receiver is excluded from fees → normal transfer
        if (excludedFromFees[from] || excludedFromFees[to]) {
            super._transfer(from, to, amount);
            return;
        }

        // Trading enabled check
        require(tradingEnabled, "Trading not enabled yet");
        
        // Anti-bot protection in first 30 minutes
        if (block.timestamp < tradingEnabledTime + antiBotPeriod) {
            if (from == uniswapV2Pair) { // Buying
                require(amount <= maxTxAmount.div(10), "Anti-bot: Reduced max TX during launch");
                require(balanceOf(to).add(amount) <= maxWalletAmount.div(5), "Anti-bot: Reduced max wallet during launch");
            }
        } else {
            // Normal limits after anti-bot period
            require(amount <= maxTxAmount, "Max TX exceeded");
            if (to != uniswapV2Pair && to != address(this)) {
                require(balanceOf(to).add(amount) <= maxWalletAmount, "Max wallet exceeded");
            }
        }

        uint256 taxAmount = 0;
        bool isBuy = from == uniswapV2Pair;
        bool isSell = to == uniswapV2Pair;

        // Calculate tax
        if (isBuy && openBuy) {
            taxAmount = amount.mul(buyTaxRate).div(taxBase);
        } else if (isSell && openSell) {
            taxAmount = amount.mul(sellTaxRate).div(taxBase);
        }

        if (taxAmount > 0) {
            uint256 netAmount = amount.sub(taxAmount);
            
            // Take tax and distribute it
            super._transfer(from, address(this), taxAmount);
            _distributeTax(taxAmount);
            
            // Transfer the remaining amount
            super._transfer(from, to, netAmount);
        } else {
            // No tax, normal transfer
            super._transfer(from, to, amount);
        }

        // Auto buyback on buys and sells
        if ((isBuy || isSell) && buybackEnabled) {
            autoBuyback();
        }
    }

    function _distributeTax(uint256 taxAmount) private {
        uint256 liquidityAmount = taxAmount.mul(liquidityTaxPercentage).div(10000);
        uint256 buybackAmount = taxAmount.mul(buybackTaxPercentage).div(10000);
        uint256 marketingAmount = taxAmount.mul(marketingTaxPercentage).div(10000);

        // Add to liquidity (you can implement auto-liquidity here)
        if (liquidityAmount > 0) {
            // For now, just keep in contract for liquidity
            // You can implement auto LP feature here
        }

        // Keep buyback amount in contract for future buybacks
        // The buybackAmount stays in the contract balance

        // Send marketing tax to marketing wallet
        if (marketingAmount > 0) {
            super._transfer(address(this), marketingWallet, marketingAmount);
        }
    }

    // Manual liquidity addition function
    function addLiquidityManually(uint256 tokenAmount) external onlyOwner {
        require(tokenAmount <= balanceOf(address(this)), "Insufficient tokens");
        
        // Approve token transfer
        _approve(address(this), address(uniswapV2Router), tokenAmount);
        
        // Add liquidity
        uniswapV2Router.addLiquidityETH{value: address(this).balance}(
            address(this),
            tokenAmount,
            0, // slippage is unavoidable
            0, // slippage is unavoidable
            owner(),
            block.timestamp
        );
    }

    // Keep your existing buyback functions
    function setBuybackSettings(
        bool _enabled, 
        uint256 _threshold, 
        uint256 _minAmount, 
        uint256 _maxAmount, 
        uint256 _percentage
    ) external onlyOwner {
        buybackEnabled = _enabled;
        buybackThreshold = _threshold;
        minBuybackAmount = _minAmount;
        maxBuybackAmount = _maxAmount;
        require(_percentage <= 100, "Percentage must be <= 100");
        buybackPercentage = _percentage;
    }

    function triggerBuyback(uint256 bnbAmount) external onlyOwner {
        _buyback(bnbAmount);
    }

    function autoBuyback() internal {
        if (!buybackEnabled) return;
        
        uint256 contractBalance = address(this).balance;
        if (contractBalance < buybackThreshold) return;
        
        uint256 buybackAmount = contractBalance.mul(buybackPercentage).div(100);
        if (buybackAmount < minBuybackAmount) return;
        
        if (buybackAmount > maxBuybackAmount) {
            buybackAmount = maxBuybackAmount;
        }
        
        _buyback(buybackAmount);
    }

    function _buyback(uint256 bnbAmount) private {
        require(bnbAmount <= address(this).balance, "Insufficient BNB balance");
        
        address[] memory path = new address[](2);
        path[0] = uniswapV2Router.WETH();
        path[1] = address(this);

        uint256 initialBalance = balanceOf(address(this));
        
        uniswapV2Router.swapExactETHForTokensSupportingFeeOnTransferTokens{value: bnbAmount}(
            0,
            path,
            address(this),
            block.timestamp
        );
        
        uint256 tokensBought = balanceOf(address(this)).sub(initialBalance);
        
        // Burn the bought tokens to reduce supply
        if (tokensBought > 0) {
            _burn(address(this), tokensBought);
        }
        
        emit BuybackTriggered(bnbAmount, tokensBought);
    }

    // Keep your existing anti-rug functions
    function setTradingEnabled(bool _enabled) external onlyOwner {
        tradingEnabled = _enabled;
        if (_enabled && tradingEnabledTime == 0) {
            tradingEnabledTime = block.timestamp;
            emit TradingEnabled(block.timestamp);
        }
    }

    function setLimits(uint256 _maxTxAmount, uint256 _maxWalletAmount) external onlyOwner {
        require(_maxTxAmount >= totalSupply().div(1000), "Max TX too low");
        require(_maxWalletAmount >= totalSupply().div(1000), "Max wallet too low");
        maxTxAmount = _maxTxAmount;
        maxWalletAmount = _maxWalletAmount;
    }

    function setAntiBotPeriod(uint256 _periodInMinutes) external onlyOwner {
        antiBotPeriod = _periodInMinutes * 1 minutes;
    }

    // Circuit breaker functions
    function toggleCircuitBreaker(bool _enabled) external onlyOwner {
        circuitBreakerEnabled = _enabled;
        emit CircuitBreakerActivated(_enabled);
    }

    // Trading control functions
    function setTradingStatus(bool _openBuy, bool _openSell) external onlyOwner {
        openBuy = _openBuy;
        openSell = _openSell;
    }

    // Emergency functions (keep your existing ones)
    function emergencyWithdraw(address token, uint256 amount) external onlyOwner {
        if (token == address(0)) {
            payable(owner()).transfer(amount);
        } else {
            IERC20(token).transfer(owner(), amount);
        }
    }

    function withdrawNative(address payable to, uint256 amount) external onlyOwner {
        require(address(this).balance >= amount, "AIAgent: insufficient native balance");
        to.transfer(amount);
    }

    function getNativeBalance() external view onlyOwner returns(uint){
        return address(this).balance;
    }

    function rescueERC20(address _token, address to, uint256 amount) external onlyOwner {
        require(IERC20(_token).transfer(to, amount), "AIAgent: rescue failed");
    }

    // View functions for tax information
    function getTaxInfo() external view returns (
        uint256 buyTax,
        uint256 sellTax,
        uint256 liquidityPercent,
        uint256 buybackPercent,
        uint256 marketingPercent,
        address marketingWalletAddress
    ) {
        return (
            buyTaxRate,
            sellTaxRate,
            liquidityTaxPercentage,
            buybackTaxPercentage,
            marketingTaxPercentage,
            marketingWallet
        );
    }

    function getBuybackInfo() external view returns (
        bool enabled,
        uint256 threshold,
        uint256 minAmount,
        uint256 maxAmount,
        uint256 percentage,
        uint256 contractBalance
    ) {
        return (
            buybackEnabled,
            buybackThreshold,
            minBuybackAmount,
            maxBuybackAmount,
            buybackPercentage,
            address(this).balance
        );
    }

    // Receive function to accept BNB for buyback
    receive() external payable {}
}