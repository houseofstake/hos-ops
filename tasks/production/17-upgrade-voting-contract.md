# Task 17: PRODUCTION: Upgrade voting contract

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to upgrade the voting contract to v1.1.2 for the **production** HoS contracts.

Links:
- Commit: `32611da14aed42127421e777eec0e98094de6d2d`
- Github commit: https://github.com/houseofstake/house-of-stake-contracts/commit/32611da14aed42127421e777eec0e98094de6d2d
- The Github changes: https://github.com/houseofstake/house-of-stake-contracts/compare/f3bad575341507d0c63c7e219a19a88c5a9527d2...32611da14aed42127421e777eec0e98094de6d2d

### Notable changes since v1.1.1

- Add simple/strong majority selection for classic proposals; `approve_proposal` now requires `majority_type` for both classic and fast track flows
- Schedule weekly voting starts for fast track proposals for Monday 00:00 UTC instead of Monday 00:00 CET (fixed UTC+1)

This task uses `echo -n` to avoid adding a newline before base64 encoding.

### For reference: DAO proposal creation process

Rebuilding contracts

```bash
git checkout 32611da14aed42127421e777eec0e98094de6d2d
./build_release.sh
```

We're interested in the voting binary located at `res/release/voting_contract.wasm`
```bash
export CONTRACT_HASH=$(cat res/release/voting_contract.wasm | sha256sum | awk '{ print $1 }' | xxd -r -p | base58)
echo $CONTRACT_HASH
# Expected: 6tquw22BwFDJKrdt3Bde7dgQysLmwEsybZjttCncbFGr
ls -l res/release/voting_contract.wasm
# Expected size: 347268 bytes
```

Uploading the contract binary to the DAO using `store_blob`, we'll need to attach `3.48 NEAR` to cover storage costs.

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT store_blob file-args res/release/voting_contract.wasm prepaid-gas '300.0 Tgas' attached-deposit '3.48 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Expected returned result: "6tquw22BwFDJKrdt3Bde7dgQysLmwEsybZjttCncbFGr"
# TX ID: https://nearblocks.io/txns/EgrxRPuNUsx3PsFnLxayaiJSQGksqW1KHCmMvBqnyxud
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Upgrade voting contract to `1.1.2`'
export CONTRACT_ID='vote.dao'
export PROPOSAL_ARGS='{"proposal": {"description": "'$PROPOSAL_DESCRIPTION'","kind":{"UpgradeRemote":{"receiver_id":"'$CONTRACT_ID'","method_name":"upgrade","hash":"'$CONTRACT_HASH'"}}}}'
echo $PROPOSAL_ARGS
# Expected: {"proposal": {"description": "PRODUCTION: Upgrade voting contract to `1.1.2`","kind":{"UpgradeRemote":{"receiver_id":"vote.dao","method_name":"upgrade","hash":"6tquw22BwFDJKrdt3Bde7dgQysLmwEsybZjttCncbFGr"}}}}
export PROPOSAL_ARGS_B64=$(echo -n $PROPOSAL_ARGS | base64)
echo $PROPOSAL_ARGS_B64
# Expected: eyJwcm9wb3NhbCI6IHsiZGVzY3JpcHRpb24iOiAiUFJPRFVDVElPTjogVXBncmFkZSB2b3RpbmcgY29udHJhY3QgdG8gYDEuMS4yYCIsImtpbmQiOnsiVXBncmFkZVJlbW90ZSI6eyJyZWNlaXZlcl9pZCI6InZvdGUuZGFvIiwibWV0aG9kX25hbWUiOiJ1cGdyYWRlIiwiaGFzaCI6IjZ0cXV3MjJCd0ZESktyZHQzQmRlN2RnUXlzTG13RXN5YlpqdHRDbmNiRkdyIn19fX0=
```

CLI command to create the proposal:

```bash
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 40
# TX ID: https://nearblocks.io/txns/EDMxDWd9TV69gafbsKNtS4qarisJMzo7cDdmTBPKRMXL
```

## Proposal Details

**Proposal ID:** #40

**Description:** PRODUCTION: Upgrade voting contract to `1.1.2`

**Expected result:** Once executed the proposal will call `upgrade` on the `vote.dao` contract with arguments of new contract binary with hash `6tquw22BwFDJKrdt3Bde7dgQysLmwEsybZjttCncbFGr`. If the upgrade is successful, the voting contract will be running version `1.1.2`.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
# PRODUCTION environment
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 40}' network-config mainnet now
```

### Step 2: Verify Target Contract and Parameters

- [ ] **CRITICAL**: Confirm target contract matches PRODUCTION environment
- [ ] Verify the proposal kind is `UpgradeRemote`
- [ ] Verify the function being called is `upgrade`
- [ ] Check all parameters are as specified in the proposal description
- [ ] Build release artifacts (wasm) based on commit `32611da14aed42127421e777eec0e98094de6d2d`
- [ ] Check the voting contract hash from the arguments matches the expected hash `6tquw22BwFDJKrdt3Bde7dgQysLmwEsybZjttCncbFGr`
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
- The voting contract hash should be `6tquw22BwFDJKrdt3Bde7dgQysLmwEsybZjttCncbFGr`

## Transaction Links

- Previous task: [Task 16: PRODUCTION: Upgrade opener contract on `registrar`](./16-upgrade-registrar-opener.md)
- Store blob transaction: https://nearblocks.io/txns/EgrxRPuNUsx3PsFnLxayaiJSQGksqW1KHCmMvBqnyxud
- Proposal creation transaction: https://nearblocks.io/txns/EDMxDWd9TV69gafbsKNtS4qarisJMzo7cDdmTBPKRMXL
