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
	use xcm_executor::traits::WeightBounds;

use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use log::info;
	use sp_runtime::traits::Convert;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Origin: From<<Self as SystemConfig>::Origin>
			+ Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

		type Call: From<Call<Self>> + Encode;

		type XcmSender: SendXcm;
		type XcmExecutor: ExecuteXcm<<Self as pallet::Config>::Call>;
		// type WeightInfo: pallet_automation_time::WeightInfo;
		type AccountIdToMultiLocation: Convert<Self::AccountId, MultiLocation>;

		type Weigher: WeightBounds<<Self as pallet::Config>::Call>;
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
	pub enum Error<T> {
		// TODO: expand into XcmExecutionFailed(XcmError) after https://github.com/paritytech/substrate/pull/10242 done
		/// XCM execution failed.
		XcmExecutionFailed,
	}

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
			let call = xcm_test::OakChainCallBuilder::automation_time_schedule_notify(provided_id, vec![time], message);

			let transact_instruction = Transact::<()> {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: 6_000_000_000,
				call: call.encode().into(),
			};
			let asset = MultiAsset {
				id: Concrete(MultiLocation::here()),
				fun: Fungibility::Fungible(7_000_000_000),
			};
			let buy_execution_weight_instruction = BuyExecution::<()> {
				fees: asset.clone(),
				weight_limit: Limited(10_000_000),
			};
			let multiassets: MultiAssets = vec![asset.clone()].into();
			let withdraw_asset_instruction = WithdrawAsset::<()>(multiassets);

			let refund_multilocation = MultiLocation {
				parents: 0,
				interior: X1(AccountId32 {
					network: Any,
					id: [0u8; 32],
				}),
			};
			let refund_surplus_instruction = RefundSurplus::<()>;
			let deposit_asset_instruction = DepositAsset::<()> {
				assets: MultiAssetFilter::Wild(All),
				max_assets: 1,
				beneficiary: refund_multilocation.clone(),
			};
			let xcm_instruction_weight_set = Xcm(vec![
				withdraw_asset_instruction.clone(),
				buy_execution_weight_instruction,
				transact_instruction.clone(),
				refund_surplus_instruction.clone(),
				deposit_asset_instruction.clone(),
			]);

			let weight = T::Weigher::weight(&mut xcm_instruction_weight_set.clone().into()).unwrap();
			info!("############# Custom Weight: {:?}", weight);

			let buy_execution_instruction = BuyExecution::<()> {
				fees: asset.clone(),
				weight_limit: Limited(weight),
			};

			let xcm_instruction_set = Xcm(vec![
				withdraw_asset_instruction,
				buy_execution_instruction,
				transact_instruction,
				refund_surplus_instruction,
				deposit_asset_instruction,
			]);

			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(para.into())),
				xcm_instruction_set,
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
		pub fn send_schedule_notify_in_unit(origin: OriginFor<T>, para: ParaId, provided_id: Vec<u8>, time: u64, message: Vec<u8>) -> DispatchResult {
			// ensure_root(origin)?;
			let who = ensure_signed(origin)?;

			let call = xcm_test::OakChainCallBuilder::automation_time_schedule_notify(provided_id, vec![time], message);

			let transact_instruction = Transact::<()> {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: 1_000_000_000,
				call: call.encode().into(),
			};
			let buy_execution_weight_instruction = BuyExecution::<()> {
				fees: MultiAsset {
					id: Concrete(MultiLocation::here()),
					fun: Fungibility::Fungible(9_600_000_000_000),
				},
				weight_limit: Limited(10_000_000),
			};
			// let sovereign_account: <T as frame_system::Config>::AccountId = 1;
			let id = who
				.using_encoded(|mut d| <[u8; 32]>::decode(&mut d))
				.map_err(|_| Error::<T>::XcmExecutionFailed)?;
			let refund_multilocation = MultiLocation {
				parents: 0,
				interior: X1(AccountId32 {
					network: Any,
					id,
				}),
			};
			let refund_surplus_instruction = RefundSurplus::<()>;
			let deposit_asset_instruction = DepositAsset::<()> {
				assets: MultiAssetFilter::Wild(All),
				max_assets: 1,
				beneficiary: refund_multilocation.clone(),
			};
			let xcm_instruction_weight_set = Xcm(vec![
				buy_execution_weight_instruction,
				transact_instruction.clone(),
				refund_surplus_instruction.clone(),
				deposit_asset_instruction.clone(),
			]);

			let weight = T::Weigher::weight(&mut xcm_instruction_weight_set.clone().into()).unwrap();
			info!("############# Custom Weight: {:?}", weight);

			let buy_execution_instruction = BuyExecution::<()> {
				fees: MultiAsset {
					id: Concrete(MultiLocation {
						parents: 1,
						interior: X1(Parachain(2001)),
					}),
					fun: Fungibility::Fungible(9_600_000_000_000),
				},
				weight_limit: Limited(weight),
			};

			let recipient_xcm_instruction_set = Xcm(vec![
				buy_execution_instruction,
				transact_instruction,
				refund_surplus_instruction,
				deposit_asset_instruction,
			]);

			let multiassets: MultiAssets = vec![MultiAsset {
				id: Concrete(MultiLocation::here()),
				fun: Fungibility::Fungible(9_700_000_000_000),
			}].into();
			let withdraw_asset_instruction = WithdrawAsset::<()>(multiassets);
			let internal_instruction_set = Xcm(vec![
				withdraw_asset_instruction,
				DepositReserveAsset {
					assets: MultiAssetFilter::Definite(vec![MultiAsset {
						id: Concrete(MultiLocation::here()),
						fun: Fungibility::Fungible(9_700_000_000_000),
					}].into()),
					max_assets: 1,
					dest: MultiLocation {
						parents: 1,
						interior: X1(Parachain(para.into())),
					},
					xcm: recipient_xcm_instruction_set,
				}
			]);

			let xcm_origin = T::AccountIdToMultiLocation::convert(who);

			T::XcmExecutor::execute_xcm_in_credit(
				xcm_origin,
				internal_instruction_set.into(),
				2_000_000_000,
				2_000_000_000
			).ensure_complete()
			.map_err(|error| {
				log::error!("Failed execute transfer message with {:?}", error);
				Error::<T>::XcmExecutionFailed
			})?;
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn send_schedule_xcmp(origin: OriginFor<T>, para: ParaId, provided_id: Vec<u8>, time: u64) -> DispatchResult {
			ensure_root(origin)?;

			let call_name = b"automation_time_schedule".to_vec();

			let inner_call = xcm_test::TestChainCallBuilder::remark_with_event(b"heya".to_vec());
			let call = xcm_test::OakChainCallBuilder::automation_time_schedule_xcmp(2001.into(), provided_id, time, inner_call.encode());

			let transact_instruction = Transact::<()> {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: 1_000_000_000,
				call: call.encode().into(),
			};
			let asset = MultiAsset {
				id: Concrete(MultiLocation::here()),
				fun: Fungibility::Fungible(2_000_000_000),
			};
			let buy_execution_weight_instruction = BuyExecution::<()> {
				fees: asset.clone(),
				weight_limit: Limited(10_000_000),
			};
			let multiassets: MultiAssets = vec![asset.clone()].into();
			let withdraw_asset_instruction = WithdrawAsset::<()>(multiassets);
			let xcm_instruction_weight_set = Xcm(vec![
				withdraw_asset_instruction.clone(),
				buy_execution_weight_instruction,
				transact_instruction.clone(),
			]);

			let weight = T::Weigher::weight(&mut xcm_instruction_weight_set.clone().into()).unwrap();
			info!("############# Custom Weight: {:?}", weight);

			let buy_execution_instruction = BuyExecution::<()> {
				fees: asset.clone(),
				weight_limit: Limited(weight),
			};

			let xcm_instruction_set = Xcm(vec![
				withdraw_asset_instruction,
				buy_execution_instruction,
				transact_instruction,
			]);

			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(para.into())),
				xcm_instruction_set,
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
