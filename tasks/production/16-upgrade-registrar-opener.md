# Task 16: PRODUCTION: Upgrade opener contract on `registrar`

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to upgrade the opener contract on the **production** registrar account (`registrar`) to commit `c33edd7`. The running code is `cf0697a8`, the same code used in [Task 15](./15-approve-tla-cohort-one-batch.md).

After this upgrade, both batch creation and the council's single-name `create_account` path attach shared contract code and run its setup in the same transaction, without adding a key. The shared-code account and setup arguments are supplied by the operator; using `gpub.hos` must be checked in each council-approved batch or single-name call.

Links:
- Commit: `c33edd7a3495378c5503fbe5e56e814ae9ce667f`
- Github commit: https://github.com/houseofstake/tla-contracts/commit/c33edd7a3495378c5503fbe5e56e814ae9ce667f
- The Github changes: https://github.com/houseofstake/tla-contracts/compare/3b9f3dbed4021a625f03770e8c8acbc283366080...c33edd7a3495378c5503fbe5e56e814ae9ce667f

The opener moved from `registrar-opener` into `tla-contracts` in `3b9f3db` with no contract code changes, so the compare above shows every code change against the running contract.

### Notable changes since `cf0697a8`

- A batch names the shared code account and the setup arguments instead of an owner key, and both are part of the digest the council approves
- The council's `create_account` now takes `name`, `global_code` and `init_args` instead of `name` and `owner_key`, and uses the same keyless creation and setup checks as a batch
- Each name is created, funded, attached to that code and set up in one transaction, with no key added
- One `open_names` call takes up to 11 names instead of 20, since each name also runs its setup

This task uses `echo -n` to avoid adding a newline before base64 encoding.

### For reference: DAO proposal creation process

Rebuilding the contract

```bash
git clone https://github.com/houseofstake/tla-contracts.git
cd tla-contracts
git checkout c33edd7a3495378c5503fbe5e56e814ae9ce667f
cd contracts/registrar-opener
cargo near build reproducible-wasm
cd ../..
```

The opener binary located at `target/near/registrar_opener/registrar_opener.wasm`
```bash
export WASM=target/near/registrar_opener/registrar_opener.wasm
export CONTRACT_HASH=$(cat $WASM | sha256sum | awk '{ print $1 }' | xxd -r -p | base58)
echo $CONTRACT_HASH
# Expected: DGpi3vGXeiJJ6gps8t5CDcDr5DbuzWXTMc6CVWcdWCXt
ls -l $WASM
# Expected size: 218033 bytes
```

Encode inner args (the arguments passed to `upgrade` on `registrar`):

```bash
unset INNER_ARGS_B64 PROPOSAL_ARGS
INNER_ARGS_B64=$({ printf '{"code":"'; base64 < "$WASM" | tr -d '\n'; printf '"}'; } | base64 | tr -d '\n')
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Upgrade opener contract on `registrar` to `c33edd7`'
export CONTRACT_ID='registrar'
PROPOSAL_ARGS=$(jq -c -n --arg description "$PROPOSAL_DESCRIPTION" --arg receiver "$CONTRACT_ID" --arg args "$INNER_ARGS_B64" \
  '{proposal: {description: $description, kind: {FunctionCall: {receiver_id: $receiver, actions: [{method_name: "upgrade", args: $args, deposit: "1", gas: "100000000000000"}]}}}}')
printf '%s' "$PROPOSAL_ARGS" | sha256sum
# Expected: a840a7227c2017268c7e585f926a23c88735c81ce1ab0134445da3b82b88d05f
printf '%s' "$PROPOSAL_ARGS" | wc -c
# Expected size: 387864 bytes
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal json-args "$PROPOSAL_ARGS" prepaid-gas '300.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 39
# TX ID: https://nearblocks.io/txns/W3GvocrHzC6UisVs19cRhKxTTvsTuSYQK3CF72qu5TY
```

## Proposal Details

**Proposal ID:** #39

**Description:** PRODUCTION: Upgrade opener contract on `registrar` to `c33edd7`

**Expected result:** Once executed the proposal will call `upgrade` on the `registrar` contract with the new contract binary with hash `DGpi3vGXeiJJ6gps8t5CDcDr5DbuzWXTMc6CVWcdWCXt`. If the upgrade is successful, `registrar` runs the new code and `opener_view` shows `state_version` `2`.

## Verification Steps

> **ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal. It carries the whole binary, so filter the output:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 39}' network-config mainnet now | jq '{description, status, kind: {receiver_id: .kind.FunctionCall.receiver_id, actions: [.kind.FunctionCall.actions[] | {method_name, deposit, gas}]}}'
```

Hash the code inside the inner arguments:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 39}' network-config mainnet now | jq -r '.kind.FunctionCall.actions[0].args | @base64d | fromjson | .code' | base64 -d | sha256sum | awk '{ print $1 }' | xxd -r -p | base58
# Expected: DGpi3vGXeiJJ6gps8t5CDcDr5DbuzWXTMc6CVWcdWCXt
```

### Step 2: Verify Target Contract and Parameters
- [ ] Confirm target contract is `registrar`
- [ ] Verify the proposal kind is `FunctionCall` and contains exactly one action
- [ ] Verify the function being called is `upgrade`
- [ ] Build the release wasm from commit `c33edd7a3495378c5503fbe5e56e814ae9ce667f` and check its hash is `DGpi3vGXeiJJ6gps8t5CDcDr5DbuzWXTMc6CVWcdWCXt`
- [ ] Check the code in the inner arguments hashes to `DGpi3vGXeiJJ6gps8t5CDcDr5DbuzWXTMc6CVWcdWCXt`
- [ ] Check the action deposit is `1` yoctoNEAR
- [ ] Check the action gas is `100000000000000` (100 Tgas)

## Transaction Links

- Previous task: [Task 15: PRODUCTION: Approve TLA Cohort One batch](./15-approve-tla-cohort-one-batch.md)
- Proposal creation transaction: https://nearblocks.io/txns/W3GvocrHzC6UisVs19cRhKxTTvsTuSYQK3CF72qu5TY
