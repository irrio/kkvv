use std::collections::BTreeMap;

pub struct MemTable {
    data: BTreeMap<Box<[u8]>, Box<[u8]>>,
}

impl MemTable {
    pub fn new() -> Self {
        MemTable {
            data: BTreeMap::new(),
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.data.get(key).map(|b| b.as_ref())
    }

    pub fn set(&mut self, key: Box<[u8]>, val: Box<[u8]>) {
        self.data.insert(key, val);
    }
}
