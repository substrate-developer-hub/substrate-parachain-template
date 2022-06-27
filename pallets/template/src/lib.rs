#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

use cumulus_pallet_xcm::{ensure_sibling_para, Origin as CumulusOrigin};
use cumulus_primitives_core::ParaId;
use frame_system::Config as SystemConfig;
use polkadot_parachain::primitives::Sibling;
use sp_runtime::traits::{AccountIdConversion, Convert};
use sp_std::prelude::*;
use xcm::latest::prelude::*;

pub use pallet::*;
use oak_xcm::XcmInstructionGenerator;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {

use super::*;
	use frame_support::{pallet_prelude::*, traits::{ExistenceRequirement, Currency}};
	use frame_system::pallet_prelude::*;
	use log::info;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Origin: From<<Self as SystemConfig>::Origin>
			+ Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

		type Call: From<Call<Self>> + Encode;

		type XcmSender: SendXcm;
		type XcmExecutor: ExecuteXcm<<Self as pallet::Config>::Call>;

		type OakXcmInstructionGenerator: XcmInstructionGenerator<Self>;
		type Currency: Currency<Self::AccountId>;
		type SelfParaId: Get<ParaId>;
		type OakAutomationParaId: Get<ParaId>;
		type AccountIdToMultiLocation: Convert<Self::AccountId, MultiLocation>;
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
		// Invalid Refund Address
		InvalidRefundAddress,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/**
		 * This function wraps the currency transfer private function that can move tokens from wallet to wallet. 
		 * The intention is for this function to be called by the `Transact` XCM instruction when Turing calls back to this chain.
		 * While we are using transfer, this is just an example and any private function can be used to substitute.
		 *
		 * By calling `ensure_sibling_para`, we can ensure that only sibling parachains will be able to call and returns the para ID.
		 * Using this parachain ID, the user can make sure that the para ID is whitelisted.
		 */
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn delayed_transfer(origin: OriginFor<T>, source: T::AccountId, dest: T::AccountId, value: BalanceOf<T>) -> DispatchResult {
			let origin_para_id: ParaId = ensure_sibling_para(<T as Config>::Origin::from(origin))?;
			info!("Send balance on a delayed transfer, source: {:?}, dest: {:?}, value: {:?}, origin_para_id: {:?}", source, dest, value, origin_para_id);
			<T as Config>::Currency::transfer(
				&source,
				&dest,
				value,
				ExistenceRequirement::AllowDeath,
			)?;
			Ok(())
		}

		/**
		 * This function implements XCM call to OAK with the OAK XCM crate. It uses the `delayed_transfer` extrinsic above.
		 * We can create an XCMP instruction with that call wrapped in the instructions to be sent back to this chain.
		 * This implementation withdraws assets for the fee from the sovereign account of this chain on Turing.
		 * Therefore, TUR tokens must be available for this sovereign account on the Turing chain. 
		 *
		 * NOTE: 7_000_000_000 as the fungible asset amount being withdrawn is just a temporary measure until fees are put into the crate.
		 */
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn send_schedule_xcmp_with_crate(
			origin: OriginFor<T>,
			provided_id: Vec<u8>,
			execution_times: Vec<u64>,
			dest: T::AccountId,
			value: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let tur_para_id: ParaId = ParaId::from(T::OakAutomationParaId::get());
			let self_para_id: ParaId = T::SelfParaId::get();
			let call_name = b"automation_time_schedule_xcmp_with_crate".to_vec();
			let inner_call = <T as Config>::Call::from(Call::<T>::delayed_transfer { source: who.clone(), dest, value })
				.encode()
				.into();
			let transact_instruction =
				T::OakXcmInstructionGenerator::create_schedule_xcmp_instruction(provided_id, execution_times, self_para_id, inner_call);
			let asset = MultiAsset {
				id: Concrete(MultiLocation::here()),
				fun: Fungibility::Fungible(7_000_000_000),
			};

			let refund_account = match AccountIdConversion::<T::AccountId>::try_into_account(&Sibling::from(self_para_id)) {
				Some(para_id) => {
					para_id
				},
				None => {
					Err(Error::<T>::InvalidRefundAddress)?
				}
			};
			let xcm_instruction_set = T::OakXcmInstructionGenerator::create_xcm_instruction_set(asset, transact_instruction, refund_account);

			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(tur_para_id.into())),
				xcm_instruction_set,
			) {
				Ok(()) => {
					Self::deposit_event(Event::CallSent(call_name));
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorSendingCall(e, tur_para_id, call_name));
				}
			};
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn send_schedule_notify_in_unit(
			origin: OriginFor<T>,
			provided_id: Vec<u8>,
			execution_times: Vec<u64>,
			dest: T::AccountId,
			value: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let tur_para_id: ParaId = ParaId::from(T::OakAutomationParaId::get());
			let self_para_id: ParaId = T::SelfParaId::get();
			let call_name = b"automation_time_schedule_xcmp_in_unit".to_vec();

			let inner_call = <T as Config>::Call::from(Call::<T>::delayed_transfer { source: who.clone(), dest, value })
				.encode()
				.into();

			let transact_instruction =
				T::OakXcmInstructionGenerator::create_schedule_xcmp_instruction(provided_id, execution_times, self_para_id, inner_call);

			let refund_surplus_instruction = RefundSurplus::<()>;

			let buy_execution_instruction = BuyExecution::<()> {
				fees: MultiAsset {
					id: Concrete(MultiLocation {
						parents: 1,
						interior: X1(Parachain(self_para_id.into())),
					}),
					fun: Fungibility::Fungible(10_000_000_000),
				},
				weight_limit: Unlimited,
			};
			let multiassets: MultiAssets = vec![MultiAsset {
				id: Concrete(MultiLocation {
					parents: 1,
					interior: X1(Parachain(self_para_id.into())),
				}),
				fun: Fungibility::Fungible(10_000_000_000),
			}].into();
			let deposit_asset_instruction = DepositAsset::<()> {
				assets: MultiAssetFilter::Definite(multiassets.clone()),
				max_assets: 1,
				beneficiary: MultiLocation {
					parents: 1,
					interior: X1(Parachain(tur_para_id.into())),
				},
			};
			// let interior: Junctions = T::AccountIdToMultiLocation::convert(who.clone())
      //           .clone()
      //           .try_into()
      //           .map_err(|_| Error::<T>::XcmExecutionFailed)?;
			// let descend_origin_instruction = DescendOrigin::<()>(interior);
			let reserve_asset_instruction = ReserveAssetDeposited::<()>(multiassets);
			let recipient_xcm_instruction_set = Xcm(vec![
				reserve_asset_instruction,
				buy_execution_instruction,
				// descend_origin_instruction,
				transact_instruction,
				refund_surplus_instruction,
				deposit_asset_instruction,
			]);

			let withdraw_asset_instruction = WithdrawAsset::<()>(vec![MultiAsset {
				id: Concrete(MultiLocation::here()),
				fun: Fungibility::Fungible(10_000_000_000),
			}].into());
			let internal_instruction_set = Xcm(vec![
				withdraw_asset_instruction,
				DepositAsset {
					assets: MultiAssetFilter::Definite(vec![MultiAsset {
						id: Concrete(MultiLocation::here()),
						fun: Fungibility::Fungible(10_000_000_000),
					}].into()),
					max_assets: 1,
					beneficiary: MultiLocation {
						parents: 1,
						interior: X1(Parachain(tur_para_id.into())),
					},
				}
			]);

			let xcm_origin = T::AccountIdToMultiLocation::convert(who);

			T::XcmExecutor::execute_xcm_in_credit(
				xcm_origin.clone(),
				internal_instruction_set.into(),
				11_000_000_000,
				11_000_000_000
			).ensure_complete()
			.map_err(|error| {
				log::error!("Failed execute transfer message with {:?}", error);
				Error::<T>::XcmExecutionFailed
			})?;

			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(tur_para_id.into())),
				recipient_xcm_instruction_set,
			) {
				Ok(()) => {
					Self::deposit_event(Event::CallSent(call_name));
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorSendingCall(e, tur_para_id, call_name));
				}
			};

			Ok(())
		}
	}
}
