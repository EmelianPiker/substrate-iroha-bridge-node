//! # Usage Example (will be removed)
//!
//! `pallet-xyk-pool`:
//! ```
//! use common::Fractional;
//! struct XYKPool {
//! //    ...
//! }
//!
//! impl common::LiquiditySource for XYKPool {
//!     fn fee() -> Fractional {
//!         unimplemented!()
//!     }
//!
//!     fn pool_token_total_supply() -> Fractional {
//!         unimplemented!()
//!     }
//!
//!     fn base_asset_reserve() -> Fractional {
//!         unimplemented!()
//!     }
//!
//!     fn target_asset_reserve() -> Fractional {
//!         unimplemented!()
//!     }
//! }
//! ```
//!
//! `pallet-liquidity-proxy`:
//! ```
//! enum LiquiditySourceType {
//!     XYKPool,
//!     Orderbook,
//! }
//!
//! enum LiquiditySourceData {
//!     XYKPool(pallet_xyk_pool::XYKPool),
//!     Orderbook(pallet_orderbook::Orderbook),
//! }
//!
//! decl_storage! {
//!     LiquiditySources: double_map DexId => TradingPairId => BTreeMap<LiquiditySourceType, LiquiditySourceData>;
//! }
//!
//! impl Module<T> {
//!     fn select_source(...) -> LiquiditySourceData {
//!         // go to storage, run select algo...
//!     }
//! }
//!
//! ```
//! `pallet-trading-pairs`
//! ```
//! decl_storage! {
//!     TradingPairs: map DexId => BTreeSet<TradingPair>;
//! }
//!
//! decl_module! {
//!     fn buy(origin, dex_id: DexId, tp: TradingPairId, amount: Fractional) {
//!         let liquidity_source = pallet_liquidity_proxy::select_source(...);
//!         ...
//!     }
//! }
//!
//! ```
//! `pallet-polkaswap`
//! ```
//! decl_storage! {
//!     DexesInfo: map DexId => DexInfo;
//! }
//!
//! decl_module! {
//!     fn buy(origin, dex_id: DexId, tp: TradingPairId, amount: Fractional) {
//!         pallet_trading_pairs::buy(...);
//!     }
//!
//!     fn register_dex() {
//!         <DexesInfo<T>>::insert(DexInfo);
//!     }
//! }
//! ```

use sp_arithmetic::FixedU128;

mod primitives;
mod traits;

pub use primitives::*;
pub use traits::*;

pub type Fractional = FixedU128;
pub type AssetId = String;
pub type DexId = String;
pub type Asset<T, GetAssetId> = currencies::Currency<T, GetAssetId>;
