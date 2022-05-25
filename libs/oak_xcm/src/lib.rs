#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

use cumulus_primitives_core::ParaId;
use sp_std::prelude::*;
use xcm::latest::prelude::*;
use frame_support::{
	pallet_prelude::*,
};
use xcm_executor::traits::WeightBounds;
// use log::info;
use sp_runtime::traits::Convert;
mod xcm_config;

pub trait XcmInstructionGenerator<T: frame_system::Config> {
	// Transact instruction creation
	fn create_schedule_native_transfer_instruction(
		provided_id: Vec<u8>,
		execution_times: Vec<u64>,
		recipient_id: T::AccountId,
		amount: u128
	) -> xcm::v2::Instruction<()>;

	fn create_schedule_notify_instruction(
		provided_id: Vec<u8>,
		execution_times: Vec<u64>,
		message: Vec<u8>,
	) -> xcm::v2::Instruction<()>;

	fn create_schedule_xcmp_instruction(para_id: ParaId, provided_id: Vec<u8>, time: u64, returnable_call: Vec<u8>) -> xcm::v2::Instruction<()>;

	fn create_cancel_task_instruction(task_id: T::Hash) -> xcm::v2::Instruction<()>;

	// Generic Instruction Creation
	fn create_xcm_instruction_set(
		asset: MultiAsset,
		transact_instruction: xcm::v2::Instruction<()>,
		refund_account: T::AccountId
	) -> xcm::v2::Xcm<()>;
}

pub struct OakXcmInstructionGenerator<A, W>(PhantomData<(A, W)>);

impl <T, A, W> XcmInstructionGenerator<T> for OakXcmInstructionGenerator<A, W> 
where 
	T: frame_system::Config,
	A: Convert<T::AccountId, [u8; 32]>,
	W: WeightBounds<<T as frame_system::Config>::Call>,
{
	// Transact instruction creation
	fn create_schedule_native_transfer_instruction(
		provided_id: Vec<u8>,
		execution_times: Vec<u64>,
		recipient_id: T::AccountId,
		amount: u128
	) -> xcm::v2::Instruction<()> {
		// let call_name = b"automation_time_schedule_native_transfer".to_vec();
		let call = xcm_config::OakChainCallBuilder::automation_time_schedule_native_transfer::<T>(
			provided_id, execution_times, recipient_id, amount
		);

		Transact::<()> {
			origin_type: OriginKind::SovereignAccount,
			require_weight_at_most: 6_000_000_000,
			call: call.encode().into(),
		}
	}

	fn create_schedule_notify_instruction(
		provided_id: Vec<u8>,
		execution_times: Vec<u64>,
		message: Vec<u8>,
	) -> xcm::v2::Instruction<()> {
		// let call_name = b"automation_time_schedule_notify".to_vec();
		let call = xcm_config::OakChainCallBuilder::automation_time_schedule_notify::<T>(provided_id, execution_times, message);

		Transact::<()> {
			origin_type: OriginKind::SovereignAccount,
			require_weight_at_most: 6_000_000_000,
			call: call.encode().into(),
		}
	}

	fn create_schedule_xcmp_instruction(para_id: ParaId, provided_id: Vec<u8>, time: u64, returnable_call: Vec<u8>) -> xcm::v2::Instruction<()> {
		// let call_name = b"automation_time_schedule_xcmp".to_vec();
		let call = xcm_config::OakChainCallBuilder::automation_time_schedule_xcmp::<T>(para_id, provided_id, time, returnable_call);

		Transact::<()> {
			origin_type: OriginKind::SovereignAccount,
			require_weight_at_most: 6_000_000_000,
			call: call.encode().into(),
		}
	}

	fn create_cancel_task_instruction(task_id: T::Hash) -> xcm::v2::Instruction<()> {
		// let call_name = b"automation_time_cancel_task".to_vec();
		let call = xcm_config::OakChainCallBuilder::automation_time_cancel_task::<T>(task_id);

		Transact::<()> {
			origin_type: OriginKind::SovereignAccount,
			require_weight_at_most: 6_000_000_000,
			call: call.encode().into(),
		}
	}

	// Generic Instruction Creation
	fn create_xcm_instruction_set(
		asset: MultiAsset,
		transact_instruction: xcm::v2::Instruction<()>,
		refund_account: T::AccountId
	) -> xcm::v2::Xcm<()> {
		let withdraw_asset_instruction = WithdrawAsset::<()>(vec![asset.clone()].into());
		let buy_execution_weight_instruction = BuyExecution::<()> { fees: asset.clone(), weight_limit: Unlimited };
		let refund_surplus_instruction = RefundSurplus::<()>;
		let deposit_asset_instruction = DepositAsset::<()> {
			assets: MultiAssetFilter::Wild(All),
			max_assets: 1,
			beneficiary: MultiLocation {
				parents: 0,
				interior: X1(AccountId32 {
					network: Any,
					id: A::convert(refund_account),
				}),
			},
		};

		let execution_weight = W::weight(&mut Xcm(vec![
			withdraw_asset_instruction.clone(),
			buy_execution_weight_instruction,
			transact_instruction.clone(),
			refund_surplus_instruction.clone(),
			deposit_asset_instruction.clone(),
		]).into()).unwrap();

		Xcm(vec![
			withdraw_asset_instruction,
			BuyExecution::<()> { fees: asset, weight_limit: Limited(execution_weight) },
			transact_instruction,
			refund_surplus_instruction,
			deposit_asset_instruction,
		])
	}
}
