use std::ops::{IndexMut, RangeBounds};

use cc_traits::{
    covariant_item_mut, covariant_item_ref, covariant_key_ref, Capacity, Clear, Collection,
    CollectionMut, CollectionRef, Get, GetKeyValue, GetMut, Iter, Keyed, KeyedRef, Len, MapInsert,
    Remove, Reserve, WithCapacity,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BinaryMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: Ord, V> BinaryMap<K, V> {
    pub fn range(&self, range: impl RangeBounds<K>) -> impl Iterator<Item = (&K, &V)> {
        let start_inclusive: usize = match range.start_bound() {
            std::ops::Bound::Included(k) => match self.keys.binary_search(k) {
                Ok(i) => i,
                Err(i) => i,
            },
            std::ops::Bound::Excluded(k) => match self.keys.binary_search(k) {
                Ok(i) => i + 1, //do not include this key
                Err(i) => i,
            },
            std::ops::Bound::Unbounded => 0,
        };

        let (end, exclusive): (usize, bool) = match range.end_bound() {
            std::ops::Bound::Included(k) => match self.keys.binary_search(k) {
                Ok(i) => (i, false),
                Err(i) => (i, true),
            },
            std::ops::Bound::Excluded(k) => match self.keys.binary_search(k) {
                Ok(i) => (i, true),
                Err(i) => (i, true),
            },
            std::ops::Bound::Unbounded => (self.len(), true),
        };

        if exclusive {
            self.keys[start_inclusive..end]
                .iter()
                .zip(self.values[start_inclusive..end].iter())
        } else {
            self.keys[start_inclusive..=end]
                .iter()
                .zip(self.values[start_inclusive..=end].iter())
        }
    }
}

impl<K, V> BinaryMap<K, V> {
    pub const fn keys(&self) -> &Vec<K> {
        &self.keys
    }

    pub const fn values(&self) -> &Vec<V> {
        &self.values
    }

    pub fn into_keys(self) -> Vec<K>{
        self.keys
    }
    
    pub fn into_values(self) -> Vec<V>{
        self.values
    }

}

// impl<K, V> Map<K, V> for BinaryMap<K, V>{

// }

impl<K, V> CollectionMut for BinaryMap<K, V> {
    type ItemMut<'a> = &'a mut V
	    where
		    Self: 'a;

    covariant_item_mut!();
}

impl<'a, K: Ord, V> GetMut<&'a K> for BinaryMap<K, V> {
    fn get_mut(&mut self, key: &'a K) -> Option<Self::ItemMut<'_>> {
        let index = self.keys.binary_search(key).ok()?;
        self.values.get_mut(index)
    }
}

impl<K: Ord, V> Get<K> for BinaryMap<K, V> {
    fn get(&self, key: K) -> Option<Self::ItemRef<'_>> {
        self.get(&key)
    }
}

impl<'a, K: Ord, V> Get<&'a K> for BinaryMap<K, V> {
    fn get(&self, key: &'a K) -> Option<Self::ItemRef<'_>> {
        let index = self.keys.binary_search(key).ok()?;
        self.values.get(index)
    }
}

impl<K: Ord, V> MapInsert<K> for BinaryMap<K, V> {
    type Output = Option<V>;

    fn insert(&mut self, key: K, mut value: Self::Item) -> Self::Output {
        match self.keys.binary_search(&key) {
            Ok(index) => {
                let position = self.values.index_mut(index);
                std::mem::swap(&mut value, position);

                Some(value)
            }
            Err(index) => {
                self.keys.insert(index, key);
                self.values.insert(index, value);
                None
            }
        }
    }
}

impl<'a, K: Ord, V> Remove<&'a K> for BinaryMap<K, V> {
    fn remove(&mut self, key: &'a K) -> Option<Self::Item> {
        let index = self.keys.binary_search(key).ok()?;
        self.keys.remove(index);
        Some(self.values.remove(index))
    }
}

impl<'a, K: Ord, V> GetKeyValue<&'a K> for BinaryMap<K, V> {
    fn get_key_value(&self, key: &'a K) -> Option<(Self::KeyRef<'_>, Self::ItemRef<'_>)> {
        let index = self.keys.binary_search(key).ok()?;

        let k = self.keys.get(index)?;
        let v = self.values.get(index)?;

        Some((k, v))
    }
}

impl<K: Ord, V> GetKeyValue<K> for BinaryMap<K, V> {
    fn get_key_value(&self, key: K) -> Option<(Self::KeyRef<'_>, Self::ItemRef<'_>)> {
        self.get_key_value(&key)
    }
}

impl<K, V> KeyedRef for BinaryMap<K, V> {
    type KeyRef<'a> = &'a K
	    where
		    Self: 'a;

    covariant_key_ref!();
}

impl<K, V> Keyed for BinaryMap<K, V> {
    type Key = K;
}

impl<K, V> Clear for BinaryMap<K, V> {
    fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }
}

impl<K, V> BinaryMap<K, V> {}

impl<K: Ord, V> FromIterator<(K, V)> for BinaryMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut vec: Vec<(K, V)> = Vec::from_iter(iter);
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        vec.dedup_by(|a, b| a.0.eq(&b.0));
        //todo don't allocate twice
        let (keys, values) = vec.into_iter().unzip();

        Self { keys, values }
    }
}

impl<K, V> Reserve for BinaryMap<K, V> {
    fn reserve(&mut self, additional: usize) {
        self.keys.reserve(additional);
        self.values.reserve(additional);
    }
}

impl<K, V> WithCapacity for BinaryMap<K, V> {
    fn with_capacity(capacity: usize) -> Self {
        let keys = Vec::with_capacity(capacity);
        let values = Vec::with_capacity(capacity);
        Self { keys, values }
    }
}

impl<K, V> Capacity for BinaryMap<K, V> {
    fn capacity(&self) -> usize {
        self.keys.capacity()
    }
}

impl<K, V> Iter for BinaryMap<K, V> {
    type Iter<'a> = core::slice::Iter<'a, V>
	    where
		    Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.values.iter()
    }
}

impl<K, V> CollectionRef for BinaryMap<K, V> {
    type ItemRef<'a>= &'a V
	    where
		    Self: 'a ;

    covariant_item_ref!();
}

impl<K, V> Collection for BinaryMap<K, V> {
    type Item = V;
}
impl<K, V> Len for BinaryMap<K, V> {
    fn len(&self) -> usize {
        self.keys.len()
    }
}

#[cfg(test)]
pub mod tests {
    use std::ops::Bound;

    use crate::map::*;

    #[test]
    pub fn len() {
        let set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c'), (2, 'd')]);
        assert_eq!(set.len(), 3)
    }

    #[test]
    pub fn get() {
        let set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c')]);

        assert_eq!(set.get(2), Some(&'b'));
        assert_eq!(set.get(4), None);
    }

    #[test]
    pub fn get_range() {
        let set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
        assert_eq!(
            set.range(2..=3).map(|x| x.1).cloned().collect::<Vec<_>>(),
            vec!['b', 'c']
        );
        assert_eq!(
            set.range(2..4).map(|x| x.1).cloned().collect::<Vec<_>>(),
            vec!['b', 'c']
        );
        assert_eq!(
            set.range((Bound::Excluded(1), Bound::Included(3)))
                .map(|x| x.1)
                .cloned()
                .collect::<Vec<_>>(),
            vec!['b', 'c']
        );
        assert_eq!(
            set.range(2..5).map(|x| x.1).cloned().collect::<Vec<_>>(),
            vec!['b', 'c', 'd']
        );
        assert_eq!(
            set.range(2..=5).map(|x| x.1).cloned().collect::<Vec<_>>(),
            vec!['b', 'c', 'd']
        );
        assert_eq!(
            set.range(2..).map(|x| x.1).cloned().collect::<Vec<_>>(),
            vec!['b', 'c', 'd']
        );
        assert_eq!(
            set.range(..3).map(|x| x.1).cloned().collect::<Vec<_>>(),
            vec!['a', 'b']
        );
        assert_eq!(
            set.range(-1..3).map(|x| x.1).cloned().collect::<Vec<_>>(),
            vec!['a', 'b']
        );
        assert_eq!(
            set.range((Bound::Excluded(-1), Bound::Excluded(3)))
                .map(|x| x.1)
                .cloned()
                .collect::<Vec<_>>(),
            vec!['a', 'b']
        );
    }

    #[test]
    pub fn get_key_value() {
        let set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c')]);

        assert_eq!(set.get_key_value(2), Some((&2, &'b')));
        assert_eq!(set.get(4), None);
    }

    #[test]
    pub fn get_mut() {
        let mut set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c')]);

        match set.get_mut(&2) {
            Some(v) => *v = 'd',
            None => panic!("Should hav matched"),
        };
        assert_eq!(set.get(&2), Some(&'d'));
    }

    #[test]
    pub fn reserve() {
        let mut set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c')]);
        set.reserve(1000);
        assert!(set.capacity() >= 1003);
    }

    #[test]
    pub fn with_capacity() {
        let set: BinaryMap<u8, char> = BinaryMap::with_capacity(100);
        assert!(set.capacity() >= 100);
    }

    #[test]
    pub fn iter() {
        let set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c')]);
        let vec: Vec<char> = set.iter().cloned().collect();
        assert_eq!(vec, vec!['a', 'b', 'c'])
    }

    // #[test]
    // pub fn extend() {
    //     let mut set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    //     set.extend([(1, 'e'), (2, 'f'), (3, 'g'), (4, 'h'), (5, 'i'), (7, 'j'), (9, 'k')]);

    //     assert_eq!(set.as_ref(), &vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    // }

    #[test]
    pub fn insert() {
        let mut set = BinaryMap::from_iter([(2, 'b'), (3, 'c')]);
        assert_eq!(set.insert(1, 'a'), None);
        assert_eq!(set.insert(2, 'd'), Some('b'));

        assert_eq!(set.values(), &vec!['a', 'd', 'c']);
        assert_eq!(set.keys(), &vec![1, 2, 3]);
    }

    #[test]
    pub fn remove() {
        let mut set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c')]);
        assert_eq!(set.remove(&2), Some('b'));
        assert_eq!(set.remove(&4), None);

        assert_eq!(set.values(), &vec!['a', 'c'])
    }

    #[test]
    pub fn clear() {
        let mut set = BinaryMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c')]);

        set.clear();

        assert_eq!(set.values(), &vec![])
    }
}
