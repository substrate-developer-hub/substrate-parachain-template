#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

use cumulus_pallet_xcm::{Origin as CumulusOrigin};
use cumulus_primitives_core::ParaId;
use frame_system::Config as SystemConfig;
use sp_std::prelude::*;
use xcm::latest::prelude::*;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod xcm_test;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Origin: From<<Self as SystemConfig>::Origin>
			+ Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

		type Call: From<Call<Self>> + Encode;

		type XcmSender: SendXcm;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// [call]
		CallSent(Vec<u8>),
		/// [error, paraId, call]
		ErrorSendingCall(SendError, ParaId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn send_remark_with_event(origin: OriginFor<T>, para: ParaId, message: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;

			let call_name = b"remark_with_event".to_vec();
			let remark = xcm_test::OakChainCallBuilder::remark_with_event(message);

			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(para.into())),
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: 10_000_000_000,
					call: remark.encode().into(),
				}]),
			) {
				Ok(()) => {
					Self::deposit_event(Event::CallSent(call_name));
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorSendingCall(e, para, call_name));
				}
			};
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn send_schedule_notify(origin: OriginFor<T>, para: ParaId, provided_id: Vec<u8>, time: u64, message: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;

			let call_name = b"automation_time_schedule_notify".to_vec();
			let call = xcm_test::OakChainCallBuilder::automation_time_schedule_notify(provided_id, time, message);

			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(para.into())),
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: 10_000_000_000,
					call: call.encode().into(),
				}]),
			) {
				Ok(()) => {
					Self::deposit_event(Event::CallSent(call_name));
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorSendingCall(e, para, call_name));
				}
			};
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn send_schedule_xcmp(origin: OriginFor<T>, para: ParaId, provided_id: Vec<u8>, time: u64) -> DispatchResult {
			ensure_root(origin)?;

			let call_name = b"automation_time_schedule".to_vec();

			let inner_call = xcm_test::TestChainCallBuilder::remark_with_event(b"heya".to_vec());
			let call = xcm_test::OakChainCallBuilder::automation_time_schedule_xcmp(2001.into(), provided_id, time, inner_call.encode());

			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(para.into())),
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: 10_000_000_000,
					call: call.encode().into(),
				}]),
			) {
				Ok(()) => {
					Self::deposit_event(Event::CallSent(call_name));
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorSendingCall(e, para, call_name));
				}
			};
			Ok(())
		}
	}
}
