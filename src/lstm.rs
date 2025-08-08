mod memtable;

pub struct Lstm {
    memtable: memtable::MemTable,
}

impl Lstm {
    pub const NO_DATA: &'static [u8] = &[];

    pub fn new() -> Self {
        Lstm {
            memtable: memtable::MemTable::new(),
        }
    }

    pub fn get(&self, key: &[u8]) -> &[u8] {
        self.memtable.get(key).unwrap_or(Lstm::NO_DATA)
    }

    pub fn del(&mut self, key: &[u8]) {
        self.set(key, Lstm::NO_DATA)
    }

    pub fn set(&mut self, key: &[u8], val: &[u8]) {
        self.memtable.set(
            key.to_owned().into_boxed_slice(),
            val.to_owned().into_boxed_slice(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_undefined() {
        let lstm = Lstm::new();
        assert_eq!(lstm.get(b"abc"), Lstm::NO_DATA);
    }

    #[test]
    fn set_and_get() {
        let mut lstm = Lstm::new();
        lstm.set(b"abc", b"xyz");
        assert_eq!(lstm.get(b"abc"), b"xyz".as_ref());
    }

    #[test]
    fn set_del_and_get() {
        let mut lstm = Lstm::new();
        lstm.set(b"abc", b"xyz");
        lstm.del(b"abc");
        assert_eq!(lstm.get(b"abc"), Lstm::NO_DATA);
    }

    #[test]
    fn set_set_and_get() {
        let mut lstm = Lstm::new();
        lstm.set(b"abc", b"first");
        lstm.set(b"abc", b"second");
        assert_eq!(lstm.get(b"abc"), b"second".as_ref());
    }
}
