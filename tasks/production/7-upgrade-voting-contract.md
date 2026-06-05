# Task 7: PRODUCTION: Upgrade voting contract

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to upgrade the voting contract to v1.1.0 for the **production** HoS contracts.

Links:
- Commit: `6bf9b71fd20e14dd163d845c2f8151199191b87d`
- Github commit: https://github.com/houseofstake/house-of-stake-contracts/commit/6bf9b71fd20e14dd163d845c2f8151199191b87d
- The Github changes: https://github.com/houseofstake/house-of-stake-contracts/compare/57482d60a3f4159994e3acebd5e8a76fd41b7453...6bf9b71fd20e14dd163d845c2f8151199191b87d

### Notable changes since v1.0.3

- Add fast track voting flow into the main voting contract, keeping the classic flow in parallel
- Implement shared queue logic between flows 
- Support partial delegation

This task uses `echo -n` to avoid adding a newline before base64 encoding.

### For reference: DAO proposal creation process

Rebuilding contracts

```bash
git checkout 6bf9b71fd20e14dd163d845c2f8151199191b87d
./build_release.sh
```

We're interested in the voting binary located at `res/release/voting_contract.wasm`
```bash
export CONTRACT_HASH=$(cat res/release/voting_contract.wasm | sha256sum | awk '{ print $1 }' | xxd -r -p | base58)
echo $CONTRACT_HASH
# Expected: 4stUSLywtKbrjK7Cq3xE7z7gJ72qtcZLpyTSgXQHg6YP
ls -l res/release/voting_contract.wasm
# Expected size: 351213 bytes
```

Uploading the contract binary to the DAO using `store_blob`, we'll need to attach `3.52 NEAR` to cover storage costs.

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT store_blob file-args res/release/voting_contract.wasm prepaid-gas '300.0 Tgas' attached-deposit '3.52 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Expected returned result: "4stUSLywtKbrjK7Cq3xE7z7gJ72qtcZLpyTSgXQHg6YP"
# TX ID: https://nearblocks.io/txns/CjU5nSb5bVzJveL3u2nmVbQQwCmjorLeRxx2A2Fw2Foa
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Upgrade voting contract to `1.1.0`'
export CONTRACT_ID='vote.dao'
export PROPOSAL_ARGS='{"proposal": {"description": "'$PROPOSAL_DESCRIPTION'","kind":{"UpgradeRemote":{"receiver_id":"'$CONTRACT_ID'","method_name":"upgrade","hash":"'$CONTRACT_HASH'"}}}}'
echo $PROPOSAL_ARGS
# Expected: {"proposal": {"description": "PRODUCTION: Upgrade voting contract to `1.1.0`","kind":{"UpgradeRemote":{"receiver_id":"vote.dao","method_name":"upgrade","hash":"4stUSLywtKbrjK7Cq3xE7z7gJ72qtcZLpyTSgXQHg6YP"}}}}
export PROPOSAL_ARGS_B64=$(echo -n $PROPOSAL_ARGS | base64)
echo $PROPOSAL_ARGS_B64
# Expected: eyJwcm9wb3NhbCI6IHsiZGVzY3JpcHRpb24iOiAiUFJPRFVDVElPTjogVXBncmFkZSB2b3RpbmcgY29udHJhY3QgdG8gYDEuMS4wYCIsImtpbmQiOnsiVXBncmFkZVJlbW90ZSI6eyJyZWNlaXZlcl9pZCI6InZvdGUuZGFvIiwibWV0aG9kX25hbWUiOiJ1cGdyYWRlIiwiaGFzaCI6IjRzdFVTTHl3dEticmpLN0NxM3hFN3o3Z0o3MnF0Y1pMcHlUU2dYUUhnNllQIn19fX0=
```

CLI command to create the proposal:

```bash
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 23
# TX ID: https://nearblocks.io/txns/8SVsUVJnE7WFj174d7CtXAdAjv8RyWHNfAUxunLYK1Gb
```

## Proposal Details

**Proposal ID:** #23

**Description:** PRODUCTION: Upgrade voting contract to `1.1.0`

**Expected result:** Once executed the proposal will call `upgrade` on the `vote.dao` contract with arguments of new contract binary with hash `4stUSLywtKbrjK7Cq3xE7z7gJ72qtcZLpyTSgXQHg6YP`. If the upgrade is successful, the voting contract will be running version `1.1.0`.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
# PRODUCTION environment
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 23}' network-config mainnet now
```

### Step 2: Verify Target Contract and Parameters

- [ ] **CRITICAL**: Confirm target contract matches PRODUCTION environment
- [ ] Verify the proposal kind is `UpgradeRemote`
- [ ] Verify the function being called is `upgrade`
- [ ] Check all parameters are as specified in the proposal description
- [ ] Build release artifacts (wasm) based on commit `6bf9b71fd20e14dd163d845c2f8151199191b87d`
- [ ] Check the voting contract hash from the arguments matches the expected hash `4stUSLywtKbrjK7Cq3xE7z7gJ72qtcZLpyTSgXQHg6YP`
- [ ] If PRODUCTION, double-check proposal description for any STAGING indicators

### Step 3: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status
- [ ] Check voting requirements
- [ ] Confirm no conflicting pending proposals

## Expected Results

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal kind should be `UpgradeRemote`
- The proposal target account ID should be `vote.dao`
- The method name should be `upgrade`
- The voting contract hash should be `4stUSLywtKbrjK7Cq3xE7z7gJ72qtcZLpyTSgXQHg6YP`

## Transaction Links

- Previous task: [Task 6: PRODUCTION: Update voting contract reviewers list](./6-update-voting-contract-reviewers.md)
- Store blob transaction: https://nearblocks.io/txns/CjU5nSb5bVzJveL3u2nmVbQQwCmjorLeRxx2A2Fw2Foa
- Proposal creation transaction: https://nearblocks.io/txns/8SVsUVJnE7WFj174d7CtXAdAjv8RyWHNfAUxunLYK1Gb
