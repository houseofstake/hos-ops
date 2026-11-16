# Task 21: PRODUCTION: Set veNEAR unlock duration to 5 days

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to set the veNEAR unlock (release) duration on the **production** venear contract (`venear.dao`) from 45 days to 5 days.

- Current value: `unlock_duration_ns` = `3888000000000000` (45 days)
- New value: `unlock_duration_sec` = `432000` (5 days), stored as `unlock_duration_ns` = `432000000000000`

`set_unlock_duration_sec` only updates the venear config. Each lockup stores `unlock_duration_ns` when it is deployed, so the change applies to new lockups only. Existing lockups keep the 45 day unlock duration.

Contract method: https://github.com/houseofstake/house-of-stake-contracts/blob/86ed610f90f28e10977278c62f41122b850cba91/venear-contract/src/governance.rs#L71-L80

### For reference: DAO proposal creation process

Encode inner args:

```bash
export INNER_ARGS='{"unlock_duration_sec":432000}'
echo $INNER_ARGS
export INNER_ARGS_B64=$(printf '%s' "$INNER_ARGS" | base64 | tr -d '\n')
echo $INNER_ARGS_B64
# Expected: eyJ1bmxvY2tfZHVyYXRpb25fc2VjIjo0MzIwMDB9
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Set veNEAR unlock duration to 5 days on `venear.dao`'
export CONTRACT_ID='venear.dao'
export PROPOSAL_ARGS=$(jq -nc --arg description "$PROPOSAL_DESCRIPTION" --arg receiver_id "$CONTRACT_ID" --arg args "$INNER_ARGS_B64" '{proposal: {description: $description, kind: {FunctionCall: {receiver_id: $receiver_id, actions: [{method_name: "set_unlock_duration_sec", args: $args, deposit: "1", gas: "20000000000000"}]}}}}')
echo $PROPOSAL_ARGS
# Expected: {"proposal":{"description":"PRODUCTION: Set veNEAR unlock duration to 5 days on `venear.dao`","kind":{"FunctionCall":{"receiver_id":"venear.dao","actions":[{"method_name":"set_unlock_duration_sec","args":"eyJ1bmxvY2tfZHVyYXRpb25fc2VjIjo0MzIwMDB9","deposit":"1","gas":"20000000000000"}]}}}}
export PROPOSAL_ARGS_B64=$(printf '%s' "$PROPOSAL_ARGS" | base64 | tr -d '\n')
echo $PROPOSAL_ARGS_B64
# Expected: eyJwcm9wb3NhbCI6eyJkZXNjcmlwdGlvbiI6IlBST0RVQ1RJT046IFNldCB2ZU5FQVIgdW5sb2NrIGR1cmF0aW9uIHRvIDUgZGF5cyBvbiBgdmVuZWFyLmRhb2AiLCJraW5kIjp7IkZ1bmN0aW9uQ2FsbCI6eyJyZWNlaXZlcl9pZCI6InZlbmVhci5kYW8iLCJhY3Rpb25zIjpbeyJtZXRob2RfbmFtZSI6InNldF91bmxvY2tfZHVyYXRpb25fc2VjIiwiYXJncyI6ImV5SjFibXh2WTJ0ZlpIVnlZWFJwYjI1ZmMyVmpJam8wTXpJd01EQjkiLCJkZXBvc2l0IjoiMSIsImdhcyI6IjIwMDAwMDAwMDAwMDAwIn1dfX19fQ==
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 45
# TX ID: https://nearblocks.io/txns/mN7CUbBJh5GFspupG5wABsusAns6d1rShmVCt72wGmn
```

## Proposal Details

**Proposal ID:** #45

**Description:** PRODUCTION: Set veNEAR unlock duration to 5 days on `venear.dao`

**Expected result:** Once executed, the proposal will call `set_unlock_duration_sec` on `venear.dao` with `unlock_duration_sec` = `432000`. `get_config` on `venear.dao` will return `unlock_duration_ns` = `"432000000000000"`, and lockups created after that will have a 5 day unlock duration.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 45}' network-config mainnet now
```

### Step 2: Decode and Verify Arguments

Decode the inner arguments to verify the actual parameters:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 45}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

Expected decoded inner args:

```json
{
  "unlock_duration_sec": 432000
}
```

### Step 3: Verify Target Contract and Parameters

- [ ] **CRITICAL**: Confirm target contract is `venear.dao`
- [ ] Verify the proposal kind is `FunctionCall` with exactly 1 action
- [ ] Verify the function being called is `set_unlock_duration_sec`
- [ ] Check `unlock_duration_sec` in the decoded inner arguments (not just the proposal description) is `432000`
- [ ] Verify the deposit is `1` yoctoNEAR and the gas is `20000000000000` (20 Tgas)

### Step 4: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status
- [ ] Check voting requirements
- [ ] Confirm no conflicting pending proposals

## Expected Results

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal kind should be `FunctionCall`
- The proposal target account ID should be `venear.dao`
- The method name should be `set_unlock_duration_sec`
- The `unlock_duration_sec` argument should be `432000`
- The deposit should be `1` (yoctoNEAR)
- The gas should be `20000000000000` (20 Tgas)

## Transaction Links

- Previous task: [Task 20: PRODUCTION: Approve TLA Cohort Two batch on `registrar`](./20-approve-tla-cohort-two-batch.md)
- Proposal creation transaction: https://nearblocks.io/txns/mN7CUbBJh5GFspupG5wABsusAns6d1rShmVCt72wGmn
