use cc_traits::*;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BinarySet<T>(Vec<T>);

impl<T> AsRef<Vec<T>> for BinarySet<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T: Ord + PartialOrd + Eq + PartialEq> Extend<T> for BinarySet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
        self.0.sort() //TODO performance
    }
}

impl<T> BinarySet<T> {}

impl<T: Ord + PartialOrd + Eq + PartialEq> FromIterator<T> for BinarySet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec: Vec<T> = Vec::from_iter(iter);
        vec.sort();

        Self(vec)
    }
}

impl<T> Reserve for BinarySet<T> {
    fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }
}

impl<T> WithCapacity for BinarySet<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<T> Capacity for BinarySet<T> {
    fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<T> Iter for BinarySet<T> {
    type Iter<'a> = core::slice::Iter<'a, T>
	    where
		    Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter()
    }
}

impl<T: Ord + PartialOrd + Eq + PartialEq> Insert for BinarySet<T> {
    type Output = bool;

    fn insert(&mut self, element: Self::Item) -> Self::Output {
        match self.0.binary_search(&element) {
            Ok(_) => false,
            Err(index) => {
                self.0.insert(index, element);
                true
            }
        }
    }
}

impl<'a, T: Ord + PartialOrd + Eq + PartialEq> Remove<&'a T> for BinarySet<T> {
    fn remove(&mut self, key: &'a T) -> Option<Self::Item> {
        match self.0.binary_search(&key) {
            Ok(index) => Some(self.0.remove(index)),
            Err(_) => None,
        }
    }
}

impl<T: Ord + PartialOrd + Eq + PartialEq> Get<T> for BinarySet<T> {
    fn get(&self, key: T) -> Option<Self::ItemRef<'_>> {
        self.get(&key)
    }
}

impl<'a, T: Ord + PartialOrd + Eq + PartialEq> Get<&'a T> for BinarySet<T> {
    fn get(&self, key: &'a T) -> Option<Self::ItemRef<'_>> {
        let index = self.0.binary_search(key).ok()?;
        self.0.get(index)
    }
}

impl<T> CollectionRef for BinarySet<T> {
    type ItemRef<'a>= &'a T
	    where
		    Self: 'a ;

    covariant_item_ref!();
}

impl<T> Collection for BinarySet<T> {
    type Item = T;
}
impl<T> Len for BinarySet<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn len() {
        let set = BinarySet::from_iter([1, 2, 3]);
        assert_eq!(set.len(), 3)
    }

    #[test]
    pub fn get() {
        let set = BinarySet::from_iter([1, 2, 3]);

        assert_eq!(set.get(2), Some(&2));
        assert_eq!(set.get(4), None);
    }

    #[test]
    pub fn reserve() {
        let mut set = BinarySet::from_iter([1, 2, 3]);
        set.reserve(1000);
        assert!(set.capacity() >= 1003);
    }

    #[test]
    pub fn with_capacity() {
        let set: BinarySet<u8> = BinarySet::with_capacity(100);
        assert!(set.capacity() >= 100);
    }

    #[test]
    pub fn iter() {
        let set = BinarySet::from_iter([1, 2, 3]);
        let vec: Vec<i32> = set.iter().cloned().collect();
        assert_eq!(vec, vec![1, 2, 3])
    }

    #[test]
    pub fn extend() {
        let mut set = BinarySet::from_iter([2, 4, 6, 8]);
        set.extend([1, 3, 5, 7, 9]);

        assert_eq!(set.as_ref(), &vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    pub fn insert() {
        let mut set = BinarySet::from_iter([2, 3]);
        assert_eq!(set.insert(1), true);
        assert_eq!(set.insert(2), false);

        assert_eq!(set.as_ref(), &vec![1, 2, 3])
    }

    #[test]
    pub fn remove() {
        let mut set = BinarySet::from_iter([1, 2, 3]);
        assert_eq!(set.remove(&2), Some(2));
        assert_eq!(set.remove(&4), None);

        assert_eq!(set.as_ref(), &vec![1, 3])
    }
}
