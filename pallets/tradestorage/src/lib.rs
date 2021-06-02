#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

pub use tradestorage::*;

#[frame_support::pallet]
pub mod tradestorage {
	use frame_support::{
		pallet_prelude::*,
		dispatch::DispatchResult, 
		traits::{Get},
		dispatch::Vec, 
		codec::{Encode, Decode}
	};
	use frame_system::pallet_prelude::BlockNumberFor;
	use frame_system::pallet_prelude::OriginFor;
	use frame_system::ensure_root;
	use sp_runtime::traits::{
		Block as BlockT, IdentifyAccount, Verify,
	};
	use sp_runtime::MultiSignature;

	pub type Signature = MultiSignature;
	
	pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[derive(Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
	pub struct TradeStruct<AccountId> {
		trade_id: Vec<u8>,
		buyer: AccountId,
		seller: AccountId,
		energy: u32,
		rate: u32
	}

	#[derive(Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
	pub struct MarketTradeStruct<AccountId> {
		market_slot: u32,
		trades: TradeStruct<AccountId>,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn trade_map)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub(super) type TradeMap<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, Option<(AccountId, AccountId, u32, u32)>, ValueQuery>;


    #[pallet::storage]
	#[pallet::getter(fn simulation_market_map)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub(super) type SimulationMarketMap<T> = StorageMap<_, Blake2_128Concat, (Vec<u8>, Vec<u8>), MarketTradeStruct<AccountId>, ValueQuery>;
	
	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	// #[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        TradeMapStored(Vec<u8>, Vec<u8>, u32, Vec<u8>, AccountId, AccountId, u32, u32),
	}


	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_trade_map(origin:OriginFor<T>, simulation_id:Vec<u8>, market_id:Vec<u8>, market_slot: u32, trade_id:Vec<u8>, buyer: AccountId, seller: AccountId, energy: u32, rate: u32) -> DispatchResult {
			let _caller = ensure_root(origin)?;
			let _buyer = buyer.clone();
			let _seller = seller.clone();
			let _trade_id = trade_id.clone();
			let trade_struct = TradeStruct {trade_id: _trade_id.clone(), buyer: _buyer.clone(), seller: _seller.clone(), energy, rate};
			let market_struct = MarketTradeStruct {market_slot: market_slot, trades: trade_struct};
			<TradeMap<T>>::insert(&trade_id, Some((_buyer.clone(), _seller.clone(), energy, rate)));
			<SimulationMarketMap<T>>::insert((&simulation_id, &market_id), market_struct);
			Self::deposit_event(Event::TradeMapStored(simulation_id, market_id, market_slot, trade_id, _buyer, _seller, energy, rate));
			Ok(())
		}
	}
}

