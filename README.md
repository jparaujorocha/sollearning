# SolLearning - Educational Token on Solana

SolLearning is a Solana-based token (SPL) created to incentivize educational achievements. This program allows educational institutions to reward students with tokens when they complete courses or reach academic milestones, creating a blockchain-based verification system for educational accomplishments.

## Project Overview

The SolLearning token (SLEARNING) functions as a reward mechanism for a decentralized educational platform on Solana. Students earn tokens for completing courses, which can then be transferred or used within the educational ecosystem.

### Core Features

- SPL token creation with controlled minting
- Role-based access control for administrators and educators
- Student registration and tracking
- Token rewards for educational achievements
- Secure token transfers and burn mechanisms

## Technical Specifications

- **Token Name**: SolLearning
- **Token Symbol**: SLEARNING
- **Decimals**: 9 (Solana standard)
- **Initial Supply**: 100,000,000 SLEARNING
- **Technology**: Solana, Anchor Framework, Rust

## Program Architecture

The program is built using the Anchor framework and includes several key components:

### State Accounts

- **ProgramState**: Core program data including token mint address, authority, and metrics
- **EducatorAccount**: Information about authorized educators who can mint tokens
- **StudentInfo**: Student records tracking achievements and token earnings

### Instructions

1. **Initialize**: Creates the token and program state
2. **RegisterEducator**: Admin registers educators with minting permissions
3. **RegisterStudent**: Registers a student in the system
4. **CreateStudentTokenAccount**: Creates a token account for a student
5. **MintToStudent**: Educators mint tokens to students for completing courses
6. **Transfer**: Allows token transfers between accounts
7. **Burn**: Burns tokens that are no longer needed

### Security Features

- Authority controls for administrative actions
- Validation checks on all accounts and signers
- Balance checks to prevent overflow/underflow
- Limits on token minting amounts
- Program-derived addresses (PDAs) for secure account management

## Development Requirements

- Rust 1.85.0 or later
- Solana 2.1.x or later
- Node.js 23.x or later
- Anchor 0.30.x or later

## Installation and Setup

1. Install Solana CLI tools
    ```bash
    sh -c "$(curl -sSfL https://release.solana.com/v2.1.14/install)"
    ```

2. Install Anchor
    ```bash
    cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
    ```

3. Clone the repository
    ```bash
    git clone https://github.com/yourusername/sollearning.git
    cd sollearning
    ```

4. Build the program
    ```bash
    anchor build
    ```

5. Deploy to a test network
    ```bash
    anchor deploy
    ```

## Usage

### Initialize the Token

```bash
solana program call <PROGRAM_ID> initialize \
    --keypair <PATH_TO_ADMIN_KEYPAIR>
```

### Register an Educator

```bash
solana program call <PROGRAM_ID> register_educator \
    --keypair <PATH_TO_ADMIN_KEYPAIR> \
    --account educator:<EDUCATOR_ADDRESS> \
    --uint64 mint_limit:1000000000000
```

### Register a Student

```bash
solana program call <PROGRAM_ID> register_student \
    --keypair <PATH_TO_PAYER_KEYPAIR> \
    --account student:<STUDENT_ADDRESS>
```

### Mint Tokens to a Student

```bash
solana program call <PROGRAM_ID> mint_to_student \
    --keypair <PATH_TO_EDUCATOR_KEYPAIR> \
    --account student:<STUDENT_ADDRESS> \
    --uint64 amount:1000000000 \
    --string course_id:"COURSE-123"
```

## Project Structure

```
sollearning/
├── Anchor.toml                # Anchor configuration
├── Cargo.toml                 # Rust workspace configuration
├── programs/
│   └── sollearning/
│       ├── Cargo.toml         # Program dependencies
│       └── src/
│           ├── lib.rs         # Program entry point
│           ├── state.rs       # Program state definitions
│           ├── error.rs       # Custom error definitions
│           ├── constants.rs   # Program constants
│           └── instructions/  # Program instructions
│               ├── mod.rs     # Module exports
│               ├── initialize.rs
│               ├── register_educator.rs
│               ├── register_student.rs
│               ├── create_student_account.rs
│               ├── mint.rs
│               ├── transfer.rs
│               └── burn.rs
└── tests/                     # Program tests
```

## Testing

To run tests:

```bash
anchor test
```

## Performance Considerations

- Optimized for Solana's account model
- Efficient instruction design to minimize transaction fees
- Careful state management to reduce storage costs

## Security

- Account validation on all instructions
- Signer verification for all privileged operations
- Balance and overflow checks
- Authority-based access control

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

---

Note: This project is for educational purposes and demonstrates how to create and manage tokens on the Solana blockchain.
