use std::{cell::RefCell, thread::LocalKey};

use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};

use types::{models::AirdropTransfers, Proposal};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

/// The memory IDs for the different stores.
/// # Note
/// These IDs are used to identify the different stores in the `MemoryManager`.
/// # Warning
/// These IDs should not be changed. New IDs should be added to the end of the list
pub static WHITELIST_MEMORY_ID: MemoryId = MemoryId::new(0);

pub static TRANSFER_REQUESTS_MEMORY_ID: MemoryId = MemoryId::new(1);

pub static AIRDROP_TRANSFERS_MEMORY_ID: MemoryId = MemoryId::new(2);

pub static PROPOSALS_MEMORY_ID: MemoryId = MemoryId::new(3);

/// A reference to a `StableBTreeMap` that is wrapped in a `RefCell`.
///# Generics
/// * `K` - The key type of the `StableBTreeMap`.
/// * `V` - The value type of the `StableBTreeMap`.
pub type StorageRef<K, V> = RefCell<StableBTreeMap<K, V, Memory>>;
pub type StaticStorageRef<K, V> = &'static LocalKey<StorageRef<K, V>>;
type MemoryManagerStorage = RefCell<MemoryManager<DefaultMemoryImpl>>;

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static WHITELIST: StorageRef<u64,Principal> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(WHITELIST_MEMORY_ID)))
    );

    pub static AIRDROP_TRANSFERS: StorageRef<u64, AirdropTransfers> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(AIRDROP_TRANSFERS_MEMORY_ID)))
    );

    pub static PROPOSALS: StorageRef<u64, Proposal> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(PROPOSALS_MEMORY_ID)))
    );
}
