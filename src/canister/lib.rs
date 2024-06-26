use candid::Principal;
use ic_cdk::{caller, init, query};
use logic::WhitelistLogic;
use storage::{metadata_storage::MetadataStorage, CellStorage};
use types::Metadata;

pub mod helpers;
pub mod logic;
pub mod result;
pub mod storage;

pub mod calls;

#[init]
pub fn init(owner: Principal, whitelisted: Vec<Principal>, proxy: Principal, group_id: u64) {
    MetadataStorage::set(Metadata::new(group_id, proxy, caller())).expect("Failed to set metadata");
    WhitelistLogic::init(owner, whitelisted)
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use crate::result::CanisterResult;
    use types::ProposalResponse;
    use types::{AirdropTransfers, Content, ProposalEntry, Status, VoteKind, VotesEntry};

    use candid::export_service;
    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = dir.parent().unwrap().join("../candid");
    write(dir.join("multisig.did"), __export_did_tmp_()).expect("Write failed.");
}
