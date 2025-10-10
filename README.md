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
# Create a staging task (default)
cargo run -- new-task --title "Set Lockup Contract" --proposal-id 10

# Create a production task (explicit)
cargo run -- new-task --title "Set Lockup Contract" --proposal-id 15 --env production
```

Tasks are organized by environment with independent versioning:
- `src/tasks/staging/1-set-lockup-contract.md`
- `src/tasks/production/1-set-lockup-contract.md`

### Commands

**init** - Initialize configuration
- `--account, -a`: Your NEAR account (required)

**new-task** - Generate a new task
- `--title, -t`: Title of the task (required)
- `--proposal-id, -p`: NEAR DAO proposal ID (optional)
- `--env, -e`: Environment - `staging` or `production` (default: `staging`)

## Repository Structure

```
hos-ops/
├── src/
│   ├── main.rs          # CLI tool source code
│   ├── template/        # Task template files
│   │   └── task-template.md
│   └── tasks/           # Generated security council tasks
│       ├── staging/     # Staging environment tasks
│       │   ├── 1-{title}.md
│       │   ├── 2-{title}.md
│       │   └── ...
│       └── production/  # Production environment tasks
│           ├── 1-{title}.md
│           ├── 2-{title}.md
│           └── ...
├── Cargo.toml
└── README.md
```

## Task Development

Tasks are generated in environment-specific directories (`src/tasks/staging/` or `src/tasks/production/`) using the CLI tool.

Each task is a standalone markdown file that contains:
- **Environment**: Staging or Production environment badge
- **Background**: Context about the proposal/change
- **Creator**: The security council member who created the task
- **Action to be taken**: Specific actions to execute
- **Verification steps**: Step-by-step validation process
- **Transaction links**: Links to relevant blockchain transactions

Tasks can be created by any security council member, but they must be approved by the other members of the security council before they can be run.

## Example Task

Here's what a generated task looks like:

```markdown
# Task 1: Set Lockup Contract

**Environment:** `STAGING`
**Created by:** kentf.near

## Background
Proposal #10 sets the lockup contract for the NEAR House of Stake.

## Verification Steps
1. Check the proposal using NEAR CLI
2. Decode and verify the arguments
3. Verify the target contract
4. Review transaction details
```
