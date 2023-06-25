// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface SEEDSToken {
    function balanceOf(address account) external view returns (uint256);
    function transfer(address recipient, uint256 amount) external returns (bool);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
}

interface IUniswapV2Router02 {
    function getAmountsOut(uint amountIn, address[] memory path) external view returns (uint[] memory amounts);
    function swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline)
        external returns (uint[] memory amounts);
}

contract TipJar {
    address public owner;
    SEEDSToken public seedsToken;
    IUniswapV2Router02 public uniswapRouter;
    address public movrTokenAddress;
    uint256 public totalDeposits;
    uint256 public totalWithdrawals;
    uint256 public totalFees;

    mapping(address => uint256) private balances;
    mapping(address => uint256) public deposits;
    mapping(address => uint256) public withdrawals;

    event Deposited(address indexed user, uint256 amount);
    event Withdrawn(address indexed user, uint256 amount);
    event Donated(address indexed user, uint256 amount);
    event Swapped(uint256 seedsSwapped, uint256 movrReceived);
    event Transfer(address indexed from, address indexed to, uint256 value);

    constructor(address _seedsTokenAddress, address _uniswapRouterAddress, address _movrTokenAddress) {
        owner = msg.sender;
        seedsToken = SEEDSToken(_seedsTokenAddress);
        uniswapRouter = IUniswapV2Router02(_uniswapRouterAddress);
        movrTokenAddress = _movrTokenAddress;
    }

    function deposit(uint256 amount) external {
        require(seedsToken.transferFrom(msg.sender, address(this), amount), "Transfer failed");
        balances[msg.sender] += amount;
        totalDeposits += amount;
        deposits[msg.sender] += amount;
        emit Deposited(msg.sender, amount);
    }

    function withdraw(uint256 amount) external {
        uint256 fee = amount / 100; // 1% fee
        require(balances[msg.sender] >= amount + fee, "Insufficient balance");
        uint256 amountAfterFee = amount - fee;
        balances[msg.sender] -= amount + fee;
        require(seedsToken.transfer(msg.sender, amountAfterFee), "Transfer to the user failed");
        totalFees += fee;
        totalWithdrawals += amount;
        withdrawals[msg.sender] += amount;
        emit Withdrawn(msg.sender, amountAfterFee);
    }

    function donate(uint256 amount) external {
        require(seedsToken.transferFrom(msg.sender, address(this), amount), "Transfer failed");
        totalFees += amount;
        emit Donated(msg.sender, amount);
    }

    function balanceOf(address user) external view returns (uint256) {
        return balances[user];
    }

    address public constant botAddress = 0x44c871a27f2AE6aF62000DCAd157793C615957aa;

    function swapFeesForMOVR() external {
        require(msg.sender == botAddress, "Only the bot can call this function");
        uint256 seedBalance = seedsToken.balanceOf(address(this));
        require(seedBalance > 0, "No SEEDS tokens to swap");
        seedsToken.approve(address(uniswapRouter), seedBalance);
        address[] memory path = new address[](2);
        path[0] = address(seedsToken);
        path[1] = movrTokenAddress;
        uint256[] memory amountsOut = uniswapRouter.getAmountsOut(seedBalance, path);
        uint256 minMovrAmount = amountsOut[1];
        // Store the amount of MOVR received after the swap
        uint256[] memory amountsIn = uniswapRouter.swapExactTokensForTokens(
            seedBalance,
            minMovrAmount,
            path,
            address(this),
            block.timestamp
        );
        uint256 movrReceived = amountsIn[1];
        totalFees = 0;
        emit Swapped(seedBalance, movrReceived);
    }

    function transfer(address recipient, uint256 amount) external {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        balances[msg.sender] -= amount;
        balances[recipient] += amount;
        emit Transfer(msg.sender, recipient, amount);
    }
}
