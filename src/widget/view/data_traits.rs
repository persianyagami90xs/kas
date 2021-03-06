// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Data traits for view widgets
//!
//! This module holds these traits and basic impls for derived types.

#[allow(unused)]
use kas::event::Manager;
use kas::event::UpdateHandle;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;

/// Trait for viewable single data items
// Note: we require Debug + 'static to allow widgets using this to implement
// WidgetCore, which requires Debug + Any.
pub trait SingleData: Debug {
    type Item: Clone;

    // TODO(gat): add get<'a>(&self) -> Self::ItemRef<'a> and get_mut

    /// Get data (clone)
    fn get_cloned(&self) -> Self::Item;

    /// Get an update handle, if any is used
    ///
    /// Widgets may use this `handle` to call `mgr.update_on_handle(handle, self.id())`.
    fn update_handle(&self) -> Option<UpdateHandle> {
        None
    }
}

/// Trait for writable single data items
pub trait SingleDataMut: SingleData {
    /// Set data
    fn set(&self, value: Self::Item) -> UpdateHandle;
}

/// Trait for viewable data lists
pub trait ListData: Debug {
    /// Key type
    type Key: Clone + Debug + PartialEq + Eq;

    /// Item type
    type Item: Clone;

    /// Number of data items available
    ///
    /// Note: users may assume this is `O(1)`.
    fn len(&self) -> usize;

    // TODO(gat): add get<'a>(&self) -> Self::ItemRef<'a> and get_mut

    /// Get data by key (clone)
    fn get_cloned(&self, key: &Self::Key) -> Option<Self::Item>;

    // TODO(gat): replace with an iterator
    /// Iterate over (key, value) pairs as a vec
    ///
    /// The result will be in deterministic implementation-defined order, with
    /// a length of `max(limit, data_len)` where `data_len` is the number of
    /// items available.
    fn iter_vec(&self, limit: usize) -> Vec<(Self::Key, Self::Item)> {
        self.iter_vec_from(0, limit)
    }

    /// Iterate over (key, value) pairs as a vec
    ///
    /// The result is the same as `self.iter_vec(start + limit).skip(start)`.
    fn iter_vec_from(&self, start: usize, limit: usize) -> Vec<(Self::Key, Self::Item)>;

    /// Get an update handle, if any is used
    ///
    /// Widgets may use this `handle` to call `mgr.update_on_handle(handle, self.id())`.
    fn update_handle(&self) -> Option<UpdateHandle> {
        None
    }
}

/// Trait for writable data lists
pub trait ListDataMut: ListData {
    /// Set data by key
    fn set(&self, key: Self::Key, item: Self::Item) -> UpdateHandle;
}

impl<T: Clone + Debug> ListData for [T] {
    type Key = usize;
    type Item = T;

    fn len(&self) -> usize {
        (*self).len()
    }

    fn get_cloned(&self, key: &usize) -> Option<Self::Item> {
        self.get(*key).cloned()
    }

    fn iter_vec(&self, limit: usize) -> Vec<(Self::Key, Self::Item)> {
        self.iter().cloned().enumerate().take(limit).collect()
    }

    fn iter_vec_from(&self, start: usize, limit: usize) -> Vec<(Self::Key, Self::Item)> {
        self.iter()
            .cloned()
            .enumerate()
            .skip(start)
            .take(limit)
            .collect()
    }
}

// TODO(spec): implement using Deref; for now can't since it "might" conflict
// with a RefCell impl on a derived type downstream, according to the solver.
// impl<T: Deref + Debug> SingleData for T
// where
//     <T as Deref>::Target: SingleData,
macro_rules! impl_via_deref {
    ($t: ident: $derived:ty) => {
        impl<$t: SingleData + ?Sized> SingleData for $derived {
            type Item = $t::Item;
            fn get_cloned(&self) -> Self::Item {
                self.deref().get_cloned()
            }
            fn update_handle(&self) -> Option<UpdateHandle> {
                self.deref().update_handle()
            }
        }
        impl<$t: SingleDataMut + ?Sized> SingleDataMut for $derived {
            fn set(&self, value: Self::Item) -> UpdateHandle {
                self.deref().set(value)
            }
        }

        impl<$t: ListData + ?Sized> ListData for $derived {
            type Key = $t::Key;
            type Item = $t::Item;

            fn len(&self) -> usize {
                self.deref().len()
            }
            fn get_cloned(&self, key: &Self::Key) -> Option<Self::Item> {
                self.deref().get_cloned(key)
            }

            fn iter_vec(&self, limit: usize) -> Vec<(Self::Key, Self::Item)> {
                self.deref().iter_vec(limit)
            }
            fn iter_vec_from(&self, start: usize, limit: usize) -> Vec<(Self::Key, Self::Item)> {
                self.deref().iter_vec_from(start, limit)
            }

            fn update_handle(&self) -> Option<UpdateHandle> {
                self.deref().update_handle()
            }
        }
        impl<$t: ListDataMut + ?Sized> ListDataMut for $derived {
            fn set(&self, key: Self::Key, item: Self::Item) -> UpdateHandle {
                self.deref().set(key, item)
            }
        }
    };
    ($t: ident: $derived:ty, $($dd:ty),+) => {
        impl_via_deref!($t: $derived);
        impl_via_deref!($t: $($dd),+);
    };
}
impl_via_deref!(T: &T, &mut T);
impl_via_deref!(T: std::rc::Rc<T>, std::sync::Arc<T>, Box<T>);

impl<T: SingleData> SingleData for RefCell<T> {
    type Item = T::Item;
    fn get_cloned(&self) -> Self::Item {
        self.borrow().get_cloned()
    }
    fn update_handle(&self) -> Option<UpdateHandle> {
        self.borrow().update_handle()
    }
}
impl<T: SingleDataMut> SingleDataMut for RefCell<T> {
    fn set(&self, value: Self::Item) -> UpdateHandle {
        self.borrow_mut().set(value)
    }
}

impl<T: ListData> ListData for RefCell<T> {
    type Key = T::Key;
    type Item = T::Item;

    fn len(&self) -> usize {
        self.borrow().len()
    }
    fn get_cloned(&self, key: &Self::Key) -> Option<Self::Item> {
        self.borrow().get_cloned(key)
    }

    fn iter_vec(&self, limit: usize) -> Vec<(Self::Key, Self::Item)> {
        self.borrow().iter_vec(limit)
    }
    fn iter_vec_from(&self, start: usize, limit: usize) -> Vec<(Self::Key, Self::Item)> {
        self.borrow().iter_vec_from(start, limit)
    }

    fn update_handle(&self) -> Option<UpdateHandle> {
        self.borrow().update_handle()
    }
}
impl<T: ListDataMut> ListDataMut for RefCell<T> {
    fn set(&self, key: Self::Key, item: Self::Item) -> UpdateHandle {
        self.borrow().set(key, item)
    }
}
