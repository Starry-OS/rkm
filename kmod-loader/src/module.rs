use core::fmt::Debug;

use alloc::{string::String, vec::Vec};

#[derive(Clone)]
pub struct ModuleInfo {
    kv: Vec<(String, String)>,
}

impl Debug for ModuleInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ModuleInfo {{ ")?;
        for (idx, (k, v)) in self.kv.iter().enumerate() {
            if idx > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, " }}")
    }
}

impl ModuleInfo {
    pub fn new() -> Self {
        ModuleInfo { kv: Vec::new() }
    }

    pub fn add_kv(&mut self, key: String, value: String) {
        self.kv.push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        for (k, v) in &self.kv {
            if k == key {
                return Some(v);
            }
        }
        None
    }
}
