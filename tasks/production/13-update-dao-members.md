# Task 13: PRODUCTION: Update DAO members (remove e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac)

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to update the membership of the **production** DAO (`hos-root.sputnik-dao.near`) Admin group, removing `e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac`.

The change is executed via a `ChangePolicy` proposal that submits the full policy with the updated member list.

Current policy can be viewed using the following command:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_policy json-args {} network-config mainnet now
```

### Members after this change

The Admin group (permissions `*:*`) will contain exactly these 5 accounts:

- `c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816`
- `as.near`
- `root.near`
- `anton-near-one.near`
- `norfolks.near`

The group remains a 3-of-N RoleWeight multi-sig, so after this change it is a 3/5 multi-sig.

### For reference: DAO proposal creation process

Drafting policy.

```bash
export PROPOSAL_ARGS='{
  "proposal": {
    "description": "PRODUCTION: Update DAO policy - remove e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac",
    "kind": {
      "ChangePolicy": {
        "policy": {
          "roles": [
            {
              "name": "Admin",
              "kind": {
                "Group": [
                  "c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816",
                  "as.near",
                  "root.near",
                  "anton-near-one.near",
                  "norfolks.near"
                ]
              },
              "permissions": [
                "*:*"
              ],
              "vote_policy": {}
            }
          ],
          "default_vote_policy": {
            "weight_kind": "RoleWeight",
            "quorum": "0",
            "threshold": "3"
          },
          "proposal_bond": "100000000000000000000000",
          "proposal_period": "604800000000000",
          "bounty_bond": "100000000000000000000000",
          "bounty_forgiveness_period": "604800000000000"
        }
      }
    }
  }
}'
export PROPOSAL_ARGS_B64=$(echo $PROPOSAL_ARGS | base64)
echo $PROPOSAL_ARGS_B64
# Expected: ewogICJwcm9wb3NhbCI6IHsKICAgICJkZXNjcmlwdGlvbiI6ICJQUk9EVUNUSU9OOiBVcGRhdGUgREFPIHBvbGljeSAtIHJlbW92ZSBlOTUzYmI2OWQxMTI5ZTRkYTg3Yjk5NzM5MzczODg0YTBiNTdkNWU2NGE2NWZkYzg2ODQ3OGYyMmU2YzMxZWFjIiwKICAgICJraW5kIjogewogICAgICAiQ2hhbmdlUG9saWN5IjogewogICAgICAgICJwb2xpY3kiOiB7CiAgICAgICAgICAicm9sZXMiOiBbCiAgICAgICAgICAgIHsKICAgICAgICAgICAgICAibmFtZSI6ICJBZG1pbiIsCiAgICAgICAgICAgICAgImtpbmQiOiB7CiAgICAgICAgICAgICAgICAiR3JvdXAiOiBbCiAgICAgICAgICAgICAgICAgICJjNjUyNTUyNTVkNjg5Zjc0YWU0NmIwYTg5ZjA0YmJhYWI5NGQzYTUxYWI5ZGM0Yjc5YjFlOWI2MWU3Y2Y2ODE2IiwKICAgICAgICAgICAgICAgICAgImFzLm5lYXIiLAogICAgICAgICAgICAgICAgICAicm9vdC5uZWFyIiwKICAgICAgICAgICAgICAgICAgImFudG9uLW5lYXItb25lLm5lYXIiLAogICAgICAgICAgICAgICAgICAibm9yZm9sa3MubmVhciIKICAgICAgICAgICAgICAgIF0KICAgICAgICAgICAgICB9LAogICAgICAgICAgICAgICJwZXJtaXNzaW9ucyI6IFsKICAgICAgICAgICAgICAgICIqOioiCiAgICAgICAgICAgICAgXSwKICAgICAgICAgICAgICAidm90ZV9wb2xpY3kiOiB7fQogICAgICAgICAgICB9CiAgICAgICAgICBdLAogICAgICAgICAgImRlZmF1bHRfdm90ZV9wb2xpY3kiOiB7CiAgICAgICAgICAgICJ3ZWlnaHRfa2luZCI6ICJSb2xlV2VpZ2h0IiwKICAgICAgICAgICAgInF1b3J1bSI6ICIwIiwKICAgICAgICAgICAgInRocmVzaG9sZCI6ICIzIgogICAgICAgICAgfSwKICAgICAgICAgICJwcm9wb3NhbF9ib25kIjogIjEwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMCIsCiAgICAgICAgICAicHJvcG9zYWxfcGVyaW9kIjogIjYwNDgwMDAwMDAwMDAwMCIsCiAgICAgICAgICAiYm91bnR5X2JvbmQiOiAiMTAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwKICAgICAgICAgICJib3VudHlfZm9yZ2l2ZW5lc3NfcGVyaW9kIjogIjYwNDgwMDAwMDAwMDAwMCIKICAgICAgICB9CiAgICAgIH0KICAgIH0KICB9Cn0K
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 31
# TX ID: https://nearblocks.io/txns/A3ZdyV6n5qMhtwu2yGMR1QZ7YkKAzcLtLD6seNrPmt4f
```

## Proposal Details

**Proposal ID:** `31`

**Description:** PRODUCTION: Update DAO policy - remove e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac

**Expected result:** Once executed the proposal replaces the DAO policy with an identical policy except that `e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac` is removed from the Admin group. The group becomes a 3/5 RoleWeight multi-sig with the 5 accounts listed above.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 31}' network-config mainnet now
```

### Step 2: Verify Target Contract and Parameters

- [ ] Confirm target DAO from the proposal is `hos-root.sputnik-dao.near` (PRODUCTION)
- [ ] Verify the proposal kind is `ChangePolicy`
- [ ] Verify the Admin group contains exactly the 5 accounts listed above
- [ ] Verify `e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac` is NOT in the group
- [ ] Verify the rest of the policy (permissions, vote policy, bonds, periods) is unchanged from the current policy

## Expected Results

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal kind should be `ChangePolicy`
- The Admin group account IDs should be exactly:
  - `c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816`
  - `as.near`
  - `root.near`
  - `anton-near-one.near`
  - `norfolks.near`

## Transaction Links

- Previous task: [Task 12: PRODUCTION: Update voting contract council](./12-update-voting-contract-council.md)
- Proposal creation transaction: https://nearblocks.io/txns/A3ZdyV6n5qMhtwu2yGMR1QZ7YkKAzcLtLD6seNrPmt4f
