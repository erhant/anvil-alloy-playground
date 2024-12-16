pragma solidity ^0.8.20;

contract Ownable {
    address public owner;
    uint value = 4;

    constructor(address _owner) {
        owner = _owner;
    }

    function callMe(uint val) external returns (uint) {
        require(owner == msg.sender, "not an owner");
        value += val;
        return value;
    }
}
