# Task 5: Update Rewards Contract

**Environment:** `PRODUCTION`

**Created by:** `lane.near`

## Background

The rewards merkle claim contract was previously deployed to `rewards.hos-dao.near`. This task re-deploys it to `rewards.dao` using the new MPC-derived key feature of the `near` CLI, and transfers remaining funds to the new contract.

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

## Part 2: Verify the Owner DAO

The owner DAO was deployed in the previous task. In this task we just verify the deployment.

### 2.1 Verify the DAO Deployment

```bash
export DAO_NAME="rewards-claims2"
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

## Part 3: Add Derived Public Key

```bash
near account add-key dao grant-full-access use-mpc-contract hos-root.sputnik-dao.near ed25519 derivation-path dao network-config mainnet
```

---

## Part 4: Deploy the Merkle Claim Contract

### 4.1 Set Environment Variables

```bash
export CLAIMS_ACCOUNT_ID="rewards.dao"
export DAO_ACCOUNT_ID="rewards-claims2.sputnik-dao.near"
export MIN_STORAGE_DEPOSIT="100000000000000000000000"  # 0.1 NEAR
```

### 4.2 Create the Account

```bash
near account create-account fund-myself $CLAIMS_ACCOUNT_ID '0.1 NEAR' autogenerate-new-keypair save-to-keychain sign-as dao network-config mainnet sign-with-mpc hos-root.sputnik-dao.near ed25519 derivation-path dao prepaid-gas '15.0 Tgas' attached-deposit '1 yoctoNEAR' submit-mpc-as-dao-proposal $SIGNER_ACCOUNT_ID 'Creating '"$CLAIMS_ACCOUNT_ID"' account' prepaid-gas '10.0 Tgas' attached-deposit '1 NEAR'
```

### 4.3 Deploy and Initialize

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

### 4.4 Delete Access Keys (Critical!)

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

### 4.5 Verify Deployment

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

---

## Part 5: Withdraw Funds from Old Contract

> **WARNING:** Do NOT proceed if there is an active campaign with unclaimed rewards. Users must be able to claim their rewards before withdrawing funds.

### 5.1 Check for Active Campaigns

```bash
export OLD_CLAIMS_ACCOUNT_ID="rewards.hos-dao.near"

# Get the last campaign ID
near contract call-function as-read-only $OLD_CLAIMS_ACCOUNT_ID get_last_campaign_id \
  json-args '{}' \
  network-config mainnet now

# Check campaign details (replace CAMPAIGN_ID with the result above)
near contract call-function as-read-only $OLD_CLAIMS_ACCOUNT_ID get_campaign \
  json-args '{"campaign_id": CAMPAIGN_ID}' \
  network-config mainnet now
```

The `claim_end` field is a UNIX timestamp in **nanoseconds**. To check if the campaign has ended:

```bash
# Set the claim_end value from the campaign (in nanoseconds)
export CLAIM_END_NS=<CLAIM_END_FROM_ABOVE>

# Convert to seconds and compare with current time
export CLAIM_END_SEC=$((CLAIM_END_NS / 1000000000))
echo "Current time:  $(date +%s) ($(date))"
echo "Claim end:     $CLAIM_END_SEC ($(date -d @$CLAIM_END_SEC))"

# Check if campaign has ended
if [ $(date +%s) -gt $CLAIM_END_SEC ]; then
  echo "✅ Campaign has ended - safe to proceed"
else
  echo "❌ Campaign is still active - DO NOT proceed"
fi
```

If the campaign is still active, **STOP** and wait for it to end.

### 5.2 Verify Owner and Check Balance

```bash
# Check the owner of the old contract
near contract call-function as-read-only $OLD_CLAIMS_ACCOUNT_ID get_config \
  json-args '{}' \
  network-config mainnet now

# The old contract is owned by the NEW DAO (ownership was transferred)
# Verify owner_account_id matches $DAO_ACCOUNT_ID from Part 4

# Check remaining balance
near account view-account-summary $OLD_CLAIMS_ACCOUNT_ID network-config mainnet now
```

### 5.3 Create Proposal to Withdraw Funds

```bash
near contract call-function as-transaction $DAO_ACCOUNT_ID add_proposal \
  json-args '{
    "proposal": {
      "description": "Withdraw remaining funds from old rewards contract to DAO",
      "kind": {
        "FunctionCall": {
          "receiver_id": "'"$OLD_CLAIMS_ACCOUNT_ID"'",
          "actions": [{
            "method_name": "withdraw",
            "args": "",
            "deposit": "0",
            "gas": "50000000000000"
          }]
        }
      }
    }
  }' \
  prepaid-gas '30.0 Tgas' \
  attached-deposit '0.1 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
```

### 5.4 Check and Approve Withdraw Proposal

```bash
# Record the proposal ID from the previous step
export WITHDRAW_PROPOSAL_ID=<PROPOSAL_ID>

# Verify the proposal contents
near contract call-function as-read-only $DAO_ACCOUNT_ID get_proposal \
  json-args '{"id": '"$WITHDRAW_PROPOSAL_ID"'}' \
  network-config mainnet now
```

**Verify the proposal contains:**
- `receiver_id`: `rewards.hos-dao.near` (the old claims contract)
- `method_name`: `withdraw`
- `deposit`: `0`

```bash
# Approve the proposal (repeat for each required signer)
near contract call-function as-transaction $DAO_ACCOUNT_ID act_proposal \
  json-args '{"id": '"$WITHDRAW_PROPOSAL_ID"', "action": "VoteApprove"}' \
  prepaid-gas '150.0 Tgas' \
  attached-deposit '0 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
```

### 5.5 Create Proposal to Transfer Funds to New Contract

After the withdraw proposal executes, the funds are now in the DAO. Create a proposal to transfer them to the new claims contract:

```bash
# Check the DAO balance to determine transfer amount
near account view-account-summary $DAO_ACCOUNT_ID network-config mainnet now

# Set the amount to transfer (in yoctoNEAR, e.g., "1000000000000000000000000" = 1 NEAR)
export TRANSFER_AMOUNT="<AMOUNT_IN_YOCTONEAR>"

near contract call-function as-transaction $DAO_ACCOUNT_ID add_proposal \
  json-args '{
    "proposal": {
      "description": "Transfer funds to new rewards contract at '"$CLAIMS_ACCOUNT_ID"'",
      "kind": {
        "Transfer": {
          "token_id": "",
          "receiver_id": "'"$CLAIMS_ACCOUNT_ID"'",
          "amount": "'"$TRANSFER_AMOUNT"'"
        }
      }
    }
  }' \
  prepaid-gas '30.0 Tgas' \
  attached-deposit '0.1 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
```

### 5.6 Check and Approve Transfer Proposal

```bash
# Record the proposal ID from the previous step
export TRANSFER_PROPOSAL_ID=<PROPOSAL_ID>

# Check proposal status
near contract call-function as-read-only $DAO_ACCOUNT_ID get_proposal \
  json-args '{"id": '"$TRANSFER_PROPOSAL_ID"'}' \
  network-config mainnet now

# Approve the proposal (repeat for each required signer)
near contract call-function as-transaction $DAO_ACCOUNT_ID act_proposal \
  json-args '{"id": '"$TRANSFER_PROPOSAL_ID"', "action": "VoteApprove"}' \
  prepaid-gas '150.0 Tgas' \
  attached-deposit '0 NEAR' \
  sign-as $SIGNER_ACCOUNT_ID \
  network-config mainnet sign-with-keychain send
```

### 5.7 Verify Funds Received

```bash
near account view-account-summary $CLAIMS_ACCOUNT_ID network-config mainnet now
```

**Verification Checklist:**

- [ ] No active campaigns on old contract (claim_end has passed)
- [ ] Withdraw proposal created and approved
- [ ] Transfer proposal created and approved
- [ ] New claims contract has received the funds

---

## Part 6: Campaign Operations

As in the [previous task](4-near-rewards-merkle-verification.md), this part remains unchanged.

---

## Part 7: Security Council Verification & Approval

As in the [previous task](4-near-rewards-merkle-verification.md), this part remains unchanged.

---

## Part 8: User Claims

As in the [previous task](4-near-rewards-merkle-verification.md), this part remains unchanged.

---

## Deployment Checklist Summary

### Initial Deployment

- [ ] Merkle claim contract built and hash verified
- [ ] Owner DAO created with correct policy
- [ ] Derived pubkey added to security council DAO
- [ ] Merkle claim contract deployed with DAO as owner
- [ ] All access keys deleted from claims contract
- [ ] Funds withdrawn from old contract and transferred to new contract

### Per-Campaign

As before.

---

## Contract Addresses (PRODUCTION)

| Contract | Account ID | Environment |
|----------|------------|-------------|
| Owner DAO | `rewards-claims2.sputnik-dao.near` | PRODUCTION |
| Merkle Claim (new) | `rewards.dao` | PRODUCTION |
| Merkle Claim (old) | `rewards.hos-dao.near` | PRODUCTION |
| veNEAR | `venear.dao` | PRODUCTION |

---

## Transaction Links (PRODUCTION)

- Add derived public key: https://nearblocks.io/txns/21u1dAbCx3GqVmu5PxexMheSgz2qwc71J7cxfYpPZiqR
- DAO creation: https://nearblocks.io/txns/EqX1nag1WDh7Whb3Ne32xQsGr68mmrYZVctt8eXGpGcu
- Claims contract deployment: https://nearblocks.io/txns/Vs2xJMdzMe5rCmL4hSTCEQLhFPbVgzzzaGqDejgfoiJ
- Key deletion: https://nearblocks.io/txns/EcL7BjbrnBeybww5LubNy6zJiKPWi47m83iVQFGFuJvS

