# Task 9: PRODUCTION: Update DAO members (replace fastnear-hos.near with anton-near-one.near)

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to update the membership of the **production** DAO (`hos-root.sputnik-dao.near`) Admin group, replacing `fastnear-hos.near` with `anton-near-one.near`.

The change is executed via a `ChangePolicy` proposal that submits the full policy with the updated member list.

Current policy can be viewed using the following command:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_policy json-args {} network-config mainnet now
```

### Members after this change

The Admin group (permissions `*:*`) will contain exactly these 6 accounts:

- `c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816`
- `as.near`
- `root.near`
- `e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac`
- `anton-near-one.near`
- `norfolks.near`

### For reference: DAO proposal creation process

Drafting policy.

```bash
export PROPOSAL_ARGS='{
  "proposal": {
    "description": "PRODUCTION: Update DAO policy - replace fastnear-hos.near with anton-near-one.near",
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
                  "e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac",
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
# Expected: ewogICJwcm9wb3NhbCI6IHsKICAgICJkZXNjcmlwdGlvbiI6ICJQUk9EVUNUSU9OOiBVcGRhdGUgREFPIHBvbGljeSAtIHJlcGxhY2UgZmFzdG5lYXItaG9zLm5lYXIgd2l0aCBhbnRvbi1uZWFyLW9uZS5uZWFyIiwKICAgICJraW5kIjogewogICAgICAiQ2hhbmdlUG9saWN5IjogewogICAgICAgICJwb2xpY3kiOiB7CiAgICAgICAgICAicm9sZXMiOiBbCiAgICAgICAgICAgIHsKICAgICAgICAgICAgICAibmFtZSI6ICJBZG1pbiIsCiAgICAgICAgICAgICAgImtpbmQiOiB7CiAgICAgICAgICAgICAgICAiR3JvdXAiOiBbCiAgICAgICAgICAgICAgICAgICJjNjUyNTUyNTVkNjg5Zjc0YWU0NmIwYTg5ZjA0YmJhYWI5NGQzYTUxYWI5ZGM0Yjc5YjFlOWI2MWU3Y2Y2ODE2IiwKICAgICAgICAgICAgICAgICAgImFzLm5lYXIiLAogICAgICAgICAgICAgICAgICAicm9vdC5uZWFyIiwKICAgICAgICAgICAgICAgICAgImU5NTNiYjY5ZDExMjllNGRhODdiOTk3MzkzNzM4ODRhMGI1N2Q1ZTY0YTY1ZmRjODY4NDc4ZjIyZTZjMzFlYWMiLAogICAgICAgICAgICAgICAgICAiYW50b24tbmVhci1vbmUubmVhciIsCiAgICAgICAgICAgICAgICAgICJub3Jmb2xrcy5uZWFyIgogICAgICAgICAgICAgICAgXQogICAgICAgICAgICAgIH0sCiAgICAgICAgICAgICAgInBlcm1pc3Npb25zIjogWwogICAgICAgICAgICAgICAgIio6KiIKICAgICAgICAgICAgICBdLAogICAgICAgICAgICAgICJ2b3RlX3BvbGljeSI6IHt9CiAgICAgICAgICAgIH0KICAgICAgICAgIF0sCiAgICAgICAgICAiZGVmYXVsdF92b3RlX3BvbGljeSI6IHsKICAgICAgICAgICAgIndlaWdodF9raW5kIjogIlJvbGVXZWlnaHQiLAogICAgICAgICAgICAicXVvcnVtIjogIjAiLAogICAgICAgICAgICAidGhyZXNob2xkIjogIjMiCiAgICAgICAgICB9LAogICAgICAgICAgInByb3Bvc2FsX2JvbmQiOiAiMTAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwKICAgICAgICAgICJwcm9wb3NhbF9wZXJpb2QiOiAiNjA0ODAwMDAwMDAwMDAwIiwKICAgICAgICAgICJib3VudHlfYm9uZCI6ICIxMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLAogICAgICAgICAgImJvdW50eV9mb3JnaXZlbmVzc19wZXJpb2QiOiAiNjA0ODAwMDAwMDAwMDAwIgogICAgICAgIH0KICAgICAgfQogICAgfQogIH0KfQo=
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 27
# TX ID: https://nearblocks.io/txns/DYBEk8zqtnQyVfxgjJ5uKzmbyupw9TbpmmZukD64vNPr
```

## Proposal Details

**Proposal ID:** `27`

**Description:** PRODUCTION: Update DAO policy - replace fastnear-hos.near with anton-near-one.near

**Expected result:** Once executed the proposal replaces the DAO policy with an identical policy except that `fastnear-hos.near` is removed from the Admin group and `anton-near-one.near` is added. The group remains a 3/6 RoleWeight multi-sig with the 6 accounts listed above.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 27}' network-config mainnet now
```

### Step 2: Verify Target Contract and Parameters

- [ ] Confirm target DAO from the proposal is `hos-root.sputnik-dao.near` (PRODUCTION)
- [ ] Verify the proposal kind is `ChangePolicy`
- [ ] Verify the Admin group contains exactly the 6 accounts listed above

## Expected Results

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal kind should be `ChangePolicy`
- The Admin group account IDs should be exactly:
  - `c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816`
  - `as.near`
  - `root.near`
  - `e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac`
  - `anton-near-one.near`
  - `norfolks.near`

## Transaction Links

- Previous task: [Task 8: PRODUCTION: Upgrade venear contract](./8-upgrade-venear-contract.md)
- Proposal creation transaction: https://nearblocks.io/txns/DYBEk8zqtnQyVfxgjJ5uKzmbyupw9TbpmmZukD64vNPr
