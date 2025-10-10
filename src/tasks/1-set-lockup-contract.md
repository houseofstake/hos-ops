# Task 1: Set Lockup Contract

**Created by:** kentf.near

## Background

[Provide context about this proposal/change]

## Proposal Details

**Proposal ID:** #10

## Action to be Taken

[Describe the specific action that will be executed]

## Verification Steps

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 10}' network-config mainnet now
```

### Step 2: Decode and Verify Arguments

Decode the inner arguments to verify the actual parameters:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 10}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

### Step 3: Verify Target Contract

- [ ] Confirm the target contract address matches expectations
- [ ] Verify the function being called is correct
- [ ] Check all parameters are as specified in the proposal description

### Step 4: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status
- [ ] Check voting requirements

## Expected Results

[Describe what the verification should show]

## Transaction Links

- Proposal creation transaction: [TBD]

## Notes

[Any additional information, warnings, or considerations]

