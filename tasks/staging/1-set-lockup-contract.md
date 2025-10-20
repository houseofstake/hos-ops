# Task 1: Set Lockup Contract

**Environment:** `STAGING`  
**Created by:** ek

## Background

Proposal to set the lockup contract for the staging HoS contracts

## Proposal Details

**Proposal ID:** #10

## Action to be Taken


## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `STAGING` task. Verify all contract addresses and proposals match the STAGING environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
# STAGING environment
near contract call-function as-read-only hos-root-staging.sputnik-dao.near get_proposal json-args '{"id": 10}' network-config mainnet now
```

### Step 2: Decode and Verify Arguments

Decode the inner arguments to verify the actual parameters:

```bash
near contract call-function as-read-only hos-root-staging.sputnik-dao.near get_proposal json-args '{"id": 10}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

### Step 3: Verify Target Contract

- [ ] **CRITICAL**: Confirm target contract matches STAGING environment
- [ ] Verify the function being called is correct
- [ ] Check all parameters are as specified in the proposal description
- [ ] If PRODUCTION, double-check proposal description for any STAGING indicators

### Step 4: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status
- [ ] Check voting requirements
- [ ] Confirm no conflicting pending proposals

## Expected Results

[Describe what the verification should show]

## Transaction Links

- Proposal creation transaction: [TBD]

## Notes

