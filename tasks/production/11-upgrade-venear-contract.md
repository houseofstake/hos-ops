# Task 11: PRODUCTION: Upgrade venear contract

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to upgrade the venear contract to v1.1.1 for the **production** HoS contracts. This is a bugfix release on top of the v1.1.0 deployment from [Task 8](./8-upgrade-venear-contract.md).

Links:
- Commit: `86ed610f90f28e10977278c62f41122b850cba91`
- Github commit: https://github.com/houseofstake/house-of-stake-contracts/commit/86ed610f90f28e10977278c62f41122b850cba91
- The Github changes: https://github.com/houseofstake/house-of-stake-contracts/compare/bfb768e8fcfc97421cafc7a6dc9f4a128a5e751b...86ed610f90f28e10977278c62f41122b850cba91

### Notable changes since v1.1.0

- Fix rounding error in partial delegation math
- Fix for small NEAR lock amounts lost of cummulative veNEAR
- State migration recomputes delegated balances for existing accounts to clear orphaned/incorrect delegated amounts
- Added regression tests and mainnet test

This task uses `echo -n` to avoid adding a newline before base64 encoding.

### For reference: DAO proposal creation process

Rebuilding contracts

```bash
git checkout 86ed610f90f28e10977278c62f41122b850cba91
./build_release.sh
```

We're interested in the venear binary located at `res/release/venear_contract.wasm`
```bash
export CONTRACT_HASH=$(cat res/release/venear_contract.wasm | sha256sum | awk '{ print $1 }' | xxd -r -p | base58)
echo $CONTRACT_HASH
# Expected: 3pGv1pvYnWBX3WZsxb53wu2Ws7NRLYnWiWdQk44yTpNk
ls -l res/release/venear_contract.wasm
# Expected size: 261960 bytes
```

Uploading the contract binary to the DAO using `store_blob`, we'll need to attach `2.62 NEAR` to cover storage costs.

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT store_blob file-args res/release/venear_contract.wasm prepaid-gas '300.0 Tgas' attached-deposit '2.62 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Expected returned result: "3pGv1pvYnWBX3WZsxb53wu2Ws7NRLYnWiWdQk44yTpNk"
# TX ID: https://nearblocks.io/txns/7auqAeqGYPF4Qs2d2x9Sukzz8Vuu6QTctLbYkr71BvYT
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Upgrade venear contract to `1.1.1`'
export CONTRACT_ID='venear.dao'
export PROPOSAL_ARGS='{"proposal": {"description": "'$PROPOSAL_DESCRIPTION'","kind":{"UpgradeRemote":{"receiver_id":"'$CONTRACT_ID'","method_name":"upgrade","hash":"'$CONTRACT_HASH'"}}}}'
echo $PROPOSAL_ARGS
# Expected: {"proposal": {"description": "PRODUCTION: Upgrade venear contract to `1.1.1`","kind":{"UpgradeRemote":{"receiver_id":"venear.dao","method_name":"upgrade","hash":"3pGv1pvYnWBX3WZsxb53wu2Ws7NRLYnWiWdQk44yTpNk"}}}}
export PROPOSAL_ARGS_B64=$(echo -n $PROPOSAL_ARGS | base64)
echo $PROPOSAL_ARGS_B64
# Expected: eyJwcm9wb3NhbCI6IHsiZGVzY3JpcHRpb24iOiAiUFJPRFVDVElPTjogVXBncmFkZSB2ZW5lYXIgY29udHJhY3QgdG8gYDEuMS4xYCIsImtpbmQiOnsiVXBncmFkZVJlbW90ZSI6eyJyZWNlaXZlcl9pZCI6InZlbmVhci5kYW8iLCJtZXRob2RfbmFtZSI6InVwZ3JhZGUiLCJoYXNoIjoiM3BHdjFwdlluV0JYM1dac3hiNTN3dTJXczdOUkxZbldpV2RRazQ0eVRwTmsifX19fQ==
```

CLI command to create the proposal:

```bash
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 29
# TX ID: https://nearblocks.io/txns/Fztrfj7YHJtXeuBhvxdDYaU3NQk2M2zybDpbwfC84VS3
```

## Proposal Details

**Proposal ID:** #29

**Description:** PRODUCTION: Upgrade venear contract to `1.1.1`

**Expected result:** Once executed the proposal will call `upgrade` on the `venear.dao` contract with arguments of new contract binary with hash `3pGv1pvYnWBX3WZsxb53wu2Ws7NRLYnWiWdQk44yTpNk`. If the upgrade is successful, the venear contract will be running version `1.1.1`.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
# PRODUCTION environment
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 29}' network-config mainnet now
```

### Step 2: Verify Target Contract and Parameters

- [ ] **CRITICAL**: Confirm target contract is `venear.dao` (PRODUCTION venear contract)
- [ ] Verify the proposal kind is `UpgradeRemote`
- [ ] Verify the function being called is `upgrade`
- [ ] Check all parameters are as specified in the proposal description
- [ ] Build release artifacts (wasm) based on commit `86ed610f90f28e10977278c62f41122b850cba91`
- [ ] Check the venear contract hash from the arguments matches the expected hash `3pGv1pvYnWBX3WZsxb53wu2Ws7NRLYnWiWdQk44yTpNk`

### Step 3: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status
- [ ] Check voting requirements
- [ ] Confirm no conflicting pending proposals

## Expected Results

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal kind should be `UpgradeRemote`
- The proposal target account ID should be `venear.dao`
- The method name should be `upgrade`
- The venear contract hash should be `3pGv1pvYnWBX3WZsxb53wu2Ws7NRLYnWiWdQk44yTpNk`

## Transaction Links

- Previous task: [Task 10: PRODUCTION: Update voting contract council](./10-update-voting-contract-council.md)
- Store blob transaction: https://nearblocks.io/txns/7auqAeqGYPF4Qs2d2x9Sukzz8Vuu6QTctLbYkr71BvYT
- Proposal creation transaction: https://nearblocks.io/txns/Fztrfj7YHJtXeuBhvxdDYaU3NQk2M2zybDpbwfC84VS3
