# Task 14: PRODUCTION: Create recovery campaign on `rewards.hos-dao.near` (claim end 15 October 2026)

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

A residual balance is left on the merkle claim contract `rewards.hos-dao.near` after the earlier reward campaigns.
The `withdraw` method is unavailable, so a recovery campaign is needed for `houseofstake.sputnik-dao.near`. This task covers creating and verifying that campaign.

The tree contains two leaves of 49,415 NEAR each. The `address` field identifies the account that executes the claim:
`norfolks.near` or `houseofstake.sputnik-dao.near`. The `lockup` field identifies the recipient of the funds:
`houseofstake.sputnik-dao.near` in both leaves. Only one claim is intended; the alternative executor is included
in case one route cannot be used.

The campaign is published by calling `create_campaign` on the claims contract. That contract only accepts owner calls. The call is wrapped in a `FunctionCall` proposal on the owner DAO `rewards-claims2.sputnik-dao.near`. Per the DAO
policy the `call` vote policy threshold is `1`, a single SecurityCouncil member approval executes the campaign.

### Merkle tree generation

The tree was generated from the following CSV using the
[Agora Campaign Admin Application](https://near-claim.vercel.app/campaigns/new).

```csv
address,lockup,amount
norfolks.near,houseofstake.sputnik-dao.near,49415000000000000000000000000
houseofstake.sputnik-dao.near,houseofstake.sputnik-dao.near,49415000000000000000000000000
```

### Campaign parameters

| Parameter | Value |
|-----------|-------|
| Claim end (human) | 2026-10-15 00:00:00 UTC |
| Claim end (ns) | `1792022400000000000` |
| Merkle root (bytes) | `[28,11,242,131,71,162,153,235,78,64,69,121,131,183,22,31,146,211,233,230,162,248,16,118,131,74,79,192,254,153,70,156]` |

### DAO proposal creation

Submit a `FunctionCall` proposal on `rewards-claims2.sputnik-dao.near` that calls `create_campaign` on
`rewards.hos-dao.near` with the merkle root and claim end above, have one SecurityCouncil member approve it, and verify the created campaign.

Build the inner arguments:

```bash
export DAO_ACCOUNT_ID="rewards-claims2.sputnik-dao.near"
export CLAIMS_CONTRACT="rewards.hos-dao.near"

# 2026-10-15 00:00:00 UTC in nanoseconds
export CLAIM_END="1792022400000000000"

# Merkle root as byte array
export MERKLE_ROOT='[28,11,242,131,71,162,153,235,78,64,69,121,131,183,22,31,146,211,233,230,162,248,16,118,131,74,79,192,254,153,70,156]'

export INNER_ARGS=$(echo -n '{"merkle_root":'"$MERKLE_ROOT"',"claim_end":"'"$CLAIM_END"'"}' | base64 | tr -d '\n')
echo $INNER_ARGS
# Expected: eyJtZXJrbGVfcm9vdCI6WzI4LDExLDI0MiwxMzEsNzEsMTYyLDE1MywyMzUsNzgsNjQsNjksMTIxLDEzMSwxODMsMjIsMzEsMTQ2LDIxMSwyMzMsMjMwLDE2MiwyNDgsMTYsMTE4LDEzMSw3NCw3OSwxOTIsMjU0LDE1Myw3MCwxNTZdLCJjbGFpbV9lbmQiOiIxNzkyMDIyNDAwMDAwMDAwMDAwIn0=
```

CLI command to create the proposal:

```bash
export SIGNER_ACCOUNT_ID="[YOUR_SC_ACCOUNT]"

near contract call-function as-transaction $DAO_ACCOUNT_ID add_proposal json-args '{
  "proposal": {
    "description": "PRODUCTION: Create recovery campaign on rewards.hos-dao.near for houseofstake.sputnik-dao.near, claim end 2026-10-15, merkle root 0x1c0bf28347a299eb4e40457983b7161f92d3e9e6a2f81076834a4fc0fe99469c",
    "kind": {
      "FunctionCall": {
        "receiver_id": "'"$CLAIMS_CONTRACT"'",
        "actions": [{
          "method_name": "create_campaign",
          "args": "'"$INNER_ARGS"'",
          "deposit": "0",
          "gas": "50000000000000"
        }]
      }
    }
  }
}' prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
# Expected proposal ID: 7.
```

## Proposal Details

**Proposal ID:** `7`

**Description:** PRODUCTION: Create recovery campaign on `rewards.hos-dao.near` claim end 2026-10-15, merkle root
`0x1c0bf28347a299eb4e40457983b7161f92d3e9e6a2f81076834a4fc0fe99469c`

**Expected result:** Once executed, `rewards.hos-dao.near` holds a new campaign with the merkle root above and a claim
window closing at 2026-10-15 00:00:00 UTC. The tree includes the two addresses listed above,
with 49,415 NEAR per leaf and only one claim intended.

## Verification Steps

### Step 1: Check the Proposal

Retrieve the proposal and decode its inner arguments:

```bash
near contract call-function as-read-only rewards-claims2.sputnik-dao.near get_proposal json-args '{"id": 7}' network-config mainnet now

near contract call-function as-read-only rewards-claims2.sputnik-dao.near get_proposal json-args '{"id": 7}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

### Step 2: Verify Target Contract and Parameters

- [ ] **CRITICAL**: Confirm target contract is `rewards.hos-dao.near`
- [ ] Verify the proposal kind is `FunctionCall`
- [ ] Verify the method being called is `create_campaign`
- [ ] Verify the `merkle_root` byte array is `[28,11,242,131,71,162,153,235,78,64,69,121,131,183,22,31,146,211,233,230,162,248,16,118,131,74,79,192,254,153,70,156]`
- [ ] Verify `claim_end` is `1792022400000000000` (2026-10-15 00:00:00 UTC)
- [ ] Verify `deposit` is `"0"` and `gas` is `50000000000000` (50 Tgas)
- [ ] Verify the tree contains exactly the two allocations listed in the CSV above and no others

### Step 3: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status is `InProgress`

### Step 4: Approve the Proposal

One SecurityCouncil member approval is enough (the `call` vote policy threshold is `1`).

```bash
export SIGNER_ACCOUNT_ID="[YOUR_SC_ACCOUNT]"

export PROPOSAL_KIND=$(near contract call-function as-read-only rewards-claims2.sputnik-dao.near get_proposal json-args '{"id": 7}' network-config mainnet now | jq -c '.kind')

near contract call-function as-transaction rewards-claims2.sputnik-dao.near act_proposal json-args '{
  "id": 7,
  "action": "VoteApprove",
  "proposal": '"$PROPOSAL_KIND"'
}' prepaid-gas '200.0 Tgas' attached-deposit '0 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
```

### Step 5: Confirm the Campaign On-Chain

```bash
near contract call-function as-read-only rewards.hos-dao.near get_campaign json-args '{"campaign_id": 6}' network-config mainnet now
```

- [ ] The campaign exists with the merkle root above
- [ ] Verify `claim_end` is `1792022400000000000`

## Expected Results

- The DAO account should be `rewards-claims2.sputnik-dao.near`
- The proposal kind should be `FunctionCall` targeting `rewards.hos-dao.near`
- The method should be `create_campaign` with `deposit` `"0"` and `gas` `50000000000000`
- The decoded merkle root should be `[28,11,242,131,71,162,153,235,78,64,69,121,131,183,22,31,146,211,233,230,162,248,16,118,131,74,79,192,254,153,70,156]`
- The decoded `claim_end` should be `1792022400000000000` (2026-10-15 00:00:00 UTC)
- After approval, a new campaign is readable from `rewards.hos-dao.near`

## Transaction Links

- Previous task: [Task 13: PRODUCTION: Update DAO members](./13-update-dao-members.md)
- Campaign process reference: [Task 4: NEAR Rewards Merkle Verification](./4-near-rewards-merkle-verification.md)
- Proposal creation transaction: [TBD]
- Approval transaction: [TBD]
