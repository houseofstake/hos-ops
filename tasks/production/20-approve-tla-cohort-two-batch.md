# Task 20: PRODUCTION: Approve TLA Cohort Two batch on `registrar`

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to approve batch `1` on the **production** registrar contract (`registrar`). The batch opens the 506 TLA Cohort Two names.

The proposal will call `approve_batch` on the `registrar` contract with the batch ID and digest listed below.

### Batch being approved

- Names: [TLA Cohort Two — 506 names in order](./data/19-tla-cohort-two-names.txt)
- Batch ID: `1`
- Global code: `gpub.hos`
  - Github commit: https://github.com/houseofstake/tla-contracts/commit/0925e0f56c4abb5be9e7c52e77822488b4503923
  - Code hash: `DavYBFdXPsa9qa6QEyiaUxodV6pax2y8Sp3XoriCNj3t`
- Init args: `{"config":{"registry":"registry.hos","council":"hos-root.sputnik-dao.near","wallet_impl":"impl.hos","hos_extension":"ext.hos","recovery":"rec.hos","chain_id":"mainnet","min_balance":"12000000000000000000000","wallet_timeout_secs":3600}}`
- Funding per account: `30000000000000000000000` yoctoNEAR (0.03 NEAR)
- Digest: `DcSRTWDLMoG3a3Dj1Gvsbd3XxVBSZVbQoriuD9iwfa24`

This task uses `echo -n` to avoid adding a newline before base64 encoding, and `tr -d '\n'` to remove the line breaks GNU `base64` adds on Linux.

### For reference: DAO proposal creation process

Encode inner args (the arguments passed to `approve_batch` on `registrar`):

```bash
export INNER_ARGS='{"batch_id": 1, "digest": "DcSRTWDLMoG3a3Dj1Gvsbd3XxVBSZVbQoriuD9iwfa24"}'
echo $INNER_ARGS
export INNER_ARGS_B64=$(echo -n $INNER_ARGS | base64 | tr -d '\n')
echo $INNER_ARGS_B64
# Expected: eyJiYXRjaF9pZCI6IDEsICJkaWdlc3QiOiAiRGNTUlRXRExNb0czYTNEajFHdnNiZDNYeFZCU1pWYlFvcml1RDlpd2ZhMjQifQ==
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Approve TLA Cohort Two batch on `registrar`'
export CONTRACT_ID='registrar'
export PROPOSAL_ARGS='{"proposal": {"description": "'$PROPOSAL_DESCRIPTION'","kind":{"FunctionCall":{"receiver_id":"'$CONTRACT_ID'","actions":[{"method_name":"approve_batch","args":"'$INNER_ARGS_B64'","deposit":"1","gas":"100000000000000"}]}}}}'
echo $PROPOSAL_ARGS
# Expected: {"proposal": {"description": "PRODUCTION: Approve TLA Cohort Two batch on `registrar`","kind":{"FunctionCall":{"receiver_id":"registrar","actions":[{"method_name":"approve_batch","args":"eyJiYXRjaF9pZCI6IDEsICJkaWdlc3QiOiAiRGNTUlRXRExNb0czYTNEajFHdnNiZDNYeFZCU1pWYlFvcml1RDlpd2ZhMjQifQ==","deposit":"1","gas":"100000000000000"}]}}}}
export PROPOSAL_ARGS_B64=$(echo -n $PROPOSAL_ARGS | base64 | tr -d '\n')
echo $PROPOSAL_ARGS_B64
# Expected: eyJwcm9wb3NhbCI6IHsiZGVzY3JpcHRpb24iOiAiUFJPRFVDVElPTjogQXBwcm92ZSBUTEEgQ29ob3J0IFR3byBiYXRjaCBvbiBgcmVnaXN0cmFyYCIsImtpbmQiOnsiRnVuY3Rpb25DYWxsIjp7InJlY2VpdmVyX2lkIjoicmVnaXN0cmFyIiwiYWN0aW9ucyI6W3sibWV0aG9kX25hbWUiOiJhcHByb3ZlX2JhdGNoIiwiYXJncyI6ImV5SmlZWFJqYUY5cFpDSTZJREVzSUNKa2FXZGxjM1FpT2lBaVJHTlRVbFJYUkV4TmIwY3pZVE5FYWpGSGRuTmlaRE5ZZUZaQ1UxcFdZbEZ2Y21sMVJEbHBkMlpoTWpRaWZRPT0iLCJkZXBvc2l0IjoiMSIsImdhcyI6IjEwMDAwMDAwMDAwMDAwMCJ9XX19fX0=
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 44
# TX ID: https://nearblocks.io/txns/CQvtEkFTxReThmFuTX997mDUfhscySvNm3cETb7RjXqi
```

## Proposal Details

**Proposal ID:** #44

**Description:** PRODUCTION: Approve TLA Cohort Two batch on `registrar`

**Expected result:** Once executed the proposal will call `approve_batch` on the `registrar` contract with batch ID `1` and digest `DcSRTWDLMoG3a3Dj1Gvsbd3XxVBSZVbQoriuD9iwfa24`. If the approval is successful, the batch's `approved` field will be `true`.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 44}' network-config mainnet now
```

Decode the inner arguments:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 44}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
# Expected:
# {
#   "batch_id": 1,
#   "digest": "DcSRTWDLMoG3a3Dj1Gvsbd3XxVBSZVbQoriuD9iwfa24"
# }
```

### Step 2: Verify Target Contract and Parameters

Use the NEAR CLI to retrieve the registrar configuration and batch:

```bash
near contract call-function as-read-only registrar opener_view json-args '{}' network-config mainnet now
# Expected:
# {
#   "admin": "hos-root.sputnik-dao.near",
#   "failed": 0,
#   "live_batches": 1,
#   "next_batch_id": 4,
#   "opened": 502,
#   "operator": "tlaoperator.near",
#   "pending_admin": null,
#   "state_version": 2
# }
near contract call-function as-read-only registrar get_batch json-args '{"batch_id": 1}' network-config mainnet now
# Expected:
# {
#   "approved": false,
#   "count": 506,
#   "digest": "DcSRTWDLMoG3a3Dj1Gvsbd3XxVBSZVbQoriuD9iwfa24",
#   "funding": "30000000000000000000000",
#   "global_code": "gpub.hos",
#   "init_args": "{\"config\":{\"registry\":\"registry.hos\",\"council\":\"hos-root.sputnik-dao.near\",\"wallet_impl\":\"impl.hos\",\"hos_extension\":\"ext.hos\",\"recovery\":\"rec.hos\",\"chain_id\":\"mainnet\",\"min_balance\":\"12000000000000000000000\",\"wallet_timeout_secs\":3600}}",
#   "remaining": 506
# }
```

Hash the code stored under `gpub.hos`:

```bash
curl -s https://rpc.mainnet.near.org -H 'content-type: application/json' -d '{"jsonrpc":"2.0","id":1,"method":"query","params":{"request_type":"view_global_contract_code_by_account_id","account_id":"gpub.hos","finality":"final"}}' | jq -r '.result.code_base64' | base64 -d | sha256sum | awk '{ print $1 }' | xxd -r -p | base58
# Expected: DavYBFdXPsa9qa6QEyiaUxodV6pax2y8Sp3XoriCNj3t
```

Recompute the digest from the expected code, setup arguments and funding, and the names in the data file. This follows `seed_digest` and `fold_digest` in `contracts/registrar-opener/src/names.rs` at `c33edd7`:

```bash
python3 - tasks/production/data/19-tla-cohort-two-names.txt <<'EOF' | xxd -r -p | base58
import hashlib, sys
GLOBAL_CODE = b"gpub.hos"
INIT_ARGS = b'{"config":{"registry":"registry.hos","council":"hos-root.sputnik-dao.near","wallet_impl":"impl.hos","hos_extension":"ext.hos","recovery":"rec.hos","chain_id":"mainnet","min_balance":"12000000000000000000000","wallet_timeout_secs":3600}}'
FUNDING = 30000000000000000000000
seed = b"registrar-opener:batch:v2"
seed += len(GLOBAL_CODE).to_bytes(4, "little") + GLOBAL_CODE
seed += len(INIT_ARGS).to_bytes(4, "little") + INIT_ARGS
seed += FUNDING.to_bytes(16, "little")
d = hashlib.sha256(seed).digest()
for name in open(sys.argv[1]).read().split():
    d = hashlib.sha256(d + name.encode()).digest()
print(d.hex())
EOF
# Expected: DcSRTWDLMoG3a3Dj1Gvsbd3XxVBSZVbQoriuD9iwfa24
```

- [ ] Confirm target contract is `registrar`
- [ ] Verify the proposal kind is `FunctionCall` and contains exactly one action
- [ ] Verify the function being called is `approve_batch`
- [ ] Check the batch ID from the inner arguments is `1`
- [ ] Check the digest from the inner arguments matches the batch digest and expected digest `DcSRTWDLMoG3a3Dj1Gvsbd3XxVBSZVbQoriuD9iwfa24`
- [ ] Check the batch holds exactly the 506 names in the [data file](./data/19-tla-cohort-two-names.txt)
- [ ] Check the batch `global_code` is `gpub.hos`
- [ ] Check the code stored under `gpub.hos` hashes to `DavYBFdXPsa9qa6QEyiaUxodV6pax2y8Sp3XoriCNj3t`
- [ ] Check the batch `init_args` match the [init args above](#batch-being-approved)
- [ ] Check the batch `funding` is `30000000000000000000000` yoctoNEAR (0.03 NEAR)
- [ ] Check the batch `count` is `506`
- [ ] Check the batch `remaining` is `506`
- [ ] Check the action deposit is `1` yoctoNEAR

## Transaction Links

- Previous task: [Task 19: PRODUCTION: Close `far`, update name prices, register TLA Cohort Two and set up business TLAs on `registry.hos`](./19-update-registry-pricing-cohort-two-and-business-tlas.md)
- Proposal creation transaction: https://nearblocks.io/txns/CQvtEkFTxReThmFuTX997mDUfhscySvNm3cETb7RjXqi
