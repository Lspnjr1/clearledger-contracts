# Escrow Contract

Custody and settlement contract for ClearLedger invoice financing on Stellar
testnet. It holds USDC between pool funding and issuer payout, returns funds to
the pool on repayment/default, and exposes read-only lock and history views.

> **Testnet only.** No Mainnet support, no formal audit, no Mainnet guarantees.

## Documentation

- [Custody and recovery paths](./CUSTODY.md) — sequence diagrams for funding,
  repayment, and default, including authorization and rollback behavior
- [Event catalog — Escrow Contract](../../docs/EVENTS.md#escrow-contract)
- [Storage schema — escrow_contract](../../docs/STORAGE.md#contract-escrow_contract)
- [Threat model — authentication gates](../../docs/THREAT_MODEL.md#authentication-gates)
- [Limitations](../../docs/LIMITATIONS.md)

## Entry points

```text
initialize(admin, pool_contract, usdc_asset)
lock(invoice_id, amount, issuer)            ← pool_contract only
release_to_issuer(invoice_id, issuer)       ← pool_contract only
release_to_pool(invoice_id, repayment_amount) ← pool_contract only
handle_default(invoice_id, caller)          ← admin or pool_contract
pause() / unpause()                         ← admin only
upgrade(new_wasm_hash)                      ← admin only
get_locked(invoice_id) / get_locked_at(invoice_id) / get_history(invoice_id)
```

See [`CUSTODY.md`](./CUSTODY.md) for the full caller, authorization, asset
movement, state, event, and failure matrix.
