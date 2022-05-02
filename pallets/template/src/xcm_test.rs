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
pub enum AutomationTimeCall {
    #[codec(index = 0)]
    ScheduleNotifyTask(Vec<u8>, u64, Vec<u8>),
    #[codec(index = 2)]
    ScheduleXcmpTask(ParaId, Vec<u8>, u64, Vec<u8>),
}


#[derive(Encode, Decode, RuntimeDebug)]
pub enum NeuChainCall {
    #[codec(index = 0)]
    System(SystemCall),
    #[codec(index = 60)]
    AutomationTime(AutomationTimeCall),
}

#[derive(Encode, Decode, RuntimeDebug)]
pub enum TestChainCall {
    #[codec(index = 0)]
    System(SystemCall),
}

pub struct OakChainCallBuilder;

impl OakChainCallBuilder {
    pub fn remark_with_event(message: Vec<u8>) -> NeuChainCall {
        NeuChainCall::System(SystemCall::RemarkWithEvent(message))
    }

    pub fn automation_time_schedule_notify(provided_id: Vec<u8>, time: u64, message: Vec<u8>) -> NeuChainCall {
        NeuChainCall::AutomationTime(AutomationTimeCall::ScheduleNotifyTask(provided_id, time, message))
    }

    pub fn automation_time_schedule_xcmp(para_id: ParaId, provided_id: Vec<u8>, time: u64, call: Vec<u8>) -> NeuChainCall {
        NeuChainCall::AutomationTime(AutomationTimeCall::ScheduleXcmpTask(para_id, provided_id, time, call))
    }
}

pub struct TestChainCallBuilder;

impl TestChainCallBuilder {
    pub fn remark_with_event(message: Vec<u8>) -> TestChainCall {
        TestChainCall::System(SystemCall::RemarkWithEvent(message))
    }
}