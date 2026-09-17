# Task 15: PRODUCTION: Approve TLA Cohort One batch

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to approve the TLA Cohort One batch on the **production** registrar contract (`registrar`) for the agreed set of 502 top-level account names.

The proposal will call `approve_batch` on the `registrar` contract with the batch ID and digest listed below.

Links:
- Commit: `cf0697a8cce35a5b0ee3126f20771ad40a2ae444`
- Github commit: https://github.com/houseofstake/registrar-opener/commit/cf0697a8cce35a5b0ee3126f20771ad40a2ae444

### Batch being approved

- Names: [TLA Cohort One — 502 names in order](./data/15-tla-cohort-one-names.txt)
- Batch ID: `0`
- Owner key: `ed25519:GNZKvys1PwJ78fqJ2AwQq6woMnUwRxdGQLDJEMjwKK95`
- Funding per account: `30000000000000000000000` yoctoNEAR (0.03 NEAR)
- Digest: `4PbPG4XVTwLsJzQN86VA6xst7rcFcosVKEYLW7Kq2b5u`

This task uses `echo -n` to avoid adding a newline before base64 encoding.

### For reference: DAO proposal creation process

Encode inner args (the arguments passed to `approve_batch` on `registrar`):

```bash
export INNER_ARGS='{"batch_id": 0, "digest": "4PbPG4XVTwLsJzQN86VA6xst7rcFcosVKEYLW7Kq2b5u"}'
echo $INNER_ARGS
export INNER_ARGS_B64=$(echo -n $INNER_ARGS | base64)
echo $INNER_ARGS_B64
# Expected: eyJiYXRjaF9pZCI6IDAsICJkaWdlc3QiOiAiNFBiUEc0WFZUd0xzSnpRTjg2VkE2eHN0N3JjRmNvc1ZLRVlMVzdLcTJiNXUifQ==
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Approve TLA Cohort One batch on `registrar`'
export CONTRACT_ID='registrar'
export PROPOSAL_ARGS='{"proposal": {"description": "'$PROPOSAL_DESCRIPTION'","kind":{"FunctionCall":{"receiver_id":"'$CONTRACT_ID'","actions":[{"method_name":"approve_batch","args":"'$INNER_ARGS_B64'","deposit":"1","gas":"100000000000000"}]}}}}'
echo $PROPOSAL_ARGS
# Expected: {"proposal": {"description": "PRODUCTION: Approve TLA Cohort One batch on `registrar`","kind":{"FunctionCall":{"receiver_id":"registrar","actions":[{"method_name":"approve_batch","args":"eyJiYXRjaF9pZCI6IDAsICJkaWdlc3QiOiAiNFBiUEc0WFZUd0xzSnpRTjg2VkE2eHN0N3JjRmNvc1ZLRVlMVzdLcTJiNXUifQ==","deposit":"1","gas":"100000000000000"}]}}}}
export PROPOSAL_ARGS_B64=$(echo -n $PROPOSAL_ARGS | base64)
echo $PROPOSAL_ARGS_B64
# Expected: eyJwcm9wb3NhbCI6IHsiZGVzY3JpcHRpb24iOiAiUFJPRFVDVElPTjogQXBwcm92ZSBUTEEgQ29ob3J0IE9uZSBiYXRjaCBvbiBgcmVnaXN0cmFyYCIsImtpbmQiOnsiRnVuY3Rpb25DYWxsIjp7InJlY2VpdmVyX2lkIjoicmVnaXN0cmFyIiwiYWN0aW9ucyI6W3sibWV0aG9kX25hbWUiOiJhcHByb3ZlX2JhdGNoIiwiYXJncyI6ImV5SmlZWFJqYUY5cFpDSTZJREFzSUNKa2FXZGxjM1FpT2lBaU5GQmlVRWMwV0ZaVWQweHpTbnBSVGpnMlZrRTJlSE4wTjNKalJtTnZjMVpMUlZsTVZ6ZExjVEppTlhVaWZRPT0iLCJkZXBvc2l0IjoiMSIsImdhcyI6IjEwMDAwMDAwMDAwMDAwMCJ9XX19fX0=
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 38
# TX ID: https://explorer.near.org/transactions/2ygjE4ce99WGAQLHdcd9Xm9vevb1LVCMkEhsTSM8rjnW
```

## Proposal Details

**Proposal ID:** #38

**Description:** PRODUCTION: Approve TLA Cohort One batch on `registrar`

**Expected result:** Once executed the proposal will call `approve_batch` on the `registrar` contract with batch ID `0` and digest `4PbPG4XVTwLsJzQN86VA6xst7rcFcosVKEYLW7Kq2b5u`. If the approval is successful, the batch's `approved` field will be `true`.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 38}' network-config mainnet now
```

Decode the inner arguments:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 38}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

### Step 2: Verify Target Contract and Parameters

Use the NEAR CLI to retrieve the registrar configuration and batch:

```bash
near contract call-function as-read-only registrar opener_view json-args '{}' network-config mainnet now
near contract call-function as-read-only registrar get_batch json-args '{"batch_id": 0}' network-config mainnet now
```

- [ ] Confirm target contract is `registrar`
- [ ] Verify the proposal kind is `FunctionCall` and contains exactly one action
- [ ] Verify the function being called is `approve_batch`
- [ ] Check the batch ID from the inner arguments is `0`
- [ ] Check the digest from the inner arguments matches the batch digest and expected digest `4PbPG4XVTwLsJzQN86VA6xst7rcFcosVKEYLW7Kq2b5u`
- [ ] Check the batch `owner_key` is `ed25519:GNZKvys1PwJ78fqJ2AwQq6woMnUwRxdGQLDJEMjwKK95`
- [ ] Check the batch `funding` is `30000000000000000000000` yoctoNEAR (0.03 NEAR)
- [ ] Check the batch `approved` is `false`
- [ ] Check the batch `count` is `502`
- [ ] Check the batch `remaining` is `502`
- [ ] Check the action deposit is `1` yoctoNEAR
- [ ] Check the action gas is `100000000000000` (100 Tgas)
- [ ] Verify the registrar admin is `hos-root.sputnik-dao.near`
- [ ] If PRODUCTION, double-check proposal description for any STAGING indicators

## Transaction Links

- Previous task: [Task 14: PRODUCTION: Create recovery campaign](./14-create-rewards-campaign.md)
- Proposal creation transaction: https://explorer.near.org/transactions/2ygjE4ce99WGAQLHdcd9Xm9vevb1LVCMkEhsTSM8rjnW
