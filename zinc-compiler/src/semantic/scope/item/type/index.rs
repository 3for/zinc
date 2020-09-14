//!
//! The semantic analyzer scope type item index.
//!

use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::semantic::scope::builtin::BuiltInTypeId;

///
/// The type item index where the unique IDs for all declared types are recorded.
///
/// It is initialized with some built-in and standard library types.
///
/// The index treats type aliases equal to the type they point to.
///
pub struct Index {
    /// The inner type storage with the type unique ID as the key.
    pub inner: RwLock<HashMap<usize, String>>,
}

lazy_static! {
    pub static ref INDEX: Index = Index::new();
}

impl Index {
    /// The type hashmap default capacity.
    const INITIAL_CAPACITY: usize = 512;

    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        let index = Self {
            inner: RwLock::new(HashMap::with_capacity(Self::INITIAL_CAPACITY)),
        };
        index.next_with_id(
            "structure std::crypto::ecc::Point".to_owned(),
            BuiltInTypeId::StdCryptoEccPoint as usize,
        );
        index.next_with_id(
            "structure std::crypto::schnorr::Signature".to_owned(),
            BuiltInTypeId::StdCryptoSchnorrSignature as usize,
        );
        index.next_with_id(
            "structure std::assets::Token".to_owned(),
            BuiltInTypeId::StdAssetsToken as usize,
        );
        index
    }

    ///
    /// Generate the next type sequence ID and add the ID with the type `title` to the index.
    ///
    pub fn next(&self, title: String) -> usize {
        let type_id = self
            .inner
            .write()
            .expect(zinc_const::panic::MULTI_THREADING)
            .len();

        self.next_with_id(title, type_id)
    }

    ///
    /// Add the item `title` to the index with the specified `type_id` key.
    ///
    fn next_with_id(&self, title: String, type_id: usize) -> usize {
        let mut index = self
            .inner
            .write()
            .expect(zinc_const::panic::MULTI_THREADING);

        log::debug!("Type ID {:06} for {}", type_id, title);

        index.insert(type_id, title);
        type_id
    }
}
