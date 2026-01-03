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
near account create-account fund-myself $CLAIMS_ACCOUNT_ID '0.1 NEAR' autogenerate-new-keypair save-to-keychain sign-as dao network-config mainnet sign-with-mpc hos-root.sputnik-dao.near ed25519 derivation-path dao prepaid-gas '15.0 Tgas' attached-deposit '1 yoctoNEAR' submit-mpc-as-dao-proposal $SIGNER_ACCOUNT_ID 'Creating reward.dao account' prepaid-gas '10.0 Tgas' attached-deposit '1 NEAR'
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

### 4.6 Fund the Contract

Transfer NEAR to the claims contract for reward distribution:

```bash
near tokens $SIGNER_ACCOUNT_ID send-near $CLAIMS_ACCOUNT_ID '[AMOUNT] NEAR' \
  network-config mainnet sign-with-keychain send
```

---

## Part 5: Campaign Operations

As in the [previous task](4-near-rewards-merkle-verification.md), this part remains unchanged.

---

## Part 6: Security Council Verification & Approval

As in the [previous task](4-near-rewards-merkle-verification.md), this part remains unchanged.

---

## Part 7: User Claims

As in the [previous task](4-near-rewards-merkle-verification.md), this part remains unchanged.

---

## Deployment Checklist Summary

### Initial Deployment

- [ ] Merkle claim contract built and hash verified
- [ ] Owner DAO created with correct policy
- [ ] Derived pubkey added to security council DAO
- [ ] Merkle claim contract deployed with DAO as owner
- [ ] All access keys deleted from claims contract
- [ ] Claims contract funded with NEAR

### Per-Campaign

As before.

---

## Contract Addresses (PRODUCTION)

| Contract | Account ID | Environment |
|----------|------------|-------------|
| Owner DAO | `rewards-claims2.sputnik-dao.near` | PRODUCTION |
| Merkle Claim | `rewards.dao` | PRODUCTION |
| veNEAR | `venear.dao` | PRODUCTION |

---

## Transaction Links (PRODUCTION)

- Add derived public key: https://nearblocks.io/txns/21u1dAbCx3GqVmu5PxexMheSgz2qwc71J7cxfYpPZiqR
- DAO creation: https://nearblocks.io/txns/EqX1nag1WDh7Whb3Ne32xQsGr68mmrYZVctt8eXGpGcu
- Claims contract deployment: https://nearblocks.io/txns/Vs2xJMdzMe5rCmL4hSTCEQLhFPbVgzzzaGqDejgfoiJ
- Key deletion: https://nearblocks.io/txns/EcL7BjbrnBeybww5LubNy6zJiKPWi47m83iVQFGFuJvS

