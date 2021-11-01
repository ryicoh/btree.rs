use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter, Result},
};

#[derive(Eq, Clone)]
pub struct KeyValue<K, V>
where
    K: Ord + Debug,
    V: Eq,
{
    pub key: K,
    pub value: V,
}

impl<K, V> KeyValue<K, V>
where
    K: Ord + Debug,
    V: Eq,
{
}

impl<K, V> PartialOrd for KeyValue<K, V>
where
    K: Ord + Debug,
    V: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K, V> PartialEq for KeyValue<K, V>
where
    K: Ord + Debug,
    V: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K, V> Ord for KeyValue<K, V>
where
    K: Ord + Debug,
    V: Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.key == other.key {
            return Ordering::Equal;
        }

        if self.key > other.key {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
}

impl<K, V> Debug for KeyValue<K, V>
where
    K: Ord + Debug,
    V: Eq,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.key)
    }
}

#[test]
fn test_greater_than() {
    let i: i64 = 10;
    assert_eq!(i.cmp(&9), Ordering::Greater);
    assert_eq!(i.cmp(&10), Ordering::Equal);
    assert_eq!(i.cmp(&11), Ordering::Less);
}
