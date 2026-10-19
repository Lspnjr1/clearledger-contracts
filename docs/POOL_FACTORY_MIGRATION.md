# Pool Factory Migration Guide

This document describes the migration path for the existing deployed testnet pool contract to become "instance #1" under the new pool_factory, avoiding a disruptive redeploy that would orphan existing LP positions.

## Background

The current testnet deployment has a single USDC pool contract that predates the factory pattern. To support multiple assets via the new pool_factory, we need to register this existing pool as the first instance without requiring LPs to withdraw and redeposit.

## Migration Steps

### 1. Deploy pool_factory

Deploy the new pool_factory contract following the standard deployment process in `DEPLOYMENT.md`.

### 2. Register the existing pool

The factory will need a `register_existing_pool(asset, pool_address)` function that:

- Takes an asset address and an already-deployed pool contract address
- Skips the deploy-and-initialize step (since the pool already exists)
- Records the mapping: `DataKey::PoolForAsset(asset) = pool_address`
- Updates the asset index: increments `DataKey::AssetCount` and appends to `DataKey::AssetIndex`

This variant is scoped as a follow-up issue. Once implemented, the migration command would be:

```bash
stellar contract invoke \
  --id <factory_contract_id> \
  --w <deployer_wallet> \
  register_existing_pool \
  --asset <usdc_asset_address> \
  --pool_address <existing_pool_contract_id>
```

### 3. Verify registration

Call `list_assets()` on the factory to confirm the USDC asset is registered:

```bash
stellar contract invoke \
  --id <factory_contract_id> \
  list_assets
```

The response should include the USDC asset address.

### 4. Verify pool lookup

Call `get_pool_for_asset(asset)` to confirm the factory returns the existing pool address:

```bash
stellar contract invoke \
  --id <factory_contract_id> \
  get_pool_for_asset \
  --asset <usdc_asset_address>
```

## Follow-up Requirements

The `register_existing_pool` function has been implemented. This function:

- Accepts `asset: Address` and `pool_address: Address` parameters
- Requires admin authorization
- Checks that the asset is not already registered
- Reads `pool_address`'s `get_funding_asset` view and rejects the registration
  unless it equals the declared `asset`, so a caller cannot point an asset at a
  pool that custodies something else
- Rejects addresses that cannot answer that view (accounts, non-pool
  contracts, trapping calls) as `InvalidPool`; a readable pool that reports a
  different asset is rejected as `FundingAssetMismatch`. Neither path writes the
  mapping, index entry, or registration event
- Updates the same storage keys as `register_asset` would, but skips deployment
- Updates the asset index and asset-to-pool mapping

**Trust boundary:** the factory treats the declared `asset` and the pool's
on-chain funding asset as independent claims and only records a mapping when
they agree. `register_asset` applies the same check to the instance it just
deployed, before recording it.

Assets registered after the migration go through `register_asset`, which
deploys and initializes the pool instance instead of adopting an existing one.
It requires an escrow contract that was already initialized for the same asset,
because `pool::initialize` cross-checks the escrow's configured asset against
its own:

```bash
stellar contract invoke \
  --id <factory_contract_id> \
  --w <admin_wallet> \
  register_asset \
  --asset <asset_address> \
  --pool_wasm_hash <uploaded_pool_wasm_hash> \
  --invoice_contract <invoice_contract_id> \
  --escrow_contract <escrow_contract_id>
```

Both paths write `DataKey::PoolForAsset`, so `get_pool_for_asset(asset)` is the
single lookup for migrated and newly registered assets alike.

## Benefits

This migration approach:

- Preserves all existing LP positions and shares
- Avoids requiring LPs to withdraw and redeposit
- Maintains the pool's existing state and liquidity
- Allows seamless integration with the new factory pattern
- Enables future asset registrations via the standard `register_asset` flow
