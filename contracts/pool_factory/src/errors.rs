use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoolFactoryError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    AssetAlreadyRegistered = 3,
    /// `register_asset` could not read the registry contract from the invoice
    /// contract it was handed, because that invoice contract has not been
    /// initialized (or is not an invoice contract at all).
    InvoiceNotInitialized = 4,
    /// The pool address is malformed or unreachable: it does not expose a
    /// callable `get_funding_asset`, or the call failed/trapped.
    InvalidPool = 5,
    /// The pool answered `get_funding_asset` but reported an asset different
    /// from the `asset` the caller declared. Registration is rejected so the
    /// factory never records a mapping whose asset and pool disagree.
    FundingAssetMismatch = 6,
}
