# Task 18: PRODUCTION: Rotate a recovery watcher and the recovery attestation key on `rec.hos`

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to rotate one recovery watcher and the recovery attestation key on the **production** recovery contract (`rec.hos`) in a single DAO proposal.

The proposal will call `set_watchers` once, followed by `install_policy` for each of the 67 enrolled accounts, as one `FunctionCall` with 68 actions on `rec.hos`.

### Watcher being rotated

- Removed watcher: `ed25519:9vbwXrU3jNftszWiy3mABigWT7YkJQdVuiE5kzYU4J7f`
- Added watcher: `ed25519:7d4A29Rki4XLPVLvmcibJCd1tmWt6Eesznu8dCjJ1kxJ`
- Unchanged watchers: `ed25519:BcRjtC4npBDcRpVY6atmm5pSLwuv3ETV8CCpk47eeVau`, `ed25519:J8PKegsQptLcux9oeaLKCDETREtJZtfQ9ZDojgKg7MDf`
- Threshold: `2` (unchanged)

### Policies being reinstalled

- Accounts: [Recovery policies — 67 enrolled accounts in order](./data/18-rec-hos-attestation-key-accounts.json)
- New attestation key: `ed25519:6VMtz7uUTrpFZS5tquJGmww6QoxRXGqni4sLN7QRi9vf`
- MPC public keys and timelocks are unchanged

### For reference: DAO proposal creation process

Encode the `set_watchers` args:

```bash
export DATA_FILE='tasks/production/data/18-rec-hos-attestation-key-accounts.json'
export WATCHERS_ARGS='{"watchers":["ed25519:BcRjtC4npBDcRpVY6atmm5pSLwuv3ETV8CCpk47eeVau","ed25519:J8PKegsQptLcux9oeaLKCDETREtJZtfQ9ZDojgKg7MDf","ed25519:7d4A29Rki4XLPVLvmcibJCd1tmWt6Eesznu8dCjJ1kxJ"],"threshold":2}'
echo $WATCHERS_ARGS
export WATCHERS_ARGS_B64=$(printf '%s' "$WATCHERS_ARGS" | base64 | tr -d '\n')
echo $WATCHERS_ARGS_B64
# Expected: eyJ3YXRjaGVycyI6WyJlZDI1NTE5OkJjUmp0QzRucEJEY1JwVlk2YXRtbTVwU0x3dXYzRVRWOENDcGs0N2VlVmF1IiwiZWQyNTUxOTpKOFBLZWdzUXB0TGN1eDlvZWFMS0NERVRSRXRKWnRmUTlaRG9qZ0tnN01EZiIsImVkMjU1MTk6N2Q0QTI5UmtpNFhMUFZMdm1jaWJKQ2QxdG1XdDZFZXN6bnU4ZENqSjFreEoiXSwidGhyZXNob2xkIjoyfQ==
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION=$'PRODUCTION: Rotate a recovery watcher and the recovery attestation key on `rec.hos`\n\nReplaces watcher ed25519:9vbwXrU3jNftszWiy3mABigWT7YkJQdVuiE5kzYU4J7f with ed25519:7d4A29Rki4XLPVLvmcibJCd1tmWt6Eesznu8dCjJ1kxJ. The other two watchers and the threshold of 2 are unchanged.\n\nReinstalls the recovery policy of 67 enrolled accounts with attestation key ed25519:6VMtz7uUTrpFZS5tquJGmww6QoxRXGqni4sLN7QRi9vf. MPC keys and timelocks are unchanged.'
export CONTRACT_ID='rec.hos'
export PROPOSAL_ARGS=$(jq -c --arg description "$PROPOSAL_DESCRIPTION" --arg receiver_id "$CONTRACT_ID" --arg watchers_args "$WATCHERS_ARGS_B64" '{proposal: {description: $description, kind: {FunctionCall: {receiver_id: $receiver_id, actions: ([{method_name: "set_watchers", args: $watchers_args, deposit: "1", gas: "5000000000000"}] + map({method_name: "install_policy", args: (tojson | @base64), deposit: "0", gas: "3000000000000"}))}}}}' $DATA_FILE)
printf '%s' "$PROPOSAL_ARGS" | shasum -a 256
# Expected: 3505d43cb53192ace77d06bfa2c93e1ac249e7504e911709e46fd369fef6bade  -
export PROPOSAL_ARGS_B64=$(printf '%s' "$PROPOSAL_ARGS" | base64 | tr -d '\n')
printf '%s' "$PROPOSAL_ARGS_B64" | shasum -a 256
# Expected: 840b88f65b213b678b846fab57456455637d708c6f4d0e683757e8b1737eb009  -
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 42
# TX ID: https://nearblocks.io/txns/8gEjQdraFQx7quVEKAqqCiYKjsPrd1qcaq6W1iH3s1Pq
```

## Proposal Details

**Proposal ID:** #42

**Description:** PRODUCTION: Rotate a recovery watcher and the recovery attestation key on `rec.hos`

**Expected result:** Once executed, watcher `ed25519:9vbwXrU3jNftszWiy3mABigWT7YkJQdVuiE5kzYU4J7f` on `rec.hos` is replaced with `ed25519:7d4A29Rki4XLPVLvmcibJCd1tmWt6Eesznu8dCjJ1kxJ`, and the attestation key of all 67 enrolled accounts is replaced with `ed25519:6VMtz7uUTrpFZS5tquJGmww6QoxRXGqni4sLN7QRi9vf`.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
export DATA_FILE='tasks/production/data/18-rec-hos-attestation-key-accounts.json'
export PROPOSAL=$(near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 42}' network-config mainnet now 2>/dev/null)
printf '%s' "$PROPOSAL" | jq '.description, .kind.FunctionCall.receiver_id, (.kind.FunctionCall.actions | length)'
# Expected:
# "PRODUCTION: Rotate a recovery watcher and the recovery attestation key on `rec.hos`\n\nReplaces watcher ed25519:9vbwXrU3jNftszWiy3mABigWT7YkJQdVuiE5kzYU4J7f with ed25519:7d4A29Rki4XLPVLvmcibJCd1tmWt6Eesznu8dCjJ1kxJ. The other two watchers and the threshold of 2 are unchanged.\n\nReinstalls the recovery policy of 67 enrolled accounts with attestation key ed25519:6VMtz7uUTrpFZS5tquJGmww6QoxRXGqni4sLN7QRi9vf. MPC keys and timelocks are unchanged."
# "rec.hos"
# 68
```

Decode the `set_watchers` arguments:

```bash
printf '%s' "$PROPOSAL" | jq '.kind.FunctionCall.actions[0] | .method_name, .deposit, .gas, (.args | @base64d | fromjson)'
# Expected:
# "set_watchers"
# "1"
# "5000000000000"
# {
#   "watchers": [
#     "ed25519:BcRjtC4npBDcRpVY6atmm5pSLwuv3ETV8CCpk47eeVau",
#     "ed25519:J8PKegsQptLcux9oeaLKCDETREtJZtfQ9ZDojgKg7MDf",
#     "ed25519:7d4A29Rki4XLPVLvmcibJCd1tmWt6Eesznu8dCjJ1kxJ"
#   ],
#   "threshold": 2
# }
```

Check the current watchers and threshold on `rec.hos`. `ed25519:9vbwXrU3jNftszWiy3mABigWT7YkJQdVuiE5kzYU4J7f` is the watcher being removed:

```bash
near contract call-function as-read-only rec.hos watchers json-args '{}' network-config mainnet now
# Expected: [
#   "ed25519:BcRjtC4npBDcRpVY6atmm5pSLwuv3ETV8CCpk47eeVau",
#   "ed25519:J8PKegsQptLcux9oeaLKCDETREtJZtfQ9ZDojgKg7MDf",
#   "ed25519:9vbwXrU3jNftszWiy3mABigWT7YkJQdVuiE5kzYU4J7f"
# ]
near contract call-function as-read-only rec.hos threshold json-args '{}' network-config mainnet now
# Expected: 2
```

Compare the decoded `install_policy` arguments with the data file (no output means they match, in the same order):

```bash
diff <(printf '%s' "$PROPOSAL" | jq '[.kind.FunctionCall.actions[1:][].args | @base64d | fromjson]') <(jq . $DATA_FILE)
```

### Step 2: Verify Target Contract and Parameters

- [ ] Confirm target contract is `rec.hos`
- [ ] Verify the proposal kind is `FunctionCall` and contains exactly 68 actions
- [ ] Verify action 0 calls `set_watchers` with deposit `1` yoctoNEAR and gas `5000000000000` (5 Tgas)
- [ ] Check the `set_watchers` watchers are `ed25519:BcRjtC4npBDcRpVY6atmm5pSLwuv3ETV8CCpk47eeVau`, `ed25519:J8PKegsQptLcux9oeaLKCDETREtJZtfQ9ZDojgKg7MDf` and `ed25519:7d4A29Rki4XLPVLvmcibJCd1tmWt6Eesznu8dCjJ1kxJ`
- [ ] Check the removed watcher `ed25519:9vbwXrU3jNftszWiy3mABigWT7YkJQdVuiE5kzYU4J7f` is no longer in the list
- [ ] Check the `set_watchers` threshold is `2`
- [ ] Verify actions 1–67 call `install_policy` with deposit `0` and gas `3000000000000` (3 Tgas)
- [ ] Check the decoded `install_policy` arguments match the [data file](./data/18-rec-hos-attestation-key-accounts.json)
- [ ] Check every `attestation_key` is `ed25519:6VMtz7uUTrpFZS5tquJGmww6QoxRXGqni4sLN7QRi9vf`
- [ ] Check the current watchers include `ed25519:9vbwXrU3jNftszWiy3mABigWT7YkJQdVuiE5kzYU4J7f` and the threshold is `2`
- [ ] Check no account is mid-recovery when the proposal executes, since `install_policy` rejects an account that is not idle or in cooldown

### Step 3: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status
- [ ] Check voting requirements
- [ ] Confirm no conflicting pending proposals

## Expected Results

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal kind should be `FunctionCall`
- The proposal target account ID should be `rec.hos`
- The proposal should contain 1 `set_watchers` action followed by 67 `install_policy` actions
- The attestation key in every `install_policy` action should be `ed25519:6VMtz7uUTrpFZS5tquJGmww6QoxRXGqni4sLN7QRi9vf`

## Transaction Links

- Previous task: [Task 17: PRODUCTION: Upgrade voting contract](./17-upgrade-voting-contract.md)
- Proposal creation transaction: https://nearblocks.io/txns/8gEjQdraFQx7quVEKAqqCiYKjsPrd1qcaq6W1iH3s1Pq
