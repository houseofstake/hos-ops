# NEAR House of Stake Ops Task System

A tooling system for security council members to run security tasks on the NEAR House of Stake.

## Usage

### First Time Setup

Initialize your config file with your NEAR account:

```bash
cargo run -- init --account kentf.near
```

This creates a `hos-ops.toml` file with your NEAR account, which will be added to all tasks you create.

### Generate a New Task

Create a new security council task from the template:

```bash
cargo run -- new --title "Set Lockup Contract" --proposal-id 10
```

This will generate a new task file in `src/tasks/` with automatic versioning (e.g., `1-set-lockup-contract.md`) and mark you as the creator.

### Commands

**init** - Initialize configuration
- `--account, -a`: Your NEAR account (required)

**new** - Generate a new task
- `--title, -t`: Title of the task (required)
- `--proposal-id, -p`: NEAR DAO proposal ID (optional)

## Repo strructure

```
hos-ops/
├── src/
│   ├── main.rs          # CLI tool source code
│   ├── template/        # Task template files
│   │   └── task-template.md
│   └── tasks/           # Generated security council tasks
├── Cargo.toml
└── README.md
```

## Task Development

Tasks are generated in the `src/tasks/` directory using the CLI tool.

Each task is a standalone markdown file that contains:
- **Background**: Context about the proposal/change
- **Author**: The security council member who created the task
- **Action to be taken**: Specific actions to execute
- **Verification steps**: Step-by-step validation process
- **Transaction links**: Links to relevant blockchain transactions

Tasks can be created by any security council member, but they must be approved by the other members of the security council before they can be run.

## Example Task

Here's what a generated task looks like:

```markdown
# Task 1: Set Lockup Contract

## Background
Proposal #10 sets the lockup contract for the NEAR House of Stake.

## Verification Steps
1. Check the proposal using NEAR CLI
2. Decode and verify the arguments
3. Verify the target contract
4. Review transaction details
```
