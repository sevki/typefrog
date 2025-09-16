use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::Deref,
    sync::{Arc, OnceLock},
};

use rustc_hash::FxHashMap;

use crate::fact::Atom;
pub use std::sync::RwLock;
pub static DB: OnceLock<RwLock<Interner<Atom>>> = OnceLock::new();

/// When we load facts out of the table, they are essentially random
/// strings. We create an intern table to map those to small integers.
pub struct Interner<T>
where
    T: Eq + Clone,
{
    // index -> value
    by_index: Vec<Arc<T>>,
    // value -> index
    by_value: FxHashMap<T, usize>,
}

impl<T> Default for Interner<T>
where
    T: Eq + Clone,
{
    fn default() -> Self {
        Self {
            by_index: Vec::new(),
            by_value: FxHashMap::default(),
        }
    }
}

/// An interned value.
#[derive(Clone, PartialEq, Eq)]
pub struct Interned<T> {
    data: Arc<T>,
    index: usize,
}

impl<T> Interned<T>
where
    T: Eq + Clone,
{
    pub fn uid(&self) -> usize {
        self.index
    }
}

impl<T> Hash for Interned<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<T> Deref for Interned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Ord for Interned<T>
where
    T: Eq + Clone + Hash,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl<T> PartialOrd for Interned<T>
where
    T: Eq + Clone + Hash,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Interner<T>
where
    T: Eq + Clone + Hash,
{
    #[allow(dead_code)]
    pub(crate) fn untern(&self, data: Interned<T>) -> T {
        let data: usize = data.index;
        self.by_index[data].as_ref().clone()
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn untern_vec(&self, data: &[Interned<T>]) -> Vec<T> {
        data.iter().map(|d| self.untern(d.clone())).collect()
    }
}

pub trait Intern<T: Clone + Eq + Hash> {
    fn intern(&self, data: T) -> Interned<T>;
}

impl<T> Intern<T> for &RwLock<Interner<T>>
where
    T: Clone + Eq + Hash,
{
    fn intern(&self, data: T) -> Interned<T> {
        if let Ok(read) = self.read() {
            if let Some(&idx) = read.by_value.get(&data) {
                return Interned {
                    index: idx,
                    data: read.by_index[idx].clone(),
                };
            }
        }
        let mut write = self.write().expect("interner poisoned");
        if let Some(&idx) = write.by_value.get(&data) {
            return Interned {
                index: idx,
                data: write.by_index[idx].clone(),
            };
        }
        let idx = write.by_index.len();
        let arc = Arc::new(data.clone());
        write.by_index.push(arc.clone());
        write.by_value.insert(data, idx);
        Interned {
            index: idx,
            data: arc,
        }
    }
}

impl<T> Display for Interned<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T> Debug for Interned<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! intern {
    ($t:expr) => {
        $crate::internment::DB
            .get_or_init(
                || $crate::internment::RwLock::new($crate::internment::Interner::default()),
            )
            .intern($t.into())
    };
}
