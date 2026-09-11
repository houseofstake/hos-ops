# Task 6: Remove lane.near from Security Council

**Environment:** `PRODUCTION`

**Created by:** `lane.near`

## Background

This task removes `lane.near` from all governance roles across the House of Stake contracts:

1. **`rewards-claims2.sputnik-dao.near`** — SecurityCouncil role
2. **`vote.dao`** — Guardians and Reviewers (owned by `hos-root.sputnik-dao.near`)
3. **`hos-root.sputnik-dao.near`** — Admin role

Proposals have already been created and voted on by `lane.near`. The remaining Security Council members need to verify and approve the outstanding proposals.

---

## Part 1: Remove from rewards-claims2.sputnik-dao.near (COMPLETED)

**Status:** Executed (threshold was 1 vote)

Proposal to remove `lane.near` from the `SecurityCouncil` role was created and auto-approved.

- Proposal creation: https://nearblocks.io/txns/RGGtwjNURdJA2GHEvRmH7wE8weRrMVPEea99dE2LJhd
- Vote approve: https://nearblocks.io/txns/3ZdQgqAJoZsxUhDS1r8rSXrQFaLkKSdgi7hBZ4aAJgLN

### Verify Removal

```bash
near contract call-function as-read-only rewards-claims2.sputnik-dao.near get_policy \
  json-args '{}' \
  network-config mainnet now
```

- [ ] `lane.near` is no longer in the `SecurityCouncil` group
- [ ] SecurityCouncil now has 5 members

---

## Part 2: Remove from vote.dao Guardians and Reviewers (NEEDS 2 MORE VOTES)

**Proposal ID:** `18` on `hos-root.sputnik-dao.near`

**Votes so far:** 1/3 (`lane.near`)

This proposal calls two methods on `vote.dao`:
1. `set_guardians` — removes `lane.near` from the guardians list
2. `set_reviewer_ids` — removes `lane.near` from the reviewers list

- Proposal creation: https://nearblocks.io/txns/589aUkZQahzomeH3bFAqowLjkA3weneh7PvsCGccyifu
- Vote by lane.near: https://nearblocks.io/txns/2B3phmLRrMkxVc3ja3MdDWR8f9ssAeGuNaG5sArr4Uo9

### Step 1: View the Proposal

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal \
  json-args '{"id": 18}' \
  network-config mainnet now
```

### Step 2: Decode and Verify the Arguments

```bash
# Decode set_guardians args
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal \
  json-args '{"id": 18}' \
  network-config mainnet now 2>&1 | \
  jq -r '.kind.FunctionCall.actions[0].args' | base64 -d | jq .

# Decode set_reviewer_ids args
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal \
  json-args '{"id": 18}' \
  network-config mainnet now 2>&1 | \
  jq -r '.kind.FunctionCall.actions[1].args' | base64 -d | jq .
```

**Expected `set_guardians` args:**

```json
{
  "guardians": [
    "as.near",
    "c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816",
    "e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac",
    "fastnear-hos.near",
    "root.near"
  ]
}
```

**Expected `set_reviewer_ids` args:**

```json
{
  "reviewer_ids": [
    "as.near",
    "c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816",
    "e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac",
    "fastnear-hos.near",
    "root.near",
    "guiwickb.near",
    "gauntletgov.near"
  ]
}
```

### Step 3: Compare Against Current Config

```bash
near contract call-function as-read-only vote.dao get_config \
  json-args '{}' \
  network-config mainnet now
```

### Step 4: Verification Checklist

| Check | Expected | Verified |
|-------|----------|:--------:|
| `receiver_id` | `vote.dao` | [ ] |
| Action 1 `method_name` | `set_guardians` | [ ] |
| Action 2 `method_name` | `set_reviewer_ids` | [ ] |
| Guardians list is current list minus `lane.near` | 5 accounts | [ ] |
| Reviewers list is current list minus `lane.near` | 7 accounts | [ ] |
| No other accounts added or removed | Compare with current config | [ ] |

### Step 5: Approve the Proposal

> **Important:** Approve this proposal BEFORE approving proposal 19 (Part 3). Once proposal 19 executes, `lane.near` is removed from the Admin role and this proposal can no longer benefit from lane.near's vote.

```bash
export SIGNER_ACCOUNT_ID="<YOUR_ACCOUNT_ID>"

near contract call-function as-transaction hos-root.sputnik-dao.near act_proposal \
  json-args '{"id": 18, "action": "VoteApprove"}' \
  prepaid-gas '200.0 Tgas' \
  attached-deposit '0 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
```

### Post-Execution Verification

After the proposal executes (3/3 votes reached), verify the config was updated:

```bash
near contract call-function as-read-only vote.dao get_config \
  json-args '{}' \
  network-config mainnet now
```

- [ ] `lane.near` is not in `guardians`
- [ ] `lane.near` is not in `reviewer_ids`
- [ ] All other accounts are unchanged

---

## Part 3: Remove from hos-root.sputnik-dao.near Admin Role (NEEDS 2 MORE VOTES)

**Proposal ID:** `19` on `hos-root.sputnik-dao.near`

**Votes so far:** 1/3 (`lane.near`)

> **Warning:** Once this proposal executes, `lane.near` loses all Admin privileges on the root DAO. Make sure proposal 18 has been approved and executed first.

- Proposal creation: https://nearblocks.io/txns/GBrmjf6CYX3bPUs5wWgbxu8jbWJ4aNHUwK49yGDDTiGY
- Vote by lane.near: https://nearblocks.io/txns/DcKof2UGjKS34TGA7tdiePfG3TBCawxC45C7pdHegJjf

### Step 1: View the Proposal

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal \
  json-args '{"id": 19}' \
  network-config mainnet now
```

**Expected proposal:**

```json
{
  "description": "Remove lane.near from Admin role",
  "kind": {
    "RemoveMemberFromRole": {
      "member_id": "lane.near",
      "role": "Admin"
    }
  },
  "status": "InProgress"
}
```

### Step 2: Verification Checklist

| Check | Expected | Verified |
|-------|----------|:--------:|
| Proposal kind | `RemoveMemberFromRole` | [ ] |
| `member_id` | `lane.near` | [ ] |
| `role` | `Admin` | [ ] |
| Proposal 18 already executed | `vote.dao` config updated | [ ] |

### Step 3: Approve the Proposal

```bash
export SIGNER_ACCOUNT_ID="<YOUR_ACCOUNT_ID>"

near contract call-function as-transaction hos-root.sputnik-dao.near act_proposal \
  json-args '{"id": 19, "action": "VoteApprove"}' \
  prepaid-gas '200.0 Tgas' \
  attached-deposit '0 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
```

### Post-Execution Verification

After the proposal executes (3/3 votes reached), verify the policy was updated:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_policy \
  json-args '{}' \
  network-config mainnet now
```

- [ ] `lane.near` is not in the `Admin` group
- [ ] Admin group now has 5 members

---

## Approval Order

It is critical to approve the proposals in the correct order:

1. **Proposal 18** (vote.dao guardians/reviewers) — approve and wait for execution
2. **Proposal 19** (hos-root Admin removal) — approve only after proposal 18 has executed

If proposal 19 is approved first, `lane.near` loses Admin access and their vote on proposal 18 no longer counts, potentially requiring re-submission.

---

## Contract Addresses (PRODUCTION)

| Contract | Account ID |
|----------|------------|
| Root DAO | `hos-root.sputnik-dao.near` |
| Rewards DAO | `rewards-claims2.sputnik-dao.near` |
| Voting | `vote.dao` |
| veNEAR | `venear.dao` |

---

## Transaction Links (PRODUCTION)

| Action | TX |
|--------|------|
| Remove from rewards-claims2 SecurityCouncil (proposal) | https://nearblocks.io/txns/RGGtwjNURdJA2GHEvRmH7wE8weRrMVPEea99dE2LJhd |
| Remove from rewards-claims2 SecurityCouncil (approve) | https://nearblocks.io/txns/3ZdQgqAJoZsxUhDS1r8rSXrQFaLkKSdgi7hBZ4aAJgLN |
| Remove from vote.dao guardians/reviewers (proposal 18) | https://nearblocks.io/txns/589aUkZQahzomeH3bFAqowLjkA3weneh7PvsCGccyifu |
| Vote approve proposal 18 (lane.near) | https://nearblocks.io/txns/2B3phmLRrMkxVc3ja3MdDWR8f9ssAeGuNaG5sArr4Uo9 |
| Remove from hos-root Admin (proposal 19) | https://nearblocks.io/txns/GBrmjf6CYX3bPUs5wWgbxu8jbWJ4aNHUwK49yGDDTiGY |
| Vote approve proposal 19 (lane.near) | https://nearblocks.io/txns/DcKof2UGjKS34TGA7tdiePfG3TBCawxC45C7pdHegJjf |
