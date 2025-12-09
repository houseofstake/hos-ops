# Task 6: NEAR Rewards Merkle Verification

**Environment:** `STAGING`
**Created by:** voteagora.near

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
export DAO_NAME="rewards-claims"
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

- [ ] DAO created at `rewards-claims.sputnik-dao.near`
- [ ] SecurityCouncil role has all 6 members
- [ ] CampaignOperator role has `gauntlet-hos.near`
- [ ] `call` vote policy threshold is `1`
- [ ] Default vote policy threshold is `2`

---
### Deploying Smart Contracts

Below is a script to deploy the claim contract by an account created by the NF with keys revoked i.e. $CLAIMS_ACCOUNT_ID.  $GAUNTLET_ACCOUNT_ID - Is becomes the owner of the claims contract and will control the campaigns.

```
export CLAIMS_ACCOUNT_ID=[TBD]
export GAUNTLET_ACCOUNT_ID=[TBD]
export STORAGE_DEPOSIT="1 NEAR"

#0.1 NEAR (enough for 10000 bytes)
export MIN_STORAGE_DEPOSIT="100000000000000000000000"

near contract deploy $CLAIMS_ACCOUNT_ID use-file $TARGET with-init-call new json-args '{
  "config": {
    "owner_account_id": "'$GAUNTLET_ACCOUNT_ID'",
    "min_storage_deposit": "'$MIN_STORAGE_DEPOSIT'"
  }
}' prepaid-gas '10.0 Tgas' attached-deposit "$STORAGE_DEPOSIT" network-config mainnet sign-with-keychain send
```

### Operations

Once the contract is deployed Gauntlet will be able to create a campaign using a data pipeline and the [Agora Campaign Admin Application](https://near-claim.vercel.app/).

The data format they use will be a three column, comma separated file, with fields: `address,lockup,amount`

Ex.

```
address,lockup,amount
example.near,lockup-example.venear.dao,1134994235059497700000000
```

Gauntlet will then upload this file using the [New Campaign Page on the Agora Campaign Admin Application](https://near-claim.vercel.app/campaigns/new). 

The merkle tree will be generated and the root will be displayed in the UI.  

An example root would be: `0x0c443eee8b546bae8046a69e4762db5c39198f50fac5f139d0a455fc45a76749`, and in raw format:

```
[
    158,
    236,
    219,
    170,
    25,
    1,
    253,
    172,
    46,
    71,
    82,
    30,
    201,
    181,
    15,
    59,
    58,
    254,
    170,
    207,
    59,
    87,
    184,
    46,
    81,
    28,
    122,
    202,
    227,
    92,
    92,
    128
]
```

As well as the entire trie data structure: 

```
{
  "format": "near-v1",
  "tree": [
    "0x0c443eee8b546bae8046a69e4762db5c39198f50fac5f139d0a455fc45a76749",
    "0xac1d19b7898d30ebd5df0d49bd3f8baeac613281ec6d2e933979fa790620aa04",
    "0xec08963f75e400c9db83ee1a7b449d209ddeae5697bc31b233ab73ad650c2b0f",
    "0x5e8fa1b8ba11df6ec59ca8299114c4c1e49f11ddb46c0d6407f7ebd406a2f502",
    "0x14e93176748df38d3a732bd54c6f0bfdd9d017be32d95989736a7b6dabf74522"
  ],
  "values": [
    {
      "value": {
        "account": "zaki.near",
        "lockup": "91eb7cf64735a35fa614c32207390ca74b622e5b.venear.dao",
        "amount": "200000000000000000000000"
      },
      "treeIndex": 4
    },
    {
      "value": {
        "account": "vinibarbosa.near",
        "lockup": "00215ad0d7d939b2a1adb3bfa675266d38cf30c8.venear.dao",
        "amount": "200000000000000000000000"
      },
      "treeIndex": 2
    },
    {
      "value": {
        "account": "klausbravegov.near",
        "lockup": "8b4555ab3d03dc94fbd410f0ff518710a0df8c65.venear.dao",
        "amount": "200000000000000000000000"
      },
      "treeIndex": 3
    }
  ],
  "leafEncoding": [
    "account",
    "lockup",
    "amount"
  ]
}
```

The Security Council & Near Foundation will determine a valid claim period from the proposed start date. The recommendation is 30 days from the timestamp of the campaign creation. 

Gauntlet will update the claims contract with a new campaign running the following example command, note the setting in the command related to the claim period.

```bash
near contract call-function as-transaction $CLAIMS_CONTRACT create_campaign json-args '{"merkle_root": [
    158,
    236,
    219,
    170,
    25,
    1,
    253,
    172,
    46,
    71,
    82,
    30,
    201,
    181,
    15,
    59,
    58,
    254,
    170,
    207,
    59,
    87,
    184,
    46,
    81,
    28,
    122,
    202,
    227,
    92,
    92,
    128
], "claim_end": "1789228321000000000"}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as $GAUNTLET_ACCOUNT_ID network-config mainnet sign-with-keychain send
```

The [Agora Campaign Admin Application](https://near-claim.vercel.app/) has a UI for users to retrieve their proof artifacts, by searching based on accountIds, on a per-claim basis.

A user then can interact with the claims contract and transfer their rewards to their lockup using this command:

```bash
near contract call-function as-transaction $CLAIM_CONTRACT.near claim json-args '{"campaign_id": 1, "merkle_proof": [[..]], "lockup_contract": "lockup-example.near", "amount": "10000" }' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as $YOUR_ACCOUNT.near network-config mainnet sign-with-keychain send
```


## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
# PRODUCTION environment
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 17}' network-config mainnet now
```

### Step 2: Decode and Verify Arguments

Decode the inner arguments to verify the actual parameters:

```bash
near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 17}' network-config mainnet now | jq '.kind.FunctionCall.actions[0].args | @base64d | fromjson'
```

### Step 3: Verify Target Contract

- [ ] **CRITICAL**: Confirm target contract matches PRODUCTION environment
- [ ] Verify the function being called is correct
- [ ] Check all parameters are as specified in the proposal description
- [ ] If PRODUCTION, double-check proposal description for any STAGING indicators

### Step 4: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status
- [ ] Check voting requirements
- [ ] Confirm no conflicting pending proposals

## Expected Results

Verification should show that the merkle root used should produce a successful merkle claim check.

## Transaction Links

- Proposal creation transaction: [TBD]

## Notes
