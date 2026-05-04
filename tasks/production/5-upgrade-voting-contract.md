# Task 5: PRODUCTION: Upgrade voting contract

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to upgrade the voting contract to v1.0.3 for the **production** HoS contracts.

Links:
- Commit: `57482d60a3f4159994e3acebd5e8a76fd41b7453`
- Github commit: https://github.com/houseofstake/house-of-stake-contracts/commit/57482d60a3f4159994e3acebd5e8a76fd41b7453
- The Github changes: https://github.com/houseofstake/house-of-stake-contracts/compare/ff25d7de8af1a778b0c1b0df2084e05cff1424b8...57482d60a3f4159994e3acebd5e8a76fd41b7453

### Notable changes since v1.0.2

- Update voting flow with timelock; reviewers only approve, council only veto
- Add migration from the old state
- Add `rejecter_id` to track vetoed account for proposal
- Add expiration time before approval
- On-chain quorum; simplify voting to `For` / `Against` / `Abstain`;
- On-chain execution for proposals
- Add `noveto` action for council
- Add reject functionality to reviewer
- Keep legacy proposal statuses to enum
- Setup production values on contract migration;

This task uses `echo -n` to avoid adding a newline before base64 encoding.

### For reference: DAO proposal creation process

Rebuilding contracts

```bash
git checkout 57482d60a3f4159994e3acebd5e8a76fd41b7453
./build_release.sh
```

We're interested in the voting binary located at `res/release/voting_contract.wasm`
```bash
export CONTRACT_HASH=$(cat res/release/voting_contract.wasm | sha256sum | awk '{ print $1 }' | xxd -r -p | base58)
echo $CONTRACT_HASH
# Expected: 8wDSroVX5ZWqHjwvwRPpy3ckncTDSQCBwsoNUrsyMKJA
ls -l res/release/voting_contract.wasm
# Size: 275883
```

Uploading the contract binary to the DAO using `store_blob`, we'll need to attach `2.76 NEAR` to cover storage costs.

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT store_blob file-args res/release/voting_contract.wasm prepaid-gas '300.0 Tgas' attached-deposit '2.76 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Expected returned result: "8wDSroVX5ZWqHjwvwRPpy3ckncTDSQCBwsoNUrsyMKJA"
# TX ID: https://nearblocks.io/txns/5KHMX3Q81SQ5Fu5brudBEiE6BtihYjXoxFSoHsi7iCsn
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Upgrade voting contract to `1.0.3`'
export CONTRACT_ID='vote.dao'
export PROPOSAL_ARGS='{"proposal": {"description": "'$PROPOSAL_DESCRIPTION'","kind":{"UpgradeRemote":{"receiver_id":"'$CONTRACT_ID'","method_name":"upgrade","hash":"'$CONTRACT_HASH'"}}}}'
echo $PROPOSAL_ARGS
# Expected: {"proposal": {"description": "PRODUCTION: Upgrade voting contract to `1.0.3`","kind":{"UpgradeRemote":{"receiver_id":"vote.dao","method_name":"upgrade","hash":"8wDSroVX5ZWqHjwvwRPpy3ckncTDSQCBwsoNUrsyMKJA"}}}}
export PROPOSAL_ARGS_B64=$(echo -n $PROPOSAL_ARGS | base64)
echo $PROPOSAL_ARGS_B64
# Expected: eyJwcm9wb3NhbCI6IHsiZGVzY3JpcHRpb24iOiAiUFJPRFVDVElPTjogVXBncmFkZSB2b3RpbmcgY29udHJhY3QgdG8gYDEuMC4zYCIsImtpbmQiOnsiVXBncmFkZVJlbW90ZSI6eyJyZWNlaXZlcl9pZCI6InZvdGUuZGFvIiwibWV0aG9kX25hbWUiOiJ1cGdyYWRlIiwiaGFzaCI6Ijh3RFNyb1ZYNVpXcUhqd3Z3UlBweTNja25jVERTUUNCd3NvTlVyc3lNS0pBIn19fX0=
```

CLI command to create the proposal:

```bash
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 21
# TX ID: https://nearblocks.io/txns/DFs9HtQwj66D96xzdhM3knX3Fdh8eAAyMKFn6F2xTppD
```

## Proposal Details

**Proposal ID:** #21

**Description:** PRODUCTION: Upgrade voting contract to `1.0.3`

**Expected result:** Once executed the proposal will call `upgrade` on the `vote.dao` contract with arguments of new contract binary with hash `8wDSroVX5ZWqHjwvwRPpy3ckncTDSQCBwsoNUrsyMKJA`. If the upgrade is successful, the voting contract will be running version `1.0.3`.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
# PRODUCTION environment
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 21}' network-config mainnet now
```

### Step 2: Verify Target Contract and Parameters

- [ ] **CRITICAL**: Confirm target contract matches PRODUCTION environment
- [ ] Verify the proposal kind is `UpgradeRemote`
- [ ] Verify the function being called is `upgrade`
- [ ] Check all parameters are as specified in the proposal description
  - [ ] Build release artifacts (wasm) based on commit `57482d60a3f4159994e3acebd5e8a76fd41b7453`
  - [ ] Check the voting contract hash from the arguments matches the expected hash `8wDSroVX5ZWqHjwvwRPpy3ckncTDSQCBwsoNUrsyMKJA`
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
- The voting contract hash should be `8wDSroVX5ZWqHjwvwRPpy3ckncTDSQCBwsoNUrsyMKJA`

## Transaction Links

- Previous task: [Task 3: PRODUCTION: Upgrade voting contract](./3-upgrade-voting-contract.md)
- Store blob transaction: https://nearblocks.io/txns/5KHMX3Q81SQ5Fu5brudBEiE6BtihYjXoxFSoHsi7iCsn
- Proposal creation transaction: https://nearblocks.io/txns/DFs9HtQwj66D96xzdhM3knX3Fdh8eAAyMKFn6F2xTppD
