// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

contract RandomTask {
    uint256 public storedNumber;
    event NumberSet(uint256 number);

    function setNumber(uint256 number) public {
        storedNumber = number;
        emit NumberSet(number);
    }

    function doubleNumber() public {
        storedNumber = storedNumber * 2;
        emit NumberSet(storedNumber);
    }
}
