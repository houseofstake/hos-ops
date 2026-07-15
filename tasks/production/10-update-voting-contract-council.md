# Task 10: PRODUCTION: Update voting contract council (replace fastnear-hos.near with anton-near-one.near)

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to update the council list on the **production** voting contract (`vote.dao`), replacing `fastnear-hos.near` with `anton-near-one.near`.

### Council accounts being set

The council will contain exactly these 6 accounts:

- `as.near`
- `c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816`
- `e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac`
- `anton-near-one.near`
- `root.near`
- `norfolks.near`

Current council can be viewed with:

```bash
near contract call-function as-read-only vote.dao get_config json-args {} network-config mainnet now
```

### For reference: DAO proposal creation process

Encode inner args (the arguments passed to `set_council_ids` on `vote.dao`):

```bash
export INNER_ARGS='{"council_ids": ["as.near", "c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816", "e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac", "anton-near-one.near", "root.near", "norfolks.near"]}'
echo $INNER_ARGS
export INNER_ARGS_B64=$(echo -n $INNER_ARGS | base64)
echo $INNER_ARGS_B64
# Expected: eyJjb3VuY2lsX2lkcyI6IFsiYXMubmVhciIsICJjNjUyNTUyNTVkNjg5Zjc0YWU0NmIwYTg5ZjA0YmJhYWI5NGQzYTUxYWI5ZGM0Yjc5YjFlOWI2MWU3Y2Y2ODE2IiwgImU5NTNiYjY5ZDExMjllNGRhODdiOTk3MzkzNzM4ODRhMGI1N2Q1ZTY0YTY1ZmRjODY4NDc4ZjIyZTZjMzFlYWMiLCAiYW50b24tbmVhci1vbmUubmVhciIsICJyb290Lm5lYXIiLCAibm9yZm9sa3MubmVhciJdfQ==
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Update council list on `vote.dao`'
export CONTRACT_ID='vote.dao'
export PROPOSAL_ARGS='{"proposal": {"description": "'$PROPOSAL_DESCRIPTION'","kind":{"FunctionCall":{"receiver_id":"'$CONTRACT_ID'","actions":[{"method_name":"set_council_ids","args":"'$INNER_ARGS_B64'","deposit":"1","gas":"20000000000000"}]}}}}'
echo $PROPOSAL_ARGS
# Expected: {"proposal": {"description": "PRODUCTION: Update council list on `vote.dao`","kind":{"FunctionCall":{"receiver_id":"vote.dao","actions":[{"method_name":"set_council_ids","args":"eyJjb3VuY2lsX2lkcyI6IFsiYXMubmVhciIsICJjNjUyNTUyNTVkNjg5Zjc0YWU0NmIwYTg5ZjA0YmJhYWI5NGQzYTUxYWI5ZGM0Yjc5YjFlOWI2MWU3Y2Y2ODE2IiwgImU5NTNiYjY5ZDExMjllNGRhODdiOTk3MzkzNzM4ODRhMGI1N2Q1ZTY0YTY1ZmRjODY4NDc4ZjIyZTZjMzFlYWMiLCAiYW50b24tbmVhci1vbmUubmVhciIsICJyb290Lm5lYXIiLCAibm9yZm9sa3MubmVhciJdfQ==","deposit":"1","gas":"20000000000000"}]}}}}
export PROPOSAL_ARGS_B64=$(echo -n $PROPOSAL_ARGS | base64)
echo $PROPOSAL_ARGS_B64
# Expected: eyJwcm9wb3NhbCI6IHsiZGVzY3JpcHRpb24iOiAiUFJPRFVDVElPTjogVXBkYXRlIGNvdW5jaWwgbGlzdCBvbiBgdm90ZS5kYW9gIiwia2luZCI6eyJGdW5jdGlvbkNhbGwiOnsicmVjZWl2ZXJfaWQiOiJ2b3RlLmRhbyIsImFjdGlvbnMiOlt7Im1ldGhvZF9uYW1lIjoic2V0X2NvdW5jaWxfaWRzIiwiYXJncyI6ImV5SmpiM1Z1WTJsc1gybGtjeUk2SUZzaVlYTXVibVZoY2lJc0lDSmpOalV5TlRVeU5UVmtOamc1WmpjMFlXVTBObUl3WVRnNVpqQTBZbUpoWVdJNU5HUXpZVFV4WVdJNVpHTTBZamM1WWpGbE9XSTJNV1UzWTJZMk9ERTJJaXdnSW1VNU5UTmlZalk1WkRFeE1qbGxOR1JoT0RkaU9UazNNemt6TnpNNE9EUmhNR0kxTjJRMVpUWTBZVFkxWm1Sak9EWTRORGM0WmpJeVpUWmpNekZsWVdNaUxDQWlZVzUwYjI0dGJtVmhjaTF2Ym1VdWJtVmhjaUlzSUNKeWIyOTBMbTVsWVhJaUxDQWlibTl5Wm05c2EzTXVibVZoY2lKZGZRPT0iLCJkZXBvc2l0IjoiMSIsImdhcyI6IjIwMDAwMDAwMDAwMDAwIn1dfX19fQ==
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 28
# TX ID: https://nearblocks.io/txns/EdN4P8rk1sJQob7JmApehnzYVtkWidbrQ1De5SJzgZmL
```

## Proposal Details

**Proposal ID:** `28`

**Description:** PRODUCTION: Update council list on `vote.dao`

**Expected result:** Once executed the proposal will call `set_council_ids` on the `vote.dao` contract, replacing `config.council_ids` with the 6 accounts listed above (`fastnear-hos.near` removed, `anton-near-one.near` added).

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 28}' network-config mainnet now
```

### Step 2: Decode and Verify Arguments

Decode the inner arguments to verify the actual council list:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 28}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

Expected decoded inner args:

```json
{
  "council_ids": [
    "as.near",
    "c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816",
    "e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac",
    "anton-near-one.near",
    "root.near",
    "norfolks.near"
  ]
}
```

### Step 3: Verify Target Contract and Parameters

- [ ] Confirm target contract from the proposal is `vote.dao`
- [ ] Verify the proposal kind is `FunctionCall`
- [ ] Verify the method being called is `set_council_ids`
- [ ] Verify the inner `council_ids` array contains exactly the 6 accounts above
- [ ] Verify the action `deposit` is `"1"` (1 yoctoNEAR) and `gas` is `"20000000000000"` (20 Tgas)

## Expected Results

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal kind should be `FunctionCall`
- The proposal target account ID should be `vote.dao`
- The method name should be `set_council_ids`
- The decoded `council_ids` should be exactly:
  - `as.near`
  - `c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816`
  - `e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac`
  - `anton-near-one.near`
  - `root.near`
  - `norfolks.near`

## Transaction Links

- Previous task: [Task 9: PRODUCTION: Update DAO members](./9-update-dao-members.md)
- Proposal creation transaction: https://nearblocks.io/txns/EdN4P8rk1sJQob7JmApehnzYVtkWidbrQ1De5SJzgZmL
