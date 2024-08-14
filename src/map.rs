use crate::hash::Hash;

use std::mem::{self, MaybeUninit};
use std::ptr;
use std::marker::PhantomData;

// hashmap
pub struct Map<K, V> {
    ptr:    *mut Bucket<K, V>,
    size:   usize,
    level:  u8,
    _owned: PhantomData<(K, V)>,
}

struct Bucket<K, V> {
    hash: u64,
    salt: u16,
    key:  K,
    val:  V,
}

impl<V, K: Hash> Bucket<K, V> {
    fn uninit() -> Self {
        Self {
            salt: 0,
            hash: 0,
            key:  unsafe { MaybeUninit::uninit().assume_init() },
            val:  unsafe { MaybeUninit::uninit().assume_init() },
        }
    }
}

impl<V, K: Hash> Map<K, V> {
    fn new() -> Self {
        Self {
            ptr:    ptr::null_mut(),
            size:   0,
            level:  0,
            _owned: PhantomData,
        }
    }

    pub fn hash(&self, key: &K) -> usize {
        todo!()
    }

    pub fn insert(&mut self, key: K, val: V) {
        let hash  = self.hash(&key);
    }
}
