// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::ledger::map::{
    iterators::{Iter, Keys, Values},
    Map,
    MapReader,
};
use console::network::prelude::*;

use core::{borrow::Borrow, hash::Hash};
use std::{borrow::Cow, collections::hash_map::HashMap};

#[derive(Clone)]
pub struct MemoryMap<
    K: PartialEq + Eq + Hash + Serialize + for<'de> Deserialize<'de>,
    V: PartialEq + Eq + Serialize + for<'de> Deserialize<'de>,
> {
    pub(super) map: HashMap<K, V>,
}

impl<
    K: Clone + PartialEq + Eq + Hash + Serialize + for<'de> Deserialize<'de>,
    V: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de>,
> FromIterator<(K, V)> for MemoryMap<K, V>
{
    /// Initializes a new `MemoryMap` from the given iterator.
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self { map: HashMap::from_iter(iter) }
    }
}

impl<
    'a,
    K: 'a + Clone + PartialEq + Eq + Hash + Serialize + for<'de> Deserialize<'de>,
    V: 'a + Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de>,
> Map<'a, K, V> for MemoryMap<K, V>
{
    ///
    /// Inserts the given key-value pair into the map.
    ///
    fn insert(&mut self, key: K, value: V) -> Result<()> {
        self.map.insert(key, value);

        Ok(())
    }

    ///
    /// Removes the key-value pair for the given key from the map.
    ///
    fn remove<Q>(&mut self, key: &Q) -> Result<()>
    where
        K: Borrow<Q>,
        Q: PartialEq + Eq + Hash + Serialize + ?Sized,
    {
        self.map.remove(key);

        Ok(())
    }
}

impl<
    'a,
    K: 'a + Clone + PartialEq + Eq + Hash + Serialize + for<'de> Deserialize<'de>,
    V: 'a + Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de>,
> MapReader<'a, K, V> for MemoryMap<K, V>
{
    type Iterator = Iter<'a, K, V>;
    type Keys = Keys<'a, K, V>;
    type Values = Values<'a, K, V>;

    ///
    /// Returns `true` if the given key exists in the map.
    ///
    fn contains_key<Q>(&self, key: &Q) -> Result<bool>
    where
        K: Borrow<Q>,
        Q: PartialEq + Eq + Hash + Serialize + ?Sized,
    {
        Ok(self.map.contains_key(key))
    }

    ///
    /// Returns the value for the given key from the map, if it exists.
    ///
    fn get<Q>(&'a self, key: &Q) -> Result<Option<Cow<'a, V>>>
    where
        K: Borrow<Q>,
        Q: PartialEq + Eq + Hash + Serialize + ?Sized,
    {
        Ok(self.map.get(key).map(Cow::Borrowed))
    }

    ///
    /// Returns an iterator visiting each key-value pair in the map.
    ///
    fn iter(&'a self) -> Self::Iterator {
        Iter::new(self.map.iter())
    }

    ///
    /// Returns an iterator over each key in the map.
    ///
    fn keys(&'a self) -> Self::Keys {
        Keys::new(self.iter())
    }

    ///
    /// Returns an iterator over each value in the map.
    ///
    fn values(&'a self) -> Self::Values {
        Values::new(self.iter())
    }
}

impl<
    K: Clone + PartialEq + Eq + Hash + Serialize + for<'de> Deserialize<'de>,
    V: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de>,
> Default for MemoryMap<K, V>
{
    fn default() -> Self {
        Self { map: HashMap::new() }
    }
}
