use crate::Fractional;

pub trait LiquiditySource {
    /// Fee for swapping tokens on pool.
    fn fee() -> Fractional;
    /// Amount of active pool tokens.
    fn pool_token_total_supply() -> Fractional;
    /// Amount of base tokens in the pool.
    fn base_asset_reserve() -> Fractional;
    /// Amount of target tokens in the pool.
    fn target_asset_reserve() -> Fractional;
    // fn add_liquidity, remove_liquidity...
}
