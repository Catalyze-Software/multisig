use std::{cell::RefCell, collections::HashMap, time::Duration};

use candid::{CandidType, Principal};

use serde::Deserialize;

use crate::rust_declarations::types::{TransactionRequestData, Votes, WhitelistRequestData};

pub static DAY_IN_NANOS: u64 = Duration::from_secs(1 * 24 * 60 * 60).as_nanos() as u64;

#[derive(Deserialize, CandidType)]
pub struct Store {
    pub owner: Principal,
    pub whitelist: Vec<Principal>,

    pub transaction_request_id: u32,
    pub transaction_requests: HashMap<u32, TransactionRequestData>,
    pub transaction_request_expiry: u64,

    pub whitelist_request_id: u32,
    pub whitelist_requests: HashMap<u32, WhitelistRequestData>,
    pub whitelist_request_expiry: u64,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            owner: Principal::anonymous(),
            whitelist: Default::default(),

            transaction_request_id: 0,
            transaction_requests: Default::default(),
            transaction_request_expiry: DAY_IN_NANOS,

            whitelist_request_id: 0,
            whitelist_requests: Default::default(),
            whitelist_request_expiry: DAY_IN_NANOS,
        }
    }
}

thread_local! {
    pub static DATA: RefCell<Store>  = RefCell::new(Store::default());
}

impl Store {
    pub fn init(owner: Principal) {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.owner = owner.clone();
            data.whitelist.push(owner);
        });
    }

    pub fn is_whitelisted(caller: &Principal) -> Result<(), String> {
        DATA.with(|data| {
            let data = &data.borrow();
            if data.whitelist.contains(caller) || caller == &data.owner {
                Ok(())
            } else {
                Err("Caller is not whitelisted".to_string())
            }
        })
    }

    pub fn check_duplicate_vote(caller: &Principal, votes: &Votes) -> Result<(), String> {
        if votes.approvals.contains(caller) {
            return Err("Approval vote already cast".to_string());
        } else if votes.rejections.contains(caller) {
            return Err("Rejection vote already cast".to_string());
        } else {
            Ok(())
        }
    }
}