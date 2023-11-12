use arceos_api::rng::random;
// #[cfg(test)]
// mod tests;

use hashbrown::hash_map as base;

use core::borrow::Borrow;
// use crate::cell::Cell;
// use crate::collections::TryReserveError;
// use crate::collections::TryReserveErrorKind;
// use core::error::Error;
use core::fmt::{self, Debug};
#[allow(deprecated)]
use core::hash::{BuildHasher, Hash, Hasher, SipHasher13};
use core::iter::FusedIterator;
// use crate::ops::Index;
// use crate::sys;

// #[cfg_attr(not(test), rustc_diagnostic_item = "HashMap")]
// // #[stable(feature = "rust1", since = "1.0.0")]
// #[rustc_insignificant_dtor]
pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}

impl<K, V> HashMap<K, V, RandomState> {
    pub fn new() -> HashMap<K, V, RandomState> {
        Default::default()
    }
    // pub fn with_capacity(capacity: usize) -> HashMap<K, V, RandomState> {
    //     HashMap::with_capacity_and_hasher(capacity, Default::default())
    // }
}

impl<K, V, S> HashMap<K, V, S> {
//     #[inline]
//     #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
//     #[rustc_const_unstable(feature = "const_collections_with_hasher", issue = "102575")]
    pub const fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap { base: base::HashMap::with_hasher(hash_builder) }
    }

//     #[inline]
//     #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
//     pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> HashMap<K, V, S> {
//         HashMap { base: base::HashMap::with_capacity_and_hasher(capacity, hasher) }
//     }

//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn capacity(&self) -> usize {
        self.base.capacity()
    }

    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn keys(&self) -> Keys<'_, K, V> {
//         Keys { inner: self.iter() }
//     }

//     #[inline]
// //    #[rustc_lint_query_instability]
//     // #[stable(feature = "map_into_keys_values", since = "1.54.0")]
//     pub fn into_keys(self) -> IntoKeys<K, V> {
//         IntoKeys { inner: self.into_iter() }
//     }

    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn values(&self) -> Values<'_, K, V> {
//         Values { inner: self.iter() }
//     }

//     // #[stable(feature = "map_values_mut", since = "1.10.0")]
//     pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
//         ValuesMut { inner: self.iter_mut() }
//     }

// //    #[rustc_lint_query_instability]
    // // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.iter() }
    }

// //    #[rustc_lint_query_instability]
    // // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut { base: self.base.iter_mut() }
    }

    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn len(&self) -> usize {
//         self.base.len()
//     }

//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn is_empty(&self) -> bool {
//         self.base.is_empty()
//     }

//     #[inline]
// //    #[rustc_lint_query_instability]
//     // #[stable(feature = "drain", since = "1.6.0")]
//     pub fn drain(&mut self) -> Drain<'_, K, V> {
//         Drain { base: self.base.drain() }
//     }

//     #[inline]
// //    #[rustc_lint_query_instability]
//     // #[unstable(feature = "hash_extract_if", issue = "59618")]
//     pub fn extract_if<F>(&mut self, pred: F) -> ExtractIf<'_, K, V, F>
//     where
//         F: FnMut(&K, &mut V) -> bool,
//     {
//         ExtractIf { base: self.base.extract_if(pred) }
//     }

//     #[inline]
// //    #[rustc_lint_query_instability]
//     #[stable(feature = "retain_hash_collection", since = "1.18.0")]
//     pub fn retain<F>(&mut self, f: F)
//     where
//         F: FnMut(&K, &mut V) -> bool,
//     {
//         self.base.retain(f)
//     }

//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn clear(&mut self) {
//         self.base.clear();
//     }

//     #[inline]
//     #[stable(feature = "hashmap_public_hasher", since = "1.9.0")]
//     pub fn hasher(&self) -> &S {
//         self.base.hasher()
//     }
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn reserve(&mut self, additional: usize) {
//         self.base.reserve(additional)
//     }

//     #[inline]
//     #[stable(feature = "try_reserve", since = "1.57.0")]
//     pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
//         self.base.try_reserve(additional).map_err(map_try_reserve_error)
//     }

//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn shrink_to_fit(&mut self) {
//         self.base.shrink_to_fit();
//     }

//     #[inline]
//     #[stable(feature = "shrink_to", since = "1.56.0")]
//     pub fn shrink_to(&mut self, min_capacity: usize) {
//         self.base.shrink_to(min_capacity);
//     }

//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
//         map_entry(self.base.rustc_entry(key))
//     }

    // // #[stable(feature = "rust1", since = "1.0.0")]
//     #[inline]
//     pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
//     where
//         K: Borrow<Q>,
//         Q: Hash + Eq,
//     {
//         self.base.get(k)
//     }

//     #[inline]
//     #[stable(feature = "map_get_key_value", since = "1.40.0")]
//     pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
//     where
//         K: Borrow<Q>,
//         Q: Hash + Eq,
//     {
//         self.base.get_key_value(k)
//     }

//     #[inline]
//     #[unstable(feature = "map_many_mut", issue = "97601")]
//     pub fn get_many_mut<Q: ?Sized, const N: usize>(&mut self, ks: [&Q; N]) -> Option<[&'_ mut V; N]>
//     where
//         K: Borrow<Q>,
//         Q: Hash + Eq,
//     {
//         self.base.get_many_mut(ks)
//     }

//     #[inline]
//     #[unstable(feature = "map_many_mut", issue = "97601")]
//     pub unsafe fn get_many_unchecked_mut<Q: ?Sized, const N: usize>(
//         &mut self,
//         ks: [&Q; N],
//     ) -> Option<[&'_ mut V; N]>
//     where
//         K: Borrow<Q>,
//         Q: Hash + Eq,
//     {
//         self.base.get_many_unchecked_mut(ks)
//     }

//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
//     where
//         K: Borrow<Q>,
//         Q: Hash + Eq,
//     {
//         self.base.contains_key(k)
//     }

//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
//     where
//         K: Borrow<Q>,
//         Q: Hash + Eq,
//     {
//         self.base.get_mut(k)
//     }

//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }

//     // #[unstable(feature = "map_try_insert", issue = "82766")]
//     pub fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, OccupiedError<'_, K, V>> {
//         match self.entry(key) {
//             Occupied(entry) => Err(OccupiedError { entry, value }),
//             Vacant(entry) => Ok(entry.insert(value)),
//         }
//     }

//     #[inline]
    // // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
//     where
//         K: Borrow<Q>,
//         Q: Hash + Eq,
//     {
//         self.base.remove(k)
//     }

//     #[inline]
//     #[stable(feature = "hash_map_remove_entry", since = "1.27.0")]
//     pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
//     where
//         K: Borrow<Q>,
//         Q: Hash + Eq,
//     {
//         self.base.remove_entry(k)
//     }
}

// impl<K, V, S> HashMap<K, V, S>
// where
//     S: BuildHasher,
// {
//     #[inline]
//     // #[unstable(feature = "hash_raw_entry", issue = "56167")]
//     pub fn raw_entry_mut(&mut self) -> RawEntryBuilderMut<'_, K, V, S> {
//         RawEntryBuilderMut { map: self }
//     }

//     #[inline]
//     // #[unstable(feature = "hash_raw_entry", issue = "56167")]
//     pub fn raw_entry(&self) -> RawEntryBuilder<'_, K, V, S> {
//         RawEntryBuilder { map: self }
//     }
// }

// // #[stable(feature = "rust1", since = "1.0.0")]
// impl<K, V, S> Clone for HashMap<K, V, S>
// where
//     K: Clone,
//     V: Clone,
//     S: Clone,
// {
//     #[inline]
//     fn clone(&self) -> Self {
//         Self { base: self.base.clone() }
//     }

//     #[inline]
//     fn clone_from(&mut self, other: &Self) {
//         self.base.clone_from(&other.base);
//     }
// }

// // #[stable(feature = "rust1", since = "1.0.0")]
// impl<K, V, S> PartialEq for HashMap<K, V, S>
// where
//     K: Eq + Hash,
//     V: PartialEq,
//     S: BuildHasher,
// {
//     fn eq(&self, other: &HashMap<K, V, S>) -> bool {
//         if self.len() != other.len() {
//             return false;
//         }

//         self.iter().all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
//     }
// }

// // #[stable(feature = "rust1", since = "1.0.0")]
// impl<K, V, S> Eq for HashMap<K, V, S>
// where
//     K: Eq + Hash,
//     V: Eq,
//     S: BuildHasher,
// {
// }

// // #[stable(feature = "rust1", since = "1.0.0")]
// impl<K, V, S> Debug for HashMap<K, V, S>
// where
//     K: Debug,
//     V: Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_map().entries(self.iter()).finish()
//     }
// }

// // #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V, S> Default for HashMap<K, V, S>
where
    S: Default,
{
    #[inline]
    fn default() -> HashMap<K, V, S> {
        HashMap::with_hasher(Default::default())
    }
}

// // #[stable(feature = "rust1", since = "1.0.0")]
// impl<K, Q: ?Sized, V, S> Index<&Q> for HashMap<K, V, S>
// where
//     K: Eq + Hash + Borrow<Q>,
//     Q: Eq + Hash,
//     S: BuildHasher,
// {
//     type Output = V;

//     #[inline]
//     fn index(&self, key: &Q) -> &V {
//         self.get(key).expect("no entry found for key")
//     }
// }

// impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V, RandomState>
// where
//     K: Eq + Hash,
// {
//     fn from(arr: [(K, V); N]) -> Self {
//         Self::from_iter(arr)
//     }
// }

// // #[stable(feature = "rust1", since = "1.0.0")]
pub struct Iter<'a, K: 'a, V: 'a> {
    base: base::Iter<'a, K, V>,
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
// // #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V> Clone for Iter<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Iter { base: self.base.clone() }
    }
}

// #[stable(feature = "std_debug", since = "1.16.0")]
impl<K: Debug, V: Debug> fmt::Debug for Iter<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
pub struct IterMut<'a, K: 'a, V: 'a> {
    base: base::IterMut<'a, K, V>,
}

// impl<'a, K, V> IterMut<'a, K, V> {
//     #[inline]
//     pub(super) fn iter(&self) -> Iter<'_, K, V> {
//         Iter { base: self.base.rustc_iter() }
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
pub struct IntoIter<K, V> {
    base: base::IntoIter<K, V>,
}

// impl<K, V> IntoIter<K, V> {
//     #[inline]
//     pub(super) fn iter(&self) -> Iter<'_, K, V> {
//         Iter { base: self.base.rustc_iter() }
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
pub struct Keys<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V> Clone for Keys<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Keys { inner: self.inner.clone() }
    }
}

// #[stable(feature = "std_debug", since = "1.16.0")]
impl<K: Debug, V> fmt::Debug for Keys<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
pub struct Values<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V> Clone for Values<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Values { inner: self.inner.clone() }
    }
}

// #[stable(feature = "std_debug", since = "1.16.0")]
impl<K, V: Debug> fmt::Debug for Values<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

// // #[stable(feature = "drain", since = "1.6.0")]
pub struct Drain<'a, K: 'a, V: 'a> {
    base: base::Drain<'a, K, V>,
}

// impl<'a, K, V> Drain<'a, K, V> {
//     #[inline]
//     pub(super) fn iter(&self) -> Iter<'_, K, V> {
//         Iter { base: self.base.rustc_iter() }
//     }
// }

// #[unstable(feature = "hash_extract_if", issue = "59618")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ExtractIf<'a, K, V, F>
where
    F: FnMut(&K, &mut V) -> bool,
{
    base: base::ExtractIf<'a, K, V, F>,
}

// #[stable(feature = "map_values_mut", since = "1.10.0")]
pub struct ValuesMut<'a, K: 'a, V: 'a> {
    inner: IterMut<'a, K, V>,
}

// #[stable(feature = "map_into_keys_values", since = "1.54.0")]
pub struct IntoKeys<K, V> {
    inner: IntoIter<K, V>,
}

// #[stable(feature = "map_into_keys_values", since = "1.54.0")]
pub struct IntoValues<K, V> {
    inner: IntoIter<K, V>,
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
pub struct RawEntryBuilderMut<'a, K: 'a, V: 'a, S: 'a> {
    map: &'a mut HashMap<K, V, S>,
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
pub enum RawEntryMut<'a, K: 'a, V: 'a, S: 'a> {
    Occupied(RawOccupiedEntryMut<'a, K, V, S>),
    Vacant(RawVacantEntryMut<'a, K, V, S>),
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
pub struct RawOccupiedEntryMut<'a, K: 'a, V: 'a, S: 'a> {
    base: base::RawOccupiedEntryMut<'a, K, V, S>,
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
pub struct RawVacantEntryMut<'a, K: 'a, V: 'a, S: 'a> {
    base: base::RawVacantEntryMut<'a, K, V, S>,
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
pub struct RawEntryBuilder<'a, K: 'a, V: 'a, S: 'a> {
    map: &'a HashMap<K, V, S>,
}

impl<'a, K, V, S> RawEntryBuilderMut<'a, K, V, S>
where
    S: BuildHasher,
{
    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn from_key<Q: ?Sized>(self, k: &Q) -> RawEntryMut<'a, K, V, S>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        map_raw_entry(self.map.base.raw_entry_mut().from_key(k))
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn from_key_hashed_nocheck<Q: ?Sized>(self, hash: u64, k: &Q) -> RawEntryMut<'a, K, V, S>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        map_raw_entry(self.map.base.raw_entry_mut().from_key_hashed_nocheck(hash, k))
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn from_hash<F>(self, hash: u64, is_match: F) -> RawEntryMut<'a, K, V, S>
    where
        for<'b> F: FnMut(&'b K) -> bool,
    {
        map_raw_entry(self.map.base.raw_entry_mut().from_hash(hash, is_match))
    }
}

impl<'a, K, V, S> RawEntryBuilder<'a, K, V, S>
where
    S: BuildHasher,
{
    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn from_key<Q: ?Sized>(self, k: &Q) -> Option<(&'a K, &'a V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.base.raw_entry().from_key(k)
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn from_key_hashed_nocheck<Q: ?Sized>(self, hash: u64, k: &Q) -> Option<(&'a K, &'a V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.base.raw_entry().from_key_hashed_nocheck(hash, k)
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn from_hash<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {
        self.map.base.raw_entry().from_hash(hash, is_match)
    }
}

impl<'a, K, V, S> RawEntryMut<'a, K, V, S> {
    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn or_insert(self, default_key: K, default_val: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            RawEntryMut::Occupied(entry) => entry.into_key_value(),
            RawEntryMut::Vacant(entry) => entry.insert(default_key, default_val),
        }
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn or_insert_with<F>(self, default: F) -> (&'a mut K, &'a mut V)
    where
        F: FnOnce() -> (K, V),
        K: Hash,
        S: BuildHasher,
    {
        match self {
            RawEntryMut::Occupied(entry) => entry.into_key_value(),
            RawEntryMut::Vacant(entry) => {
                let (k, v) = default();
                entry.insert(k, v)
            }
        }
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut K, &mut V),
    {
        match self {
            RawEntryMut::Occupied(mut entry) => {
                {
                    let (k, v) = entry.get_key_value_mut();
                    f(k, v);
                }
                RawEntryMut::Occupied(entry)
            }
            RawEntryMut::Vacant(entry) => RawEntryMut::Vacant(entry),
        }
    }
}

impl<'a, K, V, S> RawOccupiedEntryMut<'a, K, V, S> {
    #[inline]
    #[must_use]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn key(&self) -> &K {
        self.base.key()
    }

    #[inline]
    #[must_use]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn key_mut(&mut self) -> &mut K {
        self.base.key_mut()
    }

    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn into_key(self) -> &'a mut K {
        self.base.into_key()
    }

    #[inline]
    #[must_use]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn get(&self) -> &V {
        self.base.get()
    }

    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn into_mut(self) -> &'a mut V {
        self.base.into_mut()
    }

    #[inline]
    #[must_use]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn get_mut(&mut self) -> &mut V {
        self.base.get_mut()
    }

    #[inline]
    #[must_use]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn get_key_value(&mut self) -> (&K, &V) {
        self.base.get_key_value()
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {
        self.base.get_key_value_mut()
    }

    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn into_key_value(self) -> (&'a mut K, &'a mut V) {
        self.base.into_key_value()
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn insert(&mut self, value: V) -> V {
        self.base.insert(value)
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn insert_key(&mut self, key: K) -> K {
        self.base.insert_key(key)
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn remove(self) -> V {
        self.base.remove()
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn remove_entry(self) -> (K, V) {
        self.base.remove_entry()
    }
}

impl<'a, K, V, S> RawVacantEntryMut<'a, K, V, S> {
    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        self.base.insert(key, value)
    }

    #[inline]
    // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    pub fn insert_hashed_nocheck(self, hash: u64, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        self.base.insert_hashed_nocheck(hash, key, value)
    }
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
impl<K, V, S> Debug for RawEntryBuilderMut<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawEntryBuilder").finish_non_exhaustive()
    }
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
impl<K: Debug, V: Debug, S> Debug for RawEntryMut<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RawEntryMut::Vacant(ref v) => f.debug_tuple("RawEntry").field(v).finish(),
            RawEntryMut::Occupied(ref o) => f.debug_tuple("RawEntry").field(o).finish(),
        }
    }
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
impl<K: Debug, V: Debug, S> Debug for RawOccupiedEntryMut<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawOccupiedEntryMut")
            .field("key", self.key())
            .field("value", self.get())
            .finish_non_exhaustive()
    }
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
impl<K, V, S> Debug for RawVacantEntryMut<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawVacantEntryMut").finish_non_exhaustive()
    }
}

// #[unstable(feature = "hash_raw_entry", issue = "56167")]
impl<K, V, S> Debug for RawEntryBuilder<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawEntryBuilder").finish_non_exhaustive()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
// #[cfg_attr(not(test), rustc_diagnostic_item = "HashMapEntry")]
// pub enum Entry<'a, K: 'a, V: 'a> {
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     Occupied(OccupiedEntry<'a, K, V>),

//     // #[stable(feature = "rust1", since = "1.0.0")]
//     Vacant(VacantEntry<'a, K, V>),
// }

// #[stable(feature = "debug_hash_map", since = "1.12.0")]
// impl<K: Debug, V: Debug> Debug for Entry<'_, K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match *self {
//             Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
//             Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
//         }
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
//     base: base::RustcOccupiedEntry<'a, K, V>,
// }

// // #[stable(feature = "debug_hash_map", since = "1.12.0")]
// impl<K: Debug, V: Debug> Debug for OccupiedEntry<'_, K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("OccupiedEntry")
//             .field("key", self.key())
//             .field("value", self.get())
//             .finish_non_exhaustive()
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// pub struct VacantEntry<'a, K: 'a, V: 'a> {
//     base: base::RustcVacantEntry<'a, K, V>,
// }

// #[stable(feature = "debug_hash_map", since = "1.12.0")]
// impl<K: Debug, V> Debug for VacantEntry<'_, K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_tuple("VacantEntry").field(self.key()).finish()
//     }
// }

// #[unstable(feature = "map_try_insert", issue = "82766")]
// pub struct OccupiedError<'a, K: 'a, V: 'a> {
//     pub entry: OccupiedEntry<'a, K, V>,
//     pub value: V,
// }

// #[unstable(feature = "map_try_insert", issue = "82766")]
// impl<K: Debug, V: Debug> Debug for OccupiedError<'_, K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("OccupiedError")
//             .field("key", self.entry.key())
//             .field("old_value", self.entry.get())
//             .field("new_value", &self.value)
//             .finish_non_exhaustive()
//     }
// }

// // #[unstable(feature = "map_try_insert", issue = "82766")]
// impl<'a, K: Debug, V: Debug> fmt::Display for OccupiedError<'a, K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "failed to insert {:?}, key {:?} already exists with value {:?}",
//             self.value,
//             self.entry.key(),
//             self.entry.get(),
//         )
//     }
// }

// #[unstable(feature = "map_try_insert", issue = "82766")]
// impl<'a, K: fmt::Debug, V: fmt::Debug> Error for OccupiedError<'a, K, V> {
//     #[allow(deprecated)]
//     fn description(&self) -> &str {
//         "key already exists"
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, K, V, S> IntoIterator for &'a HashMap<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
//    #[rustc_lint_query_instability]
    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, K, V, S> IntoIterator for &'a mut HashMap<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
//    #[rustc_lint_query_instability]
    fn into_iter(self) -> IterMut<'a, K, V> {
        self.iter_mut()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V, S> IntoIterator for HashMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
//    #[rustc_lint_query_instability]
    fn into_iter(self) -> IntoIter<K, V> {
        IntoIter { base: self.base.into_iter() }
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}
// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}

// #[stable(feature = "fused", since = "1.26.0")]
impl<K, V> FusedIterator for Iter<'_, K, V> {}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}
// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}
// #[stable(feature = "fused", since = "1.26.0")]
// impl<K, V> FusedIterator for IterMut<'_, K, V> {}

// // #[stable(feature = "std_debug", since = "1.16.0")]
// impl<K, V> fmt::Debug for IterMut<'_, K, V>
// where
//     K: fmt::Debug,
//     V: fmt::Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.iter()).finish()
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}
// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}
// #[stable(feature = "fused", since = "1.26.0")]
// impl<K, V> FusedIterator for IntoIter<K, V> {}

// // #[stable(feature = "std_debug", since = "1.16.0")]
// impl<K: Debug, V: Debug> fmt::Debug for IntoIter<K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.iter()).finish()
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<&'a K> {
        self.inner.next().map(|(k, _)| k)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V> ExactSizeIterator for Keys<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
// #[stable(feature = "fused", since = "1.26.0")]
impl<K, V> FusedIterator for Keys<'_, K, V> {}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, v)| v)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V> ExactSizeIterator for Values<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
// #[stable(feature = "fused", since = "1.26.0")]
impl<K, V> FusedIterator for Values<'_, K, V> {}

// #[stable(feature = "map_values_mut", since = "1.10.0")]
impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|(_, v)| v)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
// #[stable(feature = "map_values_mut", since = "1.10.0")]
impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
// #[stable(feature = "fused", since = "1.26.0")]
// impl<K, V> FusedIterator for ValuesMut<'_, K, V> {}

// // #[stable(feature = "std_debug", since = "1.16.0")]
// impl<K, V: fmt::Debug> fmt::Debug for ValuesMut<'_, K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.inner.iter().map(|(_, val)| val)).finish()
//     }
// }

// #[stable(feature = "map_into_keys_values", since = "1.54.0")]
impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<K> {
        self.inner.next().map(|(k, _)| k)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
// #[stable(feature = "map_into_keys_values", since = "1.54.0")]
impl<K, V> ExactSizeIterator for IntoKeys<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
// #[stable(feature = "map_into_keys_values", since = "1.54.0")]
// impl<K, V> FusedIterator for IntoKeys<K, V> {}

// // #[stable(feature = "map_into_keys_values", since = "1.54.0")]
// impl<K: Debug, V> fmt::Debug for IntoKeys<K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.inner.iter().map(|(k, _)| k)).finish()
//     }
// }

// #[stable(feature = "map_into_keys_values", since = "1.54.0")]
impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<V> {
        self.inner.next().map(|(_, v)| v)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
// #[stable(feature = "map_into_keys_values", since = "1.54.0")]
impl<K, V> ExactSizeIterator for IntoValues<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
// #[stable(feature = "map_into_keys_values", since = "1.54.0")]
// impl<K, V> FusedIterator for IntoValues<K, V> {}

// // #[stable(feature = "map_into_keys_values", since = "1.54.0")]
// impl<K, V: Debug> fmt::Debug for IntoValues<K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.inner.iter().map(|(_, v)| v)).finish()
//     }
// }

// #[stable(feature = "drain", since = "1.6.0")]
impl<'a, K, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}
// #[stable(feature = "drain", since = "1.6.0")]
impl<K, V> ExactSizeIterator for Drain<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}
// #[stable(feature = "fused", since = "1.26.0")]
// impl<K, V> FusedIterator for Drain<'_, K, V> {}

// // #[stable(feature = "std_debug", since = "1.16.0")]
// impl<K, V> fmt::Debug for Drain<'_, K, V>
// where
//     K: fmt::Debug,
//     V: fmt::Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.iter()).finish()
//     }
// }

// #[unstable(feature = "hash_extract_if", issue = "59618")]
impl<K, V, F> Iterator for ExtractIf<'_, K, V, F>
where
    F: FnMut(&K, &mut V) -> bool,
{
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

// #[unstable(feature = "hash_extract_if", issue = "59618")]
impl<K, V, F> FusedIterator for ExtractIf<'_, K, V, F> where F: FnMut(&K, &mut V) -> bool {}

// #[unstable(feature = "hash_extract_if", issue = "59618")]
impl<'a, K, V, F> fmt::Debug for ExtractIf<'a, K, V, F>
where
    F: FnMut(&K, &mut V) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtractIf").finish_non_exhaustive()
    }
}

// impl<'a, K, V> Entry<'a, K, V> {
//     #[inline]
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn or_insert(self, default: V) -> &'a mut V {
//         match self {
//             Occupied(entry) => entry.into_mut(),
//             Vacant(entry) => entry.insert(default),
//         }
//     }

//     #[inline]
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
//         match self {
//             Occupied(entry) => entry.into_mut(),
//             Vacant(entry) => entry.insert(default()),
//         }
//     }

//     #[inline]
//     #[stable(feature = "or_insert_with_key", since = "1.50.0")]
//     pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
//         match self {
//             Occupied(entry) => entry.into_mut(),
//             Vacant(entry) => {
//                 let value = default(entry.key());
//                 entry.insert(value)
//             }
//         }
//     }

//     #[inline]
//     #[stable(feature = "map_entry_keys", since = "1.10.0")]
//     pub fn key(&self) -> &K {
//         match *self {
//             Occupied(ref entry) => entry.key(),
//             Vacant(ref entry) => entry.key(),
//         }
//     }

//     #[inline]
//     #[stable(feature = "entry_and_modify", since = "1.26.0")]
//     pub fn and_modify<F>(self, f: F) -> Self
//     where
//         F: FnOnce(&mut V),
//     {
//         match self {
//             Occupied(mut entry) => {
//                 f(entry.get_mut());
//                 Occupied(entry)
//             }
//             Vacant(entry) => Vacant(entry),
//         }
//     }

//     #[inline]
//     #[unstable(feature = "entry_insert", issue = "65225")]
//     pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {
//         match self {
//             Occupied(mut entry) => {
//                 entry.insert(value);
//                 entry
//             }
//             Vacant(entry) => entry.insert_entry(value),
//         }
//     }
// }


// impl<'a, K, V: Default> Entry<'a, K, V> {
//     #[inline]
//     #[stable(feature = "entry_or_default", since = "1.28.0")]
//     pub fn or_default(self) -> &'a mut V {
//         match self {
//             Occupied(entry) => entry.into_mut(),
//             Vacant(entry) => entry.insert(Default::default()),
//         }
//     }
// }

// impl<'a, K, V> OccupiedEntry<'a, K, V> {
//     #[inline]
//     #[stable(feature = "map_entry_keys", since = "1.10.0")]
//     pub fn key(&self) -> &K {
//         self.base.key()
//     }

//     #[inline]
//     #[stable(feature = "map_entry_recover_keys2", since = "1.12.0")]
//     pub fn remove_entry(self) -> (K, V) {
//         self.base.remove_entry()
//     }

//     #[inline]
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn get(&self) -> &V {
//         self.base.get()
//     }

//     #[inline]
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn get_mut(&mut self) -> &mut V {
//         self.base.get_mut()
//     }

//     #[inline]
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn into_mut(self) -> &'a mut V {
//         self.base.into_mut()
//     }

//     #[inline]
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn insert(&mut self, value: V) -> V {
//         self.base.insert(value)
//     }

//     #[inline]
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn remove(self) -> V {
//         self.base.remove()
//     }

//     #[inline]
//     #[unstable(feature = "map_entry_replace", issue = "44286")]
//     pub fn replace_entry(self, value: V) -> (K, V) {
//         self.base.replace_entry(value)
//     }

//     #[inline]
//     #[unstable(feature = "map_entry_replace", issue = "44286")]
//     pub fn replace_key(self) -> K {
//         self.base.replace_key()
//     }
// }

// impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V> {
//     #[inline]
//     #[stable(feature = "map_entry_keys", since = "1.10.0")]
//     pub fn key(&self) -> &K {
//         self.base.key()
//     }

//     #[inline]
//     #[stable(feature = "map_entry_recover_keys2", since = "1.12.0")]
//     pub fn into_key(self) -> K {
//         self.base.into_key()
//     }

//     #[inline]
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn insert(self, value: V) -> &'a mut V {
//         self.base.insert(value)
//     }

//     #[inline]
//     #[unstable(feature = "entry_insert", issue = "65225")]
//     pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {
//         let base = self.base.insert_entry(value);
//         OccupiedEntry { base }
//     }
// }


// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V, S> FromIterator<(K, V)> for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> HashMap<K, V, S> {
        let mut map = HashMap::with_hasher(Default::default());
        map.extend(iter);
        map
    }
}

// // #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V, S> Extend<(K, V)> for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.base.extend(iter)
    }

    #[inline]
    fn extend_one(&mut self, (k, v): (K, V)) {
        self.base.insert(k, v);
    }

    #[inline]
    fn extend_reserve(&mut self, additional: usize) {
        self.base.extend_reserve(additional);
    }
}

// #[stable(feature = "hash_extend_copy", since = "1.4.0")]
impl<'a, K, V, S> Extend<(&'a K, &'a V)> for HashMap<K, V, S>
where
    K: Eq + Hash + Copy,
    V: Copy,
    S: BuildHasher,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        self.base.extend(iter)
    }

    #[inline]
    fn extend_one(&mut self, (&k, &v): (&'a K, &'a V)) {
        self.base.insert(k, v);
    }

    #[inline]
    fn extend_reserve(&mut self, additional: usize) {
        Extend::<(K, V)>::extend_reserve(self, additional)
    }
}

// #[derive(Clone)]
// #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
pub struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    pub fn new() -> RandomState {
        let rng = random();
        RandomState { k0: rng as u64, k1: (rng >> 64) as u64 }
    }
}

// #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;
    #[inline]
    #[allow(deprecated)]
    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(self.k0, self.k1))
    }
}

// #[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct DefaultHasher(SipHasher13);

impl DefaultHasher {


    // #[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
    #[inline]
    #[allow(deprecated)]
    // #[rustc_const_unstable(feature = "const_hash", issue = "104061")]
    #[must_use]
    pub const fn new() -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(0, 0))
    }
}

// #[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
impl Default for DefaultHasher {
    #[inline]
    fn default() -> DefaultHasher {
        DefaultHasher::new()
    }
}

// #[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
impl Hasher for DefaultHasher {
    // The underlying `SipHasher13` doesn't override the other
    // `write_*` methods, so it's ok not to forward them here.

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.write(msg)
    }

    // #[inline]
    // fn write_str(&mut self, s: &str) {
    //     self.0.write_str(s);
    // }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}

// #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
impl Default for RandomState {
    #[inline]
    fn default() -> RandomState {
        RandomState::new()
    }
}

// #[stable(feature = "std_debug", since = "1.16.0")]
// impl fmt::Debug for RandomState {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("RandomState").finish_non_exhaustive()
//     }
// }

// #[inline]
// fn map_entry<'a, K: 'a, V: 'a>(raw: base::RustcEntry<'a, K, V>) -> Entry<'a, K, V> {
//     match raw {
//         base::RustcEntry::Occupied(base) => Entry::Occupied(OccupiedEntry { base }),
//         base::RustcEntry::Vacant(base) => Entry::Vacant(VacantEntry { base }),
//     }
// }

// #[inline]
// pub(super) fn map_try_reserve_error(err: hashbrown::TryReserveError) -> TryReserveError {
//     match err {
//         hashbrown::TryReserveError::CapacityOverflow => {
//             TryReserveErrorKind::CapacityOverflow.into()
//         }
//         hashbrown::TryReserveError::AllocError { layout } => {
//             TryReserveErrorKind::AllocError { layout, non_exhaustive: () }.into()
//         }
//     }
// }

#[inline]
fn map_raw_entry<'a, K: 'a, V: 'a, S: 'a>(
    raw: base::RawEntryMut<'a, K, V, S>,
) -> RawEntryMut<'a, K, V, S> {
    match raw {
        base::RawEntryMut::Occupied(base) => RawEntryMut::Occupied(RawOccupiedEntryMut { base }),
        base::RawEntryMut::Vacant(base) => RawEntryMut::Vacant(RawVacantEntryMut { base }),
    }
}

// #[allow(dead_code)]
// fn assert_covariance() {
//     fn map_key<'new>(v: HashMap<&'static str, u8>) -> HashMap<&'new str, u8> {
//         v
//     }
//     fn map_val<'new>(v: HashMap<u8, &'static str>) -> HashMap<u8, &'new str> {
//         v
//     }
//     fn iter_key<'a, 'new>(v: Iter<'a, &'static str, u8>) -> Iter<'a, &'new str, u8> {
//         v
//     }
//     fn iter_val<'a, 'new>(v: Iter<'a, u8, &'static str>) -> Iter<'a, u8, &'new str> {
//         v
//     }
//     fn into_iter_key<'new>(v: IntoIter<&'static str, u8>) -> IntoIter<&'new str, u8> {
//         v
//     }
//     fn into_iter_val<'new>(v: IntoIter<u8, &'static str>) -> IntoIter<u8, &'new str> {
//         v
//     }
//     fn keys_key<'a, 'new>(v: Keys<'a, &'static str, u8>) -> Keys<'a, &'new str, u8> {
//         v
//     }
//     fn keys_val<'a, 'new>(v: Keys<'a, u8, &'static str>) -> Keys<'a, u8, &'new str> {
//         v
//     }
//     fn values_key<'a, 'new>(v: Values<'a, &'static str, u8>) -> Values<'a, &'new str, u8> {
//         v
//     }
//     fn values_val<'a, 'new>(v: Values<'a, u8, &'static str>) -> Values<'a, u8, &'new str> {
//         v
//     }
//     fn drain<'new>(
//         d: Drain<'static, &'static str, &'static str>,
//     ) -> Drain<'new, &'new str, &'new str> {
//         d
//     }
// }