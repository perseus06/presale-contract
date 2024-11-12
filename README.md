# Get Start

Solana-Based Token Presale and Staking Smart Contract

## Overview

This project involves the development of a Presale Smart Contract on the Solana blockchain using Rust. 
The smart contract facilitates a structured token presale with integrated staking mechanisms, allowing users to participate in both private and public sale phases. 
During the private sale phase, users can purchase tokens with SOL, and their acquired tokens will automatically be staked in a time-locked staking pool, yielding rewards based on different lock-up periods. 
In the public sale phase, users can directly purchase tokens without the staking requirement.

## Key Features

### Token Minting and Contract Initialization

- The project will include a minting process for the token, which is managed by the smart contract.
- The minted tokens will be deposited into a contract pool to be distributed during the presale phases.

### Private Sale Phase with Integrated Staking:

#### Private Sale Duration: 

The initial phase of the presale, where users can buy tokens using SOL.

#### Automatic Staking: 

Purchased tokens during the private sale are automatically staked in a smart contract-controlled staking pool.

#### Staking Periods:

Users can choose from the following lock-up periods:
- 3 months with a 5% reward.
- 6 months with a 10% reward.
- 9 months with a 15% reward.
- 12 months with a 25% reward.

#### Reward Calculation:

- The rewards are distributed based on the staking duration, providing higher rewards for longer lock-up periods.

#### Vesting and Unlocking:
- Tokens staked during the private sale will follow a vesting schedule, where users can only claim their tokens and rewards after the lock-up period ends.

### Public Sale Phase:

- The public sale phase will allow users to buy tokens directly without automatic staking.
- The purchased tokens are transferred directly to the buyer's wallet, giving immediate ownership without any lock-up period.

### Staking Pool and Rewards Management:

- The contract includes a staking pool that handles deposits and tracks each user's staked amount, duration, and rewards.
- The staking pool contract will ensure proper calculation of rewards and release of tokens upon the completion of the staking period.

### User Interaction and Security:

#### Trustless and Secure:

- Users interact with the smart contract directly, ensuring a trustless environment where all transactions and rewards are managed transparently.
  
#### Access Control: 
- The smart contract will include admin functions for the owner, such as starting the sale phases and configuring staking rewards, while user functions will handle purchasing and claiming.
  
#### Anti-Bot Measures: 

- Implement checks to prevent malicious bots from manipulating the presale process
