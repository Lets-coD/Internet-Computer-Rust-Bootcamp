use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use std::sync::RwLock;
use std::borrow::BorrowMut;
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[derive(CandidType, Deserialize)]
struct Exam {
    out_of: u8,
    course: String,
    curve: u8,
}

impl Storable for Exam {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Manually implement Clone for Exam
impl Exam {
    // Custom cloned method
    fn clone(&self) -> Self {
        Self {
            out_of: self.out_of,
            course: self.course.clone(),
            curve: self.curve,
        }
    }
}


const MAX_VALUE_SIZE: u32 = 100;
impl BoundedStorable for Exam {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static EXAM_MAP: RefCell<StableBTreeMap<u64, Exam, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    static PARTICIPATION_PERCENTAGE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

#[ic_cdk_macros::query]
fn get_exam(key: u64) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow().get(&key))
}
#[ic_cdk_macros::query]
fn get_participation(key: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow().get(&key))
}


#[ic_cdk_macros::update]
fn insert_exam(key: u64, value: Exam) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow_mut().insert(key, value))
}
#[ic_cdk_macros::update]
fn insert_participation(key: u64, value: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow_mut().insert(key, value))
}


#[ic_cdk_macros::update]
fn update_exam(key: u64, new_exam: Exam) -> Option<Exam> {
    EXAM_MAP.with(|exam_map| {
        // Borrow the map mutably
        let mut map = exam_map.borrow_mut();

        // Check if the key exists in the map
        if let Some(old_exam) = map.get(&key) {
            // Replace the old value with the new one
            let result = map.insert(key, new_exam);

            // Return the old value
            result.or(Some(old_exam.clone()))
        } else {
            // If the key does not exist, return None
            None
        }
    })
}






