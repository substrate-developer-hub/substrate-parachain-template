#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use verifier::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod verifier {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
    use frame_system::ensure_root;
    use frame_support::{
        traits::{Get}, 
        codec::{Encode, Decode},
        dispatch::Vec
    };

    #[derive(Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
    pub struct Bid {
        uuid: u32,
        market_uuid: Option<Vec<u8>>,
        asset_uuid: Option<Vec<u8>>,
        max_energy: u32,
        time_slot: Vec<u8>, 
    }

    #[derive(Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
    pub struct Offer {
        uuid: u32,
        market_uuid: Vec<u8>,
        asset_uuid: Vec<u8>,
        energy_type: Vec<u8>,
        max_energy: u32,
        time_slot: Vec<u8>, 
    }

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn valid_trades)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub(super) type ValidTrades<T> = StorageMap<_, Blake2_128Concat, u32, bool, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        TradeIdStored(u32),
        MatchVerified(Bid, Offer),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
	    #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        fn verify_match(origin:OriginFor<T>, bid_uuid:u32, bid_market_uuid:Vec<u8>, bid_asset_uuid:Vec<u8>, bid_max_energy:u32, bid_time_slot:Vec<u8>,
            offer_uuid:u32, offer_market_uuid:Vec<u8>, offer_asset_uuid:Vec<u8>, offer_energy_type:Vec<u8>, offer_max_energy:u32, offer_time_slot:Vec<u8>) -> DispatchResult {
            let _who = ensure_root(origin)?;
            let bid = Bid {uuid:bid_uuid , market_uuid:Some(bid_market_uuid.clone()), asset_uuid:Some(bid_asset_uuid.clone()), max_energy:bid_max_energy, time_slot:bid_time_slot.clone()};
            let offer = Offer {uuid: offer_uuid, market_uuid:offer_market_uuid.clone(), asset_uuid:offer_asset_uuid.clone(), energy_type:offer_energy_type.clone(), max_energy:offer_max_energy, time_slot:offer_time_slot.clone()};
            let offer_clone = offer.clone();
            assert_eq!(bid.uuid, offer_clone.uuid);
            assert_eq!(bid.market_uuid, Some(offer_clone.market_uuid));
            assert_eq!(bid.asset_uuid, Some(offer_clone.asset_uuid));
            let trade_id = bid.uuid.clone();
            <ValidTrades<T>>::insert(&trade_id, true);
            Self::deposit_event(Event::MatchVerified(bid, offer));
            Ok(())
        }
	}
}
