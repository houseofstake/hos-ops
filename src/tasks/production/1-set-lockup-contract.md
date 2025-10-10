# Task 1: Set Lockup Contract

**Environment:** `PRODUCTION`  
**Created by:** fastnear-hos.near

## Background

Proposal to set the lockup contract for the staging HoS contracts.

**Description:** PROD: Set lockup contract hash to `EV4eXNuKVkcYisktcT4sk9XfFFRvcefy51Qs2hQkhnK1` and minimum lockup deposit to 2 NEAR

### For reference: DAO proposal creation process

Encode inner args:

```bash
export INNER_ARGS='{"contract_hash": "EV4eXNuKVkcYisktcT4sk9XfFFRvcefy51Qs2hQkhnK1", "min_lockup_deposit": "2000000000000000000000000"}'
echo $INNER_ARGS
export INNER_ARGS_B64=$(echo $INNER_ARGS | base64)
echo $INNER_ARGS_B64
```

Base64 inner args result:
```
eyJjb250cmFjdF9oYXNoIjogIkVWNGVYTnVLVmtjWWlza3RjVDRzazlYZkZGUnZjZWZ5NTFRczJoUWtobksxIiwgIm1pbl9sb2NrdXBfZGVwb3NpdCI6ICIyMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwIn0K
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PROD: Set lockup contract hash to `EV4eXNuKVkcYisktcT4sk9XfFFRvcefy51Qs2hQkhnK1` and minimum lockup deposit to 2 NEAR'
export CONTRACT_ID='venear.dao'
export PROPOSAL_ARGS='{"proposal": {"description": "'$PROPOSAL_DESCRIPTION'","kind":{"FunctionCall":{"receiver_id":"'$CONTRACT_ID'","actions":[{"method_name":"set_lockup_contract","args":"'$INNER_ARGS_B64'","deposit":"1","gas":"20000000000000"}]}}}}'
echo $PROPOSAL_ARGS
export PROPOSAL_ARGS_B64=$(echo $PROPOSAL_ARGS | base64)
echo $PROPOSAL_ARGS_B64
```

Base64 proposal args result:
```
eyJwcm9wb3NhbCI6IHsiZGVzY3JpcHRpb24iOiAiUFJPRDogU2V0IGxvY2t1cCBjb250cmFjdCBoYXNoIHRvIGBFVjRlWE51S1ZrY1lpc2t0Y1Q0c2s5WGZGRlJ2Y2VmeTUxUXMyaFFraG5LMWAgYW5kIG1pbmltdW0gbG9ja3VwIGRlcG9zaXQgdG8gMiBORUFSIiwia2luZCI6eyJGdW5jdGlvbkNhbGwiOnsicmVjZWl2ZXJfaWQiOiJ2ZW5lYXIuZGFvIiwiYWN0aW9ucyI6W3sibWV0aG9kX25hbWUiOiJzZXRfbG9ja3VwX2NvbnRyYWN0IiwiYXJncyI6ImV5SmpiMjUwY21GamRGOW9ZWE5vSWpvZ0lrVldOR1ZZVG5WTFZtdGpXV2x6YTNSalZEUnphemxZWmtaR1VuWmpaV1o1TlRGUmN6Sm9VV3RvYmtzeElpd2dJbTFwYmw5c2IyTnJkWEJmWkdWd2IzTnBkQ0k2SUNJeU1EQXdNREF3TURBd01EQXdNREF3TURBd01EQXdNREF3SW4wSyIsImRlcG9zaXQiOiIxIiwiZ2FzIjoiMjAwMDAwMDAwMDAwMDAifV19fX19Cg
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="fastnear-hos.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet 
```

## Proposal Details

**Proposal ID:** #11

## Action to be Taken

[Describe the specific action that will be executed]

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
# PRODUCTION environment
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 11}' network-config mainnet now
```

### Step 2: Decode and Verify Arguments

Decode the inner arguments to verify the actual parameters:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 11}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

### Step 3: Verify Target Contract and Parameters

- [ ] **CRITICAL**: Confirm target contract from the proposal (e.g. `venear.dao`) matches PRODUCTION environment
- [ ] Verify the function being called is correct (e.g. `set_lockup_contract`)
- [ ] Check all parameters are as specified in the proposal description
  - [ ] Build release artifacts (wasm) based on commit specified by the release
  - [ ] Check the lockup contract hash from the inner arguments (not just proposal description) matches the expected hash
- [ ] If PRODUCTION, double-check proposal description for any STAGING indicators

### Step 4: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status
- [ ] Check voting requirements
- [ ] Confirm no conflicting pending proposals

## Expected Results (you should double-check values here)

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal target account ID should be `venear.dao`
- The method name should be `set_lockup_contract`
- The lockup contract hash should be `EV4eXNuKVkcYisktcT4sk9XfFFRvcefy51Qs2hQkhnK1`
- The minimum lockup deposit should be `2000000000000000000000000` (2 NEAR)
- The deposit should be `1` (yoctoNEAR)
- The gas should be `20000000000000` (20 Tgas)

## Transaction Links

- Proposal creation transaction: https://nearblocks.io/txns/B6KS3eFBYkttYydBuAL5DHB31btSCkVyNjjJuevXudRQ

## Notes
