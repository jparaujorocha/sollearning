# SolLearning: Decentralized Educational Rewards Platform on Solana

## Project Overview

SolLearning is a blockchain-powered educational rewards platform built on Solana, revolutionizing how educational achievements are recognized and incentivized. The platform enables authorized educators to mint tokens (SLEARNING) as verifiable rewards for student course completions, creating a transparent and immutable record of educational progress on the blockchain.

Students earn tokens for completing courses, which can be transferred or used within the educational ecosystem. This creates a blockchain-based verification system for educational accomplishments and provides tangible rewards for learning achievements.

## Key Features

- Token-based educational achievements with on-chain verification
- Role-based system with administrators, educators, and students
- Course creation and completion tracking
- Secure token minting, transfer, and burning mechanisms
- Governance through multisignature proposals
- Emergency pause functionality for enhanced security
- Comprehensive configuration controls

## Technical Specifications

- **Blockchain**: Solana
- **Development Framework**: Anchor (v0.31.0)
- **Programming Language**: Rust
- **Token Standard**: SPL Token
- **Token Name**: SolLearning (SLEARNING)
- **Initial Supply**: 100,000,000 tokens
- **Decimals**: 9 (Solana standard)
- **Minting Mechanism**: Controlled educator-driven rewards

## System Architecture

### State Accounts

- **ProgramState**: Core program data including token mint address, authority, and metrics
- **ProgramConfig**: Program configurations like educator and course limits
- **EducatorAccount**: Information about authorized educators with minting permissions
- **StudentInfo**: Student records tracking achievements and token earnings
- **Course**: Course details including reward amounts and completion tracking
- **CourseCompletion**: Records of student course completions and rewards
- **Multisig**: Multi-signature governance structure for program administration
- **Proposal**: Governance proposals for program changes
- **EmergencyMultisig**: Separate multisig for emergency controls

### Key Modules

1. **Educator Management**
   - Registration of authorized educators
   - Status control (active/inactive)
   - Minting permissions and limits

2. **Student Management**
   - Student registration and tracking
   - Token account creation
   - Achievement recording

3. **Course Management**
   - Course creation with defined rewards
   - Course metadata and updates
   - Completion verification and tracking

4. **Token Operations**
   - Controlled minting as educational rewards
   - Secure transfers between accounts
   - Token burning mechanisms
   - Balance tracking and reporting

5. **Governance**
   - Multisignature proposal system
   - Threshold-based approvals
   - Time-bound execution windows

6. **Security Features**
   - Role-based access controls
   - Emergency pause functionality
   - Function-specific granular pauses
   - Comprehensive input validation

## Development Environment Setup

### Prerequisites
- Rust 1.85.0+ (recommended installation via rustup)
- Solana CLI 2.1.x
- Node.js 23.x
- Anchor 0.30.x+
- Yarn or npm
- Cargo Fuzz (for fuzzing tests)

### Installation

1. **Install Rust and Solana Tools**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Configure Rust toolchain
rustup default stable
rustup component add rustfmt clippy

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v2.1.14/install)"

# Configure Solana CLI
solana config set --url localhost
solana-keygen new  # Generate a new keypair
```

2. **Install Anchor and Project Dependencies**
```bash
# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Clone the project
git clone https://github.com/your-organization/sollearning.git
cd sollearning

# Install Node.js dependencies
yarn install

# Install Cargo Fuzz (for advanced testing)
cargo install cargo-fuzz
```

3. **Build and Deploy**
```bash
# Build the program
anchor build

# Deploy to local validator
solana-test-validator
anchor deploy

# Or deploy to a specific network
solana config set --url devnet
anchor deploy
```

## Program Usage

### Initialize the Token and Program

```bash
solana program call <PROGRAM_ID> initialize \
    --keypair <PATH_TO_ADMIN_KEYPAIR>
```

### Register and Manage Educators

```bash
# Register an educator
solana program call <PROGRAM_ID> register_educator \
    --keypair <PATH_TO_ADMIN_KEYPAIR> \
    --account educator:<EDUCATOR_ADDRESS> \
    --uint64 mint_limit:1000000000000

# Update educator status
solana program call <PROGRAM_ID> set_educator_status \
    --keypair <PATH_TO_ADMIN_KEYPAIR> \
    --account educator:<EDUCATOR_ADDRESS> \
    --bool is_active:true \
    --uint64 new_mint_limit:2000000000000
```

### Student Operations

```bash
# Register a student
solana program call <PROGRAM_ID> register_student \
    --keypair <PATH_TO_PAYER_KEYPAIR> \
    --account student:<STUDENT_ADDRESS>

# Create a token account for a student
solana program call <PROGRAM_ID> create_student_token_account \
    --keypair <PATH_TO_PAYER_KEYPAIR> \
    --account student:<STUDENT_ADDRESS>
```

### Course Management

```bash
# Create a course
solana program call <PROGRAM_ID> create_course \
    --keypair <PATH_TO_EDUCATOR_KEYPAIR> \
    --string course_id:"COURSE-123" \
    --string course_name:"Blockchain Fundamentals" \
    --uint64 reward_amount:1000000000

# Update a course
solana program call <PROGRAM_ID> update_course \
    --keypair <PATH_TO_EDUCATOR_KEYPAIR> \
    --string course_id:"COURSE-123" \
    --string course_name:"Advanced Blockchain Fundamentals" \
    --bool is_active:true \
    --string change_description:"Updated course name"
```

### Token Operations

```bash
# Mint tokens to a student for course completion
solana program call <PROGRAM_ID> mint_to_student \
    --keypair <PATH_TO_EDUCATOR_KEYPAIR> \
    --account student:<STUDENT_ADDRESS> \
    --uint64 amount:1000000000 \
    --string course_id:"COURSE-123"

# Transfer tokens
solana program call <PROGRAM_ID> transfer \
    --keypair <PATH_TO_SENDER_KEYPAIR> \
    --account recipient:<RECIPIENT_ADDRESS> \
    --uint64 amount:500000000

# Burn tokens
solana program call <PROGRAM_ID> burn \
    --keypair <PATH_TO_OWNER_KEYPAIR> \
    --uint64 amount:200000000
```

### Governance Operations

```bash
# Create multisig
solana program call <PROGRAM_ID> create_multisig \
    --keypair <PATH_TO_AUTHORITY_KEYPAIR> \
    --pubkey[] signers:[<SIGNER1>,<SIGNER2>,<SIGNER3>] \
    --uint8 threshold:2

# Create a proposal
solana program call <PROGRAM_ID> create_proposal \
    --keypair <PATH_TO_PROPOSER_KEYPAIR> \
    --enum instruction:ChangeAuthority \
    --pubkey new_authority:<NEW_AUTHORITY_ADDRESS> \
    --string description:"Change program authority"

# Approve a proposal
solana program call <PROGRAM_ID> approve_proposal \
    --keypair <PATH_TO_SIGNER_KEYPAIR>

# Execute a proposal
solana program call <PROGRAM_ID> execute_proposal \
    --keypair <PATH_TO_EXECUTOR_KEYPAIR>
```

## Security Features

- **Role-Based Access Control**: Clear separation of administrator, educator, and student roles
- **Multisignature Governance**: Critical program changes require multiple approvals
- **Emergency Pause**: Ability to quickly pause the entire program or specific functions
- **Input Validation**: Comprehensive validation on all inputs to prevent exploits
- **Expiration Mechanisms**: Time-bound proposals to prevent stale actions
- **Threshold Controls**: Configurable approval thresholds for governance actions
- **Function-Level Granularity**: Individual functions can be paused independently

## Testing

SolLearning includes a comprehensive testing strategy:

### Integration Tests

```bash
# Run all tests
anchor test

# Run specific test suites
anchor test tests/integration/educator
anchor test tests/integration/student
anchor test tests/integration/course
anchor test tests/integration/token
```

### Fuzzing Tests

SolLearning employs advanced fuzzing for identifying potential vulnerabilities:

```bash
# Run initialization fuzzing
cargo fuzz run fuzz_initialization

# Run educator-related fuzzing
cargo fuzz run fuzz_educator

# Run token transaction fuzzing
cargo fuzz run fuzz_token
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
│           ├── constants.rs   # Program constants
│           ├── error.rs       # Custom error definitions
│           ├── instructions/  # Program instructions by module
│           │   ├── educator/  # Educator-related instructions
│           │   ├── student/   # Student-related instructions
│           │   ├── course/    # Course-related instructions
│           │   ├── token/     # Token operation instructions
│           │   ├── config/    # Configuration instructions
│           │   ├── multisig/  # Multisignature instructions
│           │   ├── proposal/  # Proposal instructions
│           │   ├── emergency/ # Emergency control instructions
│           │   └── initialize/# Initialization instructions
│           ├── states/        # Account state definitions
│           └── utils/         # Utility functions
├── tests/                     # Test framework
│   ├── common/                # Common test utilities
│   └── integration/           # Integration tests by module
│       ├── educator/          # Educator-related tests
│       ├── student/           # Student-related tests
│       ├── course/            # Course-related tests
│       ├── token/             # Token operation tests
│       ├── multisig/          # Multisignature tests
│       ├── proposal/          # Proposal tests
│       └── emergency/         # Emergency control tests
```

## Performance Optimizations

- **Efficient Account Structure**: Optimized account layouts to minimize storage costs
- **Minimal Transaction Complexity**: Instructions designed to reduce computational overhead
- **Batched Operations**: Where appropriate, operations are batched to reduce fees
- **Solana-Specific Optimizations**: Leverage Solana's parallel execution capabilities
- **Rust Performance Features**: Utilizing Rust's zero-cost abstractions and ownership model

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Distributed under the ISC License. See `LICENSE` for more information.

---

This project demonstrates a complete decentralized application on Solana for educational achievement verification and rewards.
