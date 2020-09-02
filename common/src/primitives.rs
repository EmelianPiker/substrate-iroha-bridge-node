pub struct DexInfo<AccountId, AssetId> {
    pub owner_account_id: AccountId,
    pub base_asset_id: AssetId,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq)]
pub struct TradingPair<AssetId> {
    /// Base token of exchange.
    pub base_asset_id: AssetId,
    /// Target token of exchange.
    pub target_asset_id: AssetId,
}
