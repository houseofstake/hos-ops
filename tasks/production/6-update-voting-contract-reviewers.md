# Task 6: PRODUCTION: Update voting contract reviewers list

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to update the reviewers list on the **production** voting contract (`vote.dao`) to the agreed set of 5 reviewer accounts.

The voting contract exposes `set_reviewer_ids(reviewer_ids: Vec<AccountId>)` which is `#[payable]`, owner-only, and requires exactly 1 yoctoNEAR. Calling it with the full list replaces the current `config.reviewer_ids`. The DAO (`hos-root.sputnik-dao.near`) is the owner of the voting contract, so the change is executed via a SputnikDAO `FunctionCall` proposal.

### Reviewer accounts being set

- `guiwickb.near`
- `cacossio.near`
- `constantinos.near`
- `arrr_tems.near`
- `e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac`

This task uses `echo -n` to avoid adding a newline before base64 encoding.

### For reference: DAO proposal creation process

Encode inner args (the arguments passed to `set_reviewer_ids` on `vote.dao`):

```bash
export INNER_ARGS='{"reviewer_ids": ["guiwickb.near", "cacossio.near", "constantinos.near", "arrr_tems.near", "e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac"]}'
echo $INNER_ARGS
export INNER_ARGS_B64=$(echo -n $INNER_ARGS | base64)
echo $INNER_ARGS_B64
# Expected: eyJyZXZpZXdlcl9pZHMiOiBbImd1aXdpY2tiLm5lYXIiLCAiY2Fjb3NzaW8ubmVhciIsICJjb25zdGFudGlub3MubmVhciIsICJhcnJyX3RlbXMubmVhciIsICJlOTUzYmI2OWQxMTI5ZTRkYTg3Yjk5NzM5MzczODg0YTBiNTdkNWU2NGE2NWZkYzg2ODQ3OGYyMmU2YzMxZWFjIl19
```

Proposal args:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Update reviewers list on `vote.dao`'
export CONTRACT_ID='vote.dao'
export PROPOSAL_ARGS='{"proposal": {"description": "'$PROPOSAL_DESCRIPTION'","kind":{"FunctionCall":{"receiver_id":"'$CONTRACT_ID'","actions":[{"method_name":"set_reviewer_ids","args":"'$INNER_ARGS_B64'","deposit":"1","gas":"20000000000000"}]}}}}'
echo $PROPOSAL_ARGS
# Expected: {"proposal": {"description": "PRODUCTION: Update reviewers list on `vote.dao`","kind":{"FunctionCall":{"receiver_id":"vote.dao","actions":[{"method_name":"set_reviewer_ids","args":"eyJyZXZpZXdlcl9pZHMiOiBbImd1aXdpY2tiLm5lYXIiLCAiY2Fjb3NzaW8ubmVhciIsICJjb25zdGFudGlub3MubmVhciIsICJhcnJyX3RlbXMubmVhciIsICJlOTUzYmI2OWQxMTI5ZTRkYTg3Yjk5NzM5MzczODg0YTBiNTdkNWU2NGE2NWZkYzg2ODQ3OGYyMmU2YzMxZWFjIl19","deposit":"1","gas":"20000000000000"}]}}}}
export PROPOSAL_ARGS_B64=$(echo -n $PROPOSAL_ARGS | base64)
echo $PROPOSAL_ARGS_B64
# Expected: eyJwcm9wb3NhbCI6IHsiZGVzY3JpcHRpb24iOiAiUFJPRFVDVElPTjogVXBkYXRlIHJldmlld2VycyBsaXN0IG9uIGB2b3RlLmRhb2AiLCJraW5kIjp7IkZ1bmN0aW9uQ2FsbCI6eyJyZWNlaXZlcl9pZCI6InZvdGUuZGFvIiwiYWN0aW9ucyI6W3sibWV0aG9kX25hbWUiOiJzZXRfcmV2aWV3ZXJfaWRzIiwiYXJncyI6ImV5SnlaWFpwWlhkbGNsOXBaSE1pT2lCYkltZDFhWGRwWTJ0aUxtNWxZWElpTENBaVkyRmpiM056YVc4dWJtVmhjaUlzSUNKamIyNXpkR0Z1ZEdsdWIzTXVibVZoY2lJc0lDSmhjbkp5WDNSbGJYTXVibVZoY2lJc0lDSmxPVFV6WW1JMk9XUXhNVEk1WlRSa1lUZzNZams1TnpNNU16Y3pPRGcwWVRCaU5UZGtOV1UyTkdFMk5XWmtZemcyT0RRM09HWXlNbVUyWXpNeFpXRmpJbDE5IiwiZGVwb3NpdCI6IjEiLCJnYXMiOiIyMDAwMDAwMDAwMDAwMCJ9XX19fX0=
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 22
# TX ID: https://nearblocks.io/ru/txns/5JJJo2DELMS4oEBEM8YmkYfrqTPvWrfj5Cp1r7YaonDc
```

## Proposal Details

**Proposal ID:** `22`

**Description:** PRODUCTION: Update reviewers list on `vote.dao`

**Expected result:** Once executed the proposal will call `set_reviewer_ids` on the `vote.dao` contract, replacing `config.reviewer_ids` with the 5 accounts listed above. After execution, only those accounts will be able to call reviewer-only methods (e.g. `approve` / `reject`) on the production voting contract.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": <PROPOSAL_ID>}' network-config mainnet now
```

### Step 2: Decode and Verify Arguments

Decode the inner arguments to verify the actual reviewer list:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": <PROPOSAL_ID>}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

Expected decoded inner args:

```json
{
  "reviewer_ids": [
    "guiwickb.near",
    "cacossio.near",
    "constantinos.near",
    "arrr_tems.near",
    "e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac"
  ]
}
```

### Step 3: Verify Target Contract and Parameters

- [ ] Confirm target contract from the proposal is `vote.dao`
- [ ] Verify the proposal kind is `FunctionCall`
- [ ] Verify the method being called is `set_reviewer_ids`
- [ ] Verify the inner `reviewer_ids` array contains exactly the 5 accounts above, in the same order, with no extras or omissions
- [ ] Verify the action `deposit` is `"1"` (1 yoctoNEAR — required by `assert_one_yocto`)
- [ ] Verify the action `gas` is `"20000000000000"` (20 Tgas)

### Step 4: Additional Checks

- [ ] Review the proposer account (`norfolks.near`)
- [ ] Verify the proposal status is `InProgress`
- [ ] Check voting requirements / DAO policy thresholds
- [ ] Confirm no conflicting pending proposals on `vote.dao`

## Expected Results

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal kind should be `FunctionCall`
- The proposal target account ID should be `vote.dao`
- The method name should be `set_reviewer_ids`
- The decoded inner args should contain `reviewer_ids` equal to:
  - `guiwickb.near`
  - `cacossio.near`
  - `constantinos.near`
  - `arrr_tems.near`
  - `e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac`
- The deposit should be `1` (yoctoNEAR)
- The gas should be `20000000000000` (20 Tgas)

## Transaction Links

- Previous task: [Task 5: PRODUCTION: Upgrade voting contract](./5-upgrade-voting-contract.md)
- Proposal creation transaction: https://nearblocks.io/ru/txns/5JJJo2DELMS4oEBEM8YmkYfrqTPvWrfj5Cp1r7YaonDc

## Notes

- `set_reviewer_ids` replaces the entire reviewer list — make sure no current reviewer that needs to remain is missing from the list above before voting to approve.
- The voting contract must already be running a version that exposes `set_reviewer_ids` (introduced/kept in `governance.rs`). Production was upgraded to `1.0.3` in task 5, which includes this method.
