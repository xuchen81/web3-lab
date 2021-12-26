// SPDX-License-Identifier: SPDX-License

// https://www.youtube.com/watch?v=9Qpi80dQsGU
pragma solidity ^0.8.0;

import 'hardhat/console.sol';

contract Token {
  string public name = 'My hardhat token';
  string public symbol = 'MHT';
  uint public totalSupply = 1000000;
  address public owner;
  mapping(address => uint) balances;

  constructor() {
    balances[msg.sender] = totalSupply;
    owner = msg.sender;
  }

  function transfer(address to, uint amount) external {
    console.log('sender balance is %s tokens', balances[msg.sender]);
    console.log('trying to sender %s tokens to %s', amount, to);
    require(balances[msg.sender] >= amount, 'Not Enough Tokens');
    balances[msg.sender] -= amount;
    balances[to] += amount;
  }

  function balanceOf(address account) external view returns(uint) {
    return balances[account];
  }
}