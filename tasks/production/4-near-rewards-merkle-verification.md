# Task 6: NEAR Rewards Merkle Verification

**Environment:** `PRODUCTION`

**Created by:** `voteagora.near`

**Reviewed by:** `lane.near`

## Background

This proposal outlines the process for deploying the claims contract as well as instructions for publishing the first rewards campaign. 
Once this proposal has passed Gauntlet will have the ability to publish new campaigns.

## Proposal Details

Two new architecture components are introduced:

- The [NEAR Merkle Claim Smart Contract](https://github.com/voteagora/near-merkle-claim)

- The [Agora Campaign Admin Application](https://near-claim.vercel.app/).

## Governance Architecture

To balance operational efficiency with security, the merkle claim contract is owned by a **Sputnik DAO** that acts as a proxy layer:

```
┌──────────────────┐      ┌─────────────────────┐      ┌────────────────────┐
│   Gauntlet       │─────►│   Sputnik DAO       │─────►│   MerkleClaim      │
│   (proposes)     │      │   (approves)        │      │   (executes)       │
└──────────────────┘      └─────────────────────┘      └────────────────────┘
                                   │
                          Security Council
                          (1 member approval
                           for campaigns)
```

**Permission Model:**

| Action | Who Can Propose | Approval Required |
|--------|-----------------|-------------------|
| Create campaign | Gauntlet or SC | 1 SC member |
| Withdraw funds | Gauntlet or SC | 1 SC member |
| Change DAO policy | SC only | 2 SC members |
| Add/remove members | SC only | 2 SC members |

---

### Part 1. Building Smart Contracts

### 1.1 Clone and Build

```bash
git clone https://github.com/voteagora/near-merkle-claim
cd near-merkle-claim
git checkout c7587d357be507744c1e961d42f6fdc56e8803aa
cargo near build
```

### 1.2 Verify the Build

```bash
export CONTRACT_HASH=$(cat target/near/near_merkle_claim.wasm | sha256sum | awk '{ print $1 }' | xxd -r -p | base58)
echo $CONTRACT_HASH
# Expected: FMgQ6iPcerbU6whoDpYrj9LbMRBEnPV3brvgmHWCMqMA

ls -l target/near/near_merkle_claim.wasm
# Expected size: 159744 bytes
```

**Verification Checklist:**

- [ ] Contract hash matches: `FMgQ6iPcerbU6whoDpYrj9LbMRBEnPV3brvgmHWCMqMA`
- [ ] Contract size is 159744 bytes

---

## Part 2: Deploy the Owner DAO

The claims contract will be owned by a Sputnik DAO that provides granular access control.

### 2.1 Prepare the DAO Policy

```bash
export DAO_NAME="rewards-claims2"
export SIGNER_ACCOUNT_ID="lane.near"  # Change to your SC member account

export POLICY='{
  "roles": [
    {
      "name": "SecurityCouncil",
      "kind": {
        "Group": [
          "as.near",
          "e953bb69d1129e4da87b99739373884a0b57d5e64a65fdc868478f22e6c31eac",
          "c65255255d689f74ae46b0a89f04bbaab94d3a51ab9dc4b79b1e9b61e7cf6816",
          "fastnear-hos.near",
          "lane.near",
          "root.near"
        ]
      },
      "permissions": ["*:*"],
      "vote_policy": {
        "call": {
          "weight_kind": "RoleWeight",
          "quorum": "0",
          "threshold": "1"
        }
      }
    },
    {
      "name": "CampaignOperator",
      "kind": {
        "Group": ["gauntlet-hos.near"]
      },
      "permissions": ["call:AddProposal"],
      "vote_policy": {}
    }
  ],
  "default_vote_policy": {
    "weight_kind": "RoleWeight",
    "quorum": "0",
    "threshold": "2"
  },
  "proposal_bond": "100000000000000000000000",
  "proposal_period": "604800000000000",
  "bounty_bond": "100000000000000000000000",
  "bounty_forgiveness_period": "604800000000000"
}'
```

### 2.2 Create the DAO via Factory

```bash
export CONFIG='{
  "name": "'"$DAO_NAME"'",
  "purpose": "Merkle claim contract owner DAO for NEAR rewards distribution",
  "metadata": ""
}'

export ARGS=$(echo '{"config": '"$CONFIG"', "policy": '"$POLICY"'}' | base64 -w 0)

near contract call-function as-transaction sputnik-dao.near create json-args '{
  "name": "'"$DAO_NAME"'",
  "args": "'"$ARGS"'"
}' prepaid-gas '150.0 Tgas' attached-deposit '6 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
```

### 2.3 Verify the DAO Deployment

```bash
export DAO_ACCOUNT_ID="$DAO_NAME.sputnik-dao.near"

near contract call-function as-read-only $DAO_ACCOUNT_ID get_policy \
  json-args '{}' \
  network-config mainnet now
```

**Verification Checklist:**

- [ ] DAO created at `rewards-claims2.sputnik-dao.near`
- [ ] SecurityCouncil role has all 6 members
- [ ] CampaignOperator role has `gauntlet-hos.near`
- [ ] `call` vote policy threshold is `1`
- [ ] Default vote policy threshold is `2`

---

## Part 3: Deploy the Merkle Claim Contract

### 3.1 Set Environment Variables

```bash
export CLAIMS_ACCOUNT_ID="[TBD - account created by NF]"
export DAO_ACCOUNT_ID="rewards-claims2.sputnik-dao.near"
export MIN_STORAGE_DEPOSIT="100000000000000000000000"  # 0.1 NEAR
```

### 3.2 Deploy and Initialize

```bash
near contract deploy $CLAIMS_ACCOUNT_ID \
  use-file target/near/near_merkle_claim.wasm \
  with-init-call new json-args '{
    "config": {
      "owner_account_id": "'"$DAO_ACCOUNT_ID"'",
      "min_storage_deposit": "'"$MIN_STORAGE_DEPOSIT"'"
    }
  }' \
  prepaid-gas '10.0 Tgas' \
  attached-deposit '1 NEAR' \
  network-config mainnet sign-with-keychain send
```

### 3.3 Delete Access Keys (Critical!)

This makes the contract immutable - only the owner DAO can upgrade it.

```bash
# List current keys
near account list-keys $CLAIMS_ACCOUNT_ID network-config mainnet now

# Delete each full access key (repeat for each key shown)
near account delete-keys $CLAIMS_ACCOUNT_ID public-keys 'ed25519:...' \
  network-config mainnet sign-with-keychain send

# Verify no keys remain
near account list-keys $CLAIMS_ACCOUNT_ID network-config mainnet now
```

### 3.4 Verify Deployment

```bash
near contract call-function as-read-only $CLAIMS_ACCOUNT_ID get_config \
  json-args '{}' \
  network-config mainnet now
```

Expected output:

```json
{
  "owner_account_id": "rewards-claims2.sputnik-dao.near",
  "min_storage_deposit": "100000000000000000000000"
}
```

**Verification Checklist:**

- [ ] Contract deployed to correct account
- [ ] Owner is the DAO (`rewards-claims2.sputnik-dao.near`)
- [ ] All access keys deleted
- [ ] `min_storage_deposit` is correct

### 3.5 Fund the Contract

Transfer NEAR to the claims contract for reward distribution:

```bash
near tokens $SIGNER_ACCOUNT_ID send-near $CLAIMS_ACCOUNT_ID '[AMOUNT] NEAR' \
  network-config mainnet sign-with-keychain send
```

---

## Part 4: Campaign Operations

### 4.1 Campaign Data Format

Gauntlet prepares a CSV file with three columns:

```csv
address,lockup,amount
zaki.near,91eb7cf64735a35fa614c32207390ca74b622e5b.venear.dao,200000000000000000000000
vinibarbosa.near,00215ad0d7d939b2a1adb3bfa675266d38cf30c8.venear.dao,200000000000000000000000
```

> **Note:** The `lockup` address must match the user's actual veNEAR lockup contract.

### 4.2 Generate Merkle Tree

Upload the CSV to the [Agora Campaign Admin Application](https://near-claim.vercel.app/campaigns/new).

The app generates:
- **Merkle root** (hex and byte array format)
- **Full tree structure** (for proof generation)
- **Per-user proofs** (for claiming)

### 4.3 Create Campaign Proposal (Gauntlet)

The Security Council & Near Foundation will determine a valid claim period from the proposed start date. The recommendation is 30 days from the timestamp of the campaign creation.

```bash
export DAO_ACCOUNT_ID="rewards-claims2.sputnik-dao.near"
export CLAIMS_CONTRACT="[CLAIMS_ACCOUNT_ID]"

# Set claim end to 30 days from now (in nanoseconds)
export CLAIM_END=$(echo "$(date +%s)000000000 + 2592000000000000" | bc)

# Merkle root as byte array (from Agora app)
export MERKLE_ROOT='[158,236,219,170,25,1,253,172,46,71,82,30,201,181,15,59,58,254,170,207,59,87,184,46,81,28,122,202,227,92,92,128]'

# Encode the inner function call args
export INNER_ARGS=$(echo -n '{"merkle_root":'"$MERKLE_ROOT"',"claim_end":"'"$CLAIM_END"'"}' | base64 -w 0)

# Create the proposal
near contract call-function as-transaction $DAO_ACCOUNT_ID add_proposal json-args '{
  "proposal": {
    "description": "Create rewards campaign - [DESCRIPTION]",
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
  sign-as gauntlet-hos.near \
  network-config mainnet sign-with-keychain send
```

Note the returned proposal ID.

---

## Part 5: Security Council Verification & Approval

### 5.1 View the Proposal

```bash
export DAO_ACCOUNT_ID="rewards-claims2.sputnik-dao.near"
export PROPOSAL_ID=[ID_FROM_GAUNTLET]

near contract call-function as-read-only $DAO_ACCOUNT_ID get_proposal \
  json-args '{"id": '"$PROPOSAL_ID"'}' \
  network-config mainnet now
```

### 5.2 Decode and Verify Inner Arguments

```bash
near contract call-function as-read-only $DAO_ACCOUNT_ID get_proposal \
  json-args '{"id": '"$PROPOSAL_ID"'}' \
  network-config mainnet now 2>&1 | \
  jq -r '.kind.FunctionCall.actions[0].args' | \
  base64 -d | jq .
```

Expected output:

```json
{
  "merkle_root": [158, 236, 219, ...],
  "claim_end": "1736000000000000000"
}
```

### 5.3 Convert Timestamps for Verification

```bash
# Convert claim_end from nanoseconds to human-readable
export CLAIM_END_NS="1736000000000000000"
date -d @$((CLAIM_END_NS / 1000000000))
```

### 5.4 Convert Merkle Root to Hex (for comparison)

```bash
echo '[158,236,219,170,25,1,253,172,...]' | \
  jq -r 'map(if . < 16 then "0" + (. | tostring | explode | map(. - 48) | .[0] | . as $n | if $n < 10 then [$n + 48] else [$n + 87] end | implode) else (. | tostring | if length == 2 then [(. | explode[0]) - 48, (. | explode[1]) - 48] | map(if . < 10 then . + 48 else . + 87 end) | implode else . end) end) | join("")'

# Simpler alternative - use printf
python3 -c "print('0x' + bytes([158,236,219,170,25,1,253,172,46,71,82,30,201,181,15,59,58,254,170,207,59,87,184,46,81,28,122,202,227,92,92,128]).hex())"
```

### 5.5 Security Council Verification Checklist

| Check | Expected | Verified |
|-------|----------|----------|
| `receiver_id` is correct claims contract | `[CLAIMS_ACCOUNT_ID]` | ☐ |
| `method_name` is expected function | `create_campaign` | ☐ |
| `merkle_root` matches Gauntlet's provided root | Compare byte arrays or hex | ☐ |
| `claim_end` is ~30 days from now | Convert and verify date | ☐ |
| `deposit` is zero | `"0"` | ☐ |
| `gas` is reasonable | `50000000000000` (50 Tgas) | ☐ |
| Proposer is authorized | `gauntlet-hos.near` | ☐ |
| Proposal status | `InProgress` | ☐ |

### 5.6 Approve the Proposal

Once verification is complete, a single Security Council member approves:

```bash
# First, capture the proposal kind for the act_proposal call
export PROPOSAL_KIND=$(near contract call-function as-read-only $DAO_ACCOUNT_ID get_proposal \
  json-args '{"id": '"$PROPOSAL_ID"'}' \
  network-config mainnet now 2>&1 | jq -c '.kind')

near contract call-function as-transaction $DAO_ACCOUNT_ID act_proposal json-args '{
  "id": '"$PROPOSAL_ID"',
  "action": "VoteApprove",
  "proposal": '"$PROPOSAL_KIND"'
}' prepaid-gas '200.0 Tgas' attached-deposit '0 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
```

### 5.7 Verify Campaign Was Created

```bash
near contract call-function as-read-only $CLAIMS_CONTRACT get_campaign \
  json-args '{"campaign_id": 1}' \
  network-config mainnet now
```

---

## Part 6: User Claims

Users can claim their rewards through the [Agora Campaign Admin Application](https://near-claim.vercel.app/) or via CLI:

```bash
export CLAIMS_CONTRACT="[CLAIMS_ACCOUNT_ID]"
export CAMPAIGN_ID=1
export LOCKUP_CONTRACT="[USER_LOCKUP_ADDRESS]"
export AMOUNT="[AMOUNT_IN_YOCTONEAR]"

# Merkle proof as compact JSON (no newlines)
export MERKLE_PROOF='[[236,8,150,63,...],[211,233,148,168,...]]'

near contract call-function as-transaction $CLAIMS_CONTRACT claim json-args '{
  "campaign_id": '"$CAMPAIGN_ID"',
  "merkle_proof": '"$MERKLE_PROOF"',
  "lockup_contract": "'"$LOCKUP_CONTRACT"'",
  "amount": "'"$AMOUNT"'"
}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
  sign-as [USER_ACCOUNT] \
  network-config mainnet sign-with-keychain send
```

> **Important:** The `merkle_proof` must be a JSON array (not a string). No quotes around the array value.

---

## Troubleshooting

### RPC Rate Limits

If you encounter rate limit errors, use an alternative RPC endpoint:

```bash
# FASTNEAR (recommended for ad-hoc usage)
... network-config mainnet-fastnear ...
```

### JSON Parsing Errors

Ensure all JSON arrays are on a single line with no newlines:

```bash
# Compact a JSON array
export MERKLE_PROOF=$(echo "$MERKLE_PROOF" | jq -c .)
```

### Proposal Verification Failed

```bash
# Check proposal status
near contract call-function as-read-only $DAO_ACCOUNT_ID get_proposal \
  json-args '{"id": '"$PROPOSAL_ID"'}' \
  network-config mainnet now
```

---

## Deployment Checklist Summary

### Initial Deployment

- [ ] Merkle claim contract built and hash verified
- [ ] Owner DAO created with correct policy
- [ ] Merkle claim contract deployed with DAO as owner
- [ ] All access keys deleted from claims contract
- [ ] Claims contract funded with NEAR

### Per-Campaign

- [ ] CSV data verified (addresses, lockups, amounts)
- [ ] Merkle tree generated via Agora app
- [ ] Proposal created by Gauntlet
- [ ] Security council member verified proposal parameters
- [ ] Security council member approved proposal
- [ ] Campaign creation confirmed on-chain

---

## Contract Addresses (PRODUCTION)

| Contract | Account ID | Environment |
|----------|------------|-------------|
| Owner DAO | `rewards-claims2.sputnik-dao.near` | PRODUCTION |
| Merkle Claim | `rewards.hos-dao.near` | PRODUCTION |
| veNEAR | `venear.dao` | PRODUCTION |

---

## Transaction Links (PRODUCTION)

- DAO creation: https://nearblocks.io/txns/EqX1nag1WDh7Whb3Ne32xQsGr68mmrYZVctt8eXGpGcu
- Claims contract deployment: https://nearblocks.io/txns/Vs2xJMdzMe5rCmL4hSTCEQLhFPbVgzzzaGqDejgfoiJ
- Key deletion: https://nearblocks.io/txns/EcL7BjbrnBeybww5LubNy6zJiKPWi47m83iVQFGFuJvS

---

## Notes

- The Security Council can reject malicious proposals by voting `VoteReject` or `VoteRemove`
- Campaign operators cannot withdraw funds without SC approval
- All owner functions require going through the DAO proposal process
