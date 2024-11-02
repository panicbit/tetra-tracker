use std::cmp;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Id<T: ?Sized> {
    inner: u64,
    _p: PhantomData<fn(T) -> T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _p: self._p,
        }
    }
}

impl<T> cmp::PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> cmp::Eq for Id<T> {}

impl<T> cmp::PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.inner.cmp(&other.inner))
    }
}

impl<T> cmp::Ord for Id<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> Id<T> {
    pub fn increment(&mut self) {
        *self = self.incremented();
    }

    pub fn incremented(&self) -> Id<T> {
        Id {
            inner: self.inner + 1,
            _p: PhantomData,
        }
    }

    pub fn erased(&self) -> Id<()> {
        Id {
            inner: self.inner,
            _p: PhantomData,
        }
    }
}

impl Id<()> {
    pub fn typed<T>(&self) -> Id<T> {
        Id {
            inner: self.inner,
            _p: PhantomData,
        }
    }
}

pub trait AsId<T> {
    fn as_id(&self) -> Id<T>;
}

impl<T> AsId<T> for Id<T> {
    fn as_id(&self) -> Id<T> {
        self.clone()
    }
}

impl<T, I: AsId<T>> AsId<T> for &I {
    fn as_id(&self) -> Id<T> {
        <I as AsId<T>>::as_id(self)
    }
}
