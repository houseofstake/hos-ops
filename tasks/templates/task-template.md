# Task {{NUMBER}}: {{TITLE}}

**Environment:** `{{ENVIRONMENT}}`  
**Created by:** {{CREATED_BY}}

## Background

[Provide context about this proposal/change]

## Proposal Details

**Proposal ID:** #{{PROPOSAL_ID}}

## Action to be Taken

[Describe the specific action that will be executed]

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `{{ENVIRONMENT}}` task. Verify all contract addresses and proposals match the {{ENVIRONMENT}} environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
# {{ENVIRONMENT}} environment
near contract call-function as-read-only {{DAO_CONTRACT}} get_proposal json-args '{"id": {{PROPOSAL_ID}}}' network-config mainnet now
```

### Step 2: Decode and Verify Arguments

Decode the inner arguments to verify the actual parameters:

```bash
near contract call-function as-read-only {{DAO_CONTRACT}} get_proposal json-args '{"id": {{PROPOSAL_ID}}}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

### Step 3: Verify Target Contract

- [ ] **CRITICAL**: Confirm target contract matches {{ENVIRONMENT}} environment
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
