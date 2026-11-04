# Task 19: PRODUCTION: Close `far`, update name prices, register TLA Cohort Two and set up business TLAs on `registry.hos`

**Environment:** `PRODUCTION`  
**Created by:** norfolks.near

## Background

Proposal to update the **production** TLA registry contract (`registry.hos`) in a single DAO proposal: close `far` to the public, lower name prices, register TLA Cohort Two and set up seven Business TLAs.

The proposal is one `FunctionCall` with 18 actions on `registry.hos`:

| # | Action | What it does |
|---|---|---|
| 0 | `update_fee_config` | New rent tiers, no Business name limit, Business name fee $0 |
| 1 | `register_tlas` | [TLA Cohort Two](./data/19-tla-cohort-two-names.txt), Open, Standard, no licensee |
| 2 | `activate_open_tlas` | Activates `hoot`, `meteor`, `legion`, `rhea`, `noir`, `hermes` for one year |
| 3–10 | `admin_set_tla_type` | `far`, `rekt`, `hoot`, `meteor`, `legion`, `rhea`, `noir`, `hermes` Open → Business with their [licensees](#business-tlas-being-set-up) |
| 11–17 | `enable_business_resale` | `rekt`, `hoot`, `meteor`, `legion`, `rhea`, `noir`, `hermes` |

### Fee config changes

Only the fields below change. All other fee settings are unchanged.

| Field | Current | New |
|---|---|---|
| `rent_tier_5_usd_micro` | `50000000` ($50) | `10000000` ($10) |
| `rent_tier_8_usd_micro` | `20000000` ($20) | `6666667` ($6.666667) |
| `rent_tier_10_usd_micro` | `10000000` ($10) | `5000000` ($5) |
| `rent_tier_12plus_usd_micro` | `5000000` ($5) | `3333334` ($3.333334) |
| `sub_fee_per_account_usd_micro` | `500000` ($0.50) | `0` ($0) |
| `business_max_subs` | `1000` | `4294967295` |


### Names being registered

- TLA Cohort Two: [506 names in order](./data/19-tla-cohort-two-names.txt), registered as Open, `Standard`, no licensee.
- `hoot`, `meteor`, `legion`, `rhea`, `noir` and `hermes` are activated, then changed to Business. The other names are only registered, not activated.

### Business TLAs being set up

| TLA | Licensee | Resale |
|---|---|---|
| `far` | `hos-root.sputnik-dao.near` | not enabled |
| `rekt` | `tattothetoo.near` | enabled |
| `hoot` | `hootname.near` | enabled |
| `meteor` | `meteorname.near` | enabled |
| `legion` | `tattothetoo.near` | enabled |
| `rhea` | `rheaname.near` | enabled |
| `noir` | `noirname.near` | enabled |
| `hermes` | `agentfilmer.near` | enabled |

### For reference: DAO proposal creation process

List the actions as `[method, args, gas]`, in order. The TLA Cohort Two names come from the data file:

```bash
export DATA_FILE='tasks/production/data/19-tla-cohort-two-names.txt'
export ACTIONS=$(jq -nRc '[inputs] as $names | [
  ["update_fee_config", {config: {tla_allocation_fee_usd_micro: "1000000000", rent_tier_5_usd_micro: "10000000", rent_tier_8_usd_micro: "6666667", rent_tier_10_usd_micro: "5000000", rent_tier_12plus_usd_micro: "3333334", sub_fee_per_account_usd_micro: "0", account_creation_deposit_yocto: "12000000000000000000000", business_max_subs: 4294967295, retraction_notice_ns: "604800000000000", max_rate_move_bps: 2000, quote_slippage_bps: 527, min_near_usd_rate_micro: "100000", max_near_usd_rate_micro: "100000000", rate_update_cooldown_ns: "300000000000", max_rate_age_ns: "21600000000000"}}, "2600000000000"],
  ["register_tlas", {tla_ids: $names, tla_type: "Open", premium_category: "Standard", licensee: null}, "214500000000000"],
  ["activate_open_tlas", {tla_ids: ["hoot", "meteor", "legion", "rhea", "noir", "hermes"]}, "4400000000000"],
  ["admin_set_tla_type", {tla_id: "far", tla_type: "Business", licensee: "hos-root.sputnik-dao.near"}, "2900000000000"],
  ["admin_set_tla_type", {tla_id: "rekt", tla_type: "Business", licensee: "tattothetoo.near"}, "2900000000000"],
  ["admin_set_tla_type", {tla_id: "hoot", tla_type: "Business", licensee: "hootname.near"}, "2900000000000"],
  ["admin_set_tla_type", {tla_id: "meteor", tla_type: "Business", licensee: "meteorname.near"}, "2900000000000"],
  ["admin_set_tla_type", {tla_id: "legion", tla_type: "Business", licensee: "tattothetoo.near"}, "2900000000000"],
  ["admin_set_tla_type", {tla_id: "rhea", tla_type: "Business", licensee: "rheaname.near"}, "2900000000000"],
  ["admin_set_tla_type", {tla_id: "noir", tla_type: "Business", licensee: "noirname.near"}, "2900000000000"],
  ["admin_set_tla_type", {tla_id: "hermes", tla_type: "Business", licensee: "agentfilmer.near"}, "2900000000000"],
  ["enable_business_resale", {tla_id: "rekt"}, "2900000000000"],
  ["enable_business_resale", {tla_id: "hoot"}, "2900000000000"],
  ["enable_business_resale", {tla_id: "meteor"}, "2900000000000"],
  ["enable_business_resale", {tla_id: "legion"}, "2900000000000"],
  ["enable_business_resale", {tla_id: "rhea"}, "2900000000000"],
  ["enable_business_resale", {tla_id: "noir"}, "2900000000000"],
  ["enable_business_resale", {tla_id: "hermes"}, "2900000000000"]
]' $DATA_FILE)
```

Proposal args. Each action gets deposit `1` and its args base64-encoded:

```bash
export PROPOSAL_DESCRIPTION='PRODUCTION: Close `far`, update name prices, register TLA Cohort Two and set up business TLAs on `registry.hos`. `far`: Open to Business, licensee hos-root.sputnik-dao.near. Rent tiers updated. Business name fee: $0.50 to $0. Business name limit: 1000 to none. TLA Cohort Two: 506 names. Business TLAs with resale enabled: `rekt` (tattothetoo.near), `hoot` (hootname.near), `meteor` (meteorname.near), `legion` (tattothetoo.near), `rhea` (rheaname.near), `noir` (noirname.near), `hermes` (agentfilmer.near).'
export CONTRACT_ID='registry.hos'
export PROPOSAL_ARGS=$(jq -nc \
  --arg description "$PROPOSAL_DESCRIPTION" \
  --arg receiver_id "$CONTRACT_ID" \
  --argjson actions "$ACTIONS" \
  '{proposal: {description: $description, kind: {FunctionCall: {receiver_id: $receiver_id,
    actions: ($actions | map({method_name: .[0], args: (.[1] | tojson | @base64), deposit: "1", gas: .[2]}))}}}}')
printf '%s' "$PROPOSAL_ARGS" | shasum -a 256
# Expected: 70816cf684c8b29b76551b19514b58d12ac8ef9c7dd347ea4b5306f89a31d116  -
export PROPOSAL_ARGS_B64=$(printf '%s' "$PROPOSAL_ARGS" | base64 | tr -d '\n')
printf '%s' "$PROPOSAL_ARGS_B64" | shasum -a 256
# Expected: 868471b8178e370a135189a5092dad435191a29f01f2e27133421e9c8c154164  -
```

CLI command to create the proposal:

```bash
export DAO_ACCOUNT="hos-root.sputnik-dao.near"
export SIGNER_ACCOUNT_ID="norfolks.near"
near contract call-function as-transaction $DAO_ACCOUNT add_proposal base64-args $PROPOSAL_ARGS_B64 prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as $SIGNER_ACCOUNT_ID network-config mainnet
# Proposal ID returned: 43
# TX ID: [TBD]
```

## Proposal Details

**Proposal ID:** #43

**Description:** PRODUCTION: Close `far`, update name prices, register TLA Cohort Two and set up business TLAs on `registry.hos`

**Expected result:** Once executed, `far` is a Business TLA licensed to `hos-root.sputnik-dao.near`, the new fee config is active, TLA Cohort Two is registered, and `rekt`, `hoot`, `meteor`, `legion`, `rhea`, `noir` and `hermes` are active Business TLAs with their licensees and resale enabled. The registry holds 1008 TLAs.

## Verification Steps

> **⚠️ ENVIRONMENT CHECK**: This is a `PRODUCTION` task. Verify all contract addresses and proposals match the PRODUCTION environment.

### Step 1: Check the Proposal

Use the NEAR CLI to retrieve the proposal:

```bash
export DATA_FILE='tasks/production/data/19-tla-cohort-two-names.txt'
export PROPOSAL=$(near contract call-function as-read-only hos-root.sputnik-dao.near get_proposal json-args '{"id": 43}' network-config mainnet now 2>/dev/null)
printf '%s' "$PROPOSAL" | jq '.description, .kind.FunctionCall.receiver_id, (.kind.FunctionCall.actions | length)'
# Expected:
# "PRODUCTION: Close `far`, update name prices, register TLA Cohort Two and set up business TLAs on `registry.hos`. `far`: Open to Business, licensee hos-root.sputnik-dao.near. Rent tiers updated. Business name fee: $0.50 to $0. Business name limit: 1000 to none. TLA Cohort Two: 506 names. Business TLAs with resale enabled: `rekt` (tattothetoo.near), `hoot` (hootname.near), `meteor` (meteorname.near), `legion` (tattothetoo.near), `rhea` (rheaname.near), `noir` (noirname.near), `hermes` (agentfilmer.near)."
# "registry.hos"
# 18
```

List every action with its deposit and decoded arguments. The 506 registered names are shown as a count and checked against the data file below:

```bash
printf '%s' "$PROPOSAL" | jq -c '.kind.FunctionCall.actions | to_entries[] | [.key, .value.method_name, .value.deposit, (.value.args | @base64d | fromjson | if (.tla_ids | length) > 10 then .tla_ids |= length else . end)]'
# Expected:
# [0,"update_fee_config","1",{"config":{"tla_allocation_fee_usd_micro":"1000000000","rent_tier_5_usd_micro":"10000000","rent_tier_8_usd_micro":"6666667","rent_tier_10_usd_micro":"5000000","rent_tier_12plus_usd_micro":"3333334","sub_fee_per_account_usd_micro":"0","account_creation_deposit_yocto":"12000000000000000000000","business_max_subs":4294967295,"retraction_notice_ns":"604800000000000","max_rate_move_bps":2000,"quote_slippage_bps":527,"min_near_usd_rate_micro":"100000","max_near_usd_rate_micro":"100000000","rate_update_cooldown_ns":"300000000000","max_rate_age_ns":"21600000000000"}}]
# [1,"register_tlas","1",{"tla_ids":506,"tla_type":"Open","premium_category":"Standard","licensee":null}]
# [2,"activate_open_tlas","1",{"tla_ids":["hoot","meteor","legion","rhea","noir","hermes"]}]
# [3,"admin_set_tla_type","1",{"tla_id":"far","tla_type":"Business","licensee":"hos-root.sputnik-dao.near"}]
# [4,"admin_set_tla_type","1",{"tla_id":"rekt","tla_type":"Business","licensee":"tattothetoo.near"}]
# [5,"admin_set_tla_type","1",{"tla_id":"hoot","tla_type":"Business","licensee":"hootname.near"}]
# [6,"admin_set_tla_type","1",{"tla_id":"meteor","tla_type":"Business","licensee":"meteorname.near"}]
# [7,"admin_set_tla_type","1",{"tla_id":"legion","tla_type":"Business","licensee":"tattothetoo.near"}]
# [8,"admin_set_tla_type","1",{"tla_id":"rhea","tla_type":"Business","licensee":"rheaname.near"}]
# [9,"admin_set_tla_type","1",{"tla_id":"noir","tla_type":"Business","licensee":"noirname.near"}]
# [10,"admin_set_tla_type","1",{"tla_id":"hermes","tla_type":"Business","licensee":"agentfilmer.near"}]
# [11,"enable_business_resale","1",{"tla_id":"rekt"}]
# [12,"enable_business_resale","1",{"tla_id":"hoot"}]
# [13,"enable_business_resale","1",{"tla_id":"meteor"}]
# [14,"enable_business_resale","1",{"tla_id":"legion"}]
# [15,"enable_business_resale","1",{"tla_id":"rhea"}]
# [16,"enable_business_resale","1",{"tla_id":"noir"}]
# [17,"enable_business_resale","1",{"tla_id":"hermes"}]
```

Compare the registered names with the data file (no output means they match, in the same order):

```bash
diff <(printf '%s' "$PROPOSAL" | jq -r '.kind.FunctionCall.actions[1].args | @base64d | fromjson | .tla_ids[]') $DATA_FILE
```

Compare the new fee config with the current one. Only the six fields in the [fee config table](#fee-config-changes) should differ:

```bash
diff <(near contract call-function as-read-only registry.hos get_fee_config json-args '{}' network-config mainnet now 2>/dev/null | jq -S .) <(printf '%s' "$PROPOSAL" | jq -S '.kind.FunctionCall.actions[0].args | @base64d | fromjson | .config')
```

### Step 2: Verify Target Contract and Parameters

- [ ] Confirm target contract is `registry.hos`
- [ ] Verify the proposal kind is `FunctionCall` and contains exactly 18 actions
- [ ] Verify every action has deposit `1` yoctoNEAR
- [ ] Check action 0 changes only `rent_tier_5_usd_micro`, `rent_tier_8_usd_micro`, `rent_tier_10_usd_micro`, `rent_tier_12plus_usd_micro`, `sub_fee_per_account_usd_micro` and `business_max_subs` in the fee config diff, to the values in the [fee config table](#fee-config-changes)
- [ ] Check action 1 registers the 506 names in the [data file](./data/19-tla-cohort-two-names.txt) as `Open`, `Standard`, licensee `null`
- [ ] Check action 2 activates `hoot`, `meteor`, `legion`, `rhea`, `noir` and `hermes`
- [ ] Check action 3 sets `far` to `Business` with licensee `hos-root.sputnik-dao.near`
- [ ] Check action 4 sets `rekt` to `Business` with licensee `tattothetoo.near`
- [ ] Check actions 5–10 set the six new TLAs to `Business` with the licensees in the [Business TLAs table](#business-tlas-being-set-up)
- [ ] Check actions 11–17 enable resale on `rekt`, `hoot`, `meteor`, `legion`, `rhea`, `noir` and `hermes`
- [ ] Check resale is **not** enabled on `far`

### Step 3: Additional Checks

- [ ] Review the proposer account
- [ ] Verify the proposal status
- [ ] Check voting requirements
- [ ] Confirm no conflicting pending proposals

## Expected Results

- The DAO account should be `hos-root.sputnik-dao.near`
- The proposal should be in the `InProgress` status
- The proposal kind should be `FunctionCall`
- The proposal target account ID should be `registry.hos`
- The proposal should contain 18 actions in the order listed in the [Background](#background) table

## Transaction Links

- Previous task: [Task 18: PRODUCTION: Rotate a recovery watcher and the recovery attestation key on `rec.hos`](./18-rotate-rec-hos-watcher-and-attestation-key.md)
- Proposal creation transaction: [TBD]
