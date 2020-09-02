#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;
use common::DexId;

type TradingPair = common::TradingPair;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait + common::Trait> as TemplateModule {
        TradingPairs get(fn trading_pairs): map hasher(twox_64_concat) T::DexId => BTreeSet<TradingPair>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        DexId = <T as common::Trait>::DexId,
    {
        TradingPairStored(DexId, TradingPair),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
    }
}

decl_module! {
    pub struct Module<T: Trait + pswap::Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn register(origin, dex_id: <T as pswap::Trait>::DexId, base_asset_id: T::AssetId, target_asset_id: T::AssetId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            TradingPairs::insert(dex_id, TradingPair {
                base_asset_id,
                target_asset_id
            });
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }

        // #[weight = 10_000 + T::DbWeight::get().writes(1)]
        // pub fn buy(origin, ) -> dispatch::DispatchResult {
        //     let who = ensure_signed(origin)?;
        //     Something::put(something);
        //     Self::deposit_event(RawEvent::SomethingStored(something, who));
        //     Ok(())
        // }
    }
}
