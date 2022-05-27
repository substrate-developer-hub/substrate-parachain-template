#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use sp_std::prelude::*;
use frame_support::RuntimeDebug;
use cumulus_primitives_core::ParaId;

#[derive(Encode, Decode, RuntimeDebug)]
pub enum SystemCall {
    #[codec(index = 8)]
    RemarkWithEvent(Vec<u8>),
}

#[derive(Encode, Decode, RuntimeDebug)]
pub enum AutomationTimeCall<T: frame_system::Config> {
    #[codec(index = 0)]
    ScheduleNotifyTask(Vec<u8>, Vec<u64>, Vec<u8>),
    #[codec(index = 2)]
    ScheduleXcmpTask(Vec<u8>, Vec<u64>, ParaId, Vec<u8>, u64),
    #[codec(index = 3)]
    ScheduleNativeTransferTask(Vec<u8>, Vec<u64>, T::AccountId, u128),
    #[codec(index = 4)]
    ScheduleCancelTask(T::Hash),
}


#[derive(Encode, Decode, RuntimeDebug)]
pub enum NeuChainCall<T: frame_system::Config> {
    #[codec(index = 0)]
    System(SystemCall),
    #[codec(index = 60)]
    AutomationTime(AutomationTimeCall<T>),
}

#[derive(Encode, Decode, RuntimeDebug)]
pub enum TestChainCall {
    #[codec(index = 0)]
    System(SystemCall),
}

pub struct OakChainCallBuilder;

impl OakChainCallBuilder {
    pub fn automation_time_schedule_notify<T: frame_system::Config>(provided_id: Vec<u8>, times: Vec<u64>, message: Vec<u8>) -> NeuChainCall<T> {
        NeuChainCall::AutomationTime(AutomationTimeCall::<T>::ScheduleNotifyTask(provided_id, times, message))
    }

    pub fn automation_time_schedule_xcmp<T: frame_system::Config>(
        provided_id: Vec<u8>,
        execution_times: Vec<u64>,
        para_id: ParaId,
        returnable_call: Vec<u8>,
        weight_at_most: u64,
    ) -> NeuChainCall<T> {
        NeuChainCall::AutomationTime(AutomationTimeCall::<T>::ScheduleXcmpTask(
            provided_id,
            execution_times,
            para_id,
            returnable_call,
            weight_at_most,
        ))
    }

    pub fn automation_time_schedule_native_transfer<T: frame_system::Config>(
      provided_id: Vec<u8>,
      execution_times: Vec<u64>,
      recipient_id: T::AccountId,
      amount: u128
    ) -> NeuChainCall<T> {
        NeuChainCall::AutomationTime(AutomationTimeCall::<T>::ScheduleNativeTransferTask(provided_id, execution_times, recipient_id, amount))
    }

    pub fn automation_time_cancel_task<T: frame_system::Config>(task_id: T::Hash) -> NeuChainCall<T> {
        NeuChainCall::AutomationTime(AutomationTimeCall::<T>::ScheduleCancelTask(task_id))
    }
}
