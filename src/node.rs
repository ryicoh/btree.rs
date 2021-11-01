use std::{
    fmt::{Debug, Formatter, Result},
    mem::replace,
    ptr,
};

pub struct Node<Item: Ord> {
    pub items: Vec<Item>,
    pub children: Vec<Node<Item>>,
    pub capacity: usize,
}

impl<Item> Debug for Node<Item>
where
    Item: Ord + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}\n", self.items).unwrap();
        for child in &self.children {
            write!(f, "{}", child.fmt_with_level(1)).unwrap();
        }
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum PutResult<Item>
where
    Item: Ord + Debug,
{
    Putting(usize, Vec<Item>, Item, Vec<Item>),
    Updated,
    Inserted,
}

impl<Item> Node<Item>
where
    Item: Ord + Debug,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            children: Vec::with_capacity(capacity + 1),
            items: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn fmt_with_level(&self, level: usize) -> String {
        let mut s = format!("{}{:?}\n", " - ".repeat(level), self.items);
        for child in &self.children {
            s += &child.fmt_with_level(level + 1);
        }

        return s;
    }

    // fn search(&self, item: &Item) -> (usize, bool) {
    //     for (i, it) in self.items.iter().enumerate() {
    //         let cp = it.cmp(item);
    //         if cp != Ordering::Less {
    //             return (i, cp == Ordering::Equal);
    //         }
    //     }

    //     (self.items.len(), false)
    // }

    //fn _search(&self, item: &Item, start: usize, end: usize) -> (usize, bool) {
    //    let cursor = start + (end - start) / 2;

    //    if cursor == start {
    //        return match item.cmp(&self.items[start]) {
    //            Ordering::Greater => match item.cmp(&self.items[end]) {
    //                Ordering::Less => (end, false),
    //                Ordering::Equal => (end, true),
    //                Ordering::Greater => (end + 1, false),
    //            },
    //            Ordering::Equal => (cursor, true),
    //            Ordering::Less => (start, false),
    //        };
    //    }

    //    match item.cmp(&self.items[cursor]) {
    //        Ordering::Less => self._index_equal_or_greater_than(item, start, cursor),
    //        Ordering::Equal => {
    //            return (cursor, true);
    //        }
    //        Ordering::Greater => self._index_equal_or_greater_than(item, cursor, end),
    //    }
    //}

    //fn index_equal_or_greater_than(&self, item: &Item) -> (usize, bool) {
    //    if self.items.len() == 0 {
    //        return (0, false);
    //    }
    //    self._index_equal_or_greater_than(item, 0, self.items.len() - 1)
    //}

    fn search(&self, item: &Item) -> (usize, bool) {
        let res = self.items.binary_search(item);
        if res.is_err() {
            return (res.err().unwrap(), false);
        }

        (res.unwrap(), true)
    }

    fn is_items_filled(&self) -> bool {
        self.items.len() == self.items.capacity()
    }

    fn is_children_filled(&self) -> bool {
        self.children.len() == self.children.capacity()
    }

    fn new_items(&self) -> Vec<Item> {
        Vec::with_capacity(self.items.capacity())
    }

    fn new_node(&self) -> Node<Item> {
        Node::new(self.items.capacity())
    }

    pub fn get(&mut self, item: &Item) -> Option<&Item> {
        let (idx, found) = self.search(item);
        if found {
            return Some(unsafe { &self.items.get_unchecked(idx) });
        }

        if idx + 1 > self.children.len() {
            return None;
        }

        unsafe { self.children.get_unchecked_mut(idx) }.get(item)
    }

    pub fn put(&mut self, item: Item, parent_no_space: bool) -> PutResult<Item> {
        let (cursor, exists) = self.search(&item);
        if exists {
            let _ = replace(&mut self.items[cursor], item);
            return PutResult::Updated;
        }

        let res = PutResult::Inserted;
        if self.children.is_empty() {
            self.items.insert(cursor, item);
            debug_assert!(self.items.len() <= self.capacity);
        } else {
            let is_max = self.is_items_filled();
            let res = self.children[cursor].put(item, is_max);
            match res {
                PutResult::Putting(_child_cursor, mut left, center, mut right) => {
                    self.items.insert(cursor, center);
                    self.children[cursor].items.append(&mut left);
                    let mut right_node = self.new_node();
                    right_node.items.append(&mut right);
                    self.children.insert(cursor + 1, right_node);
                    debug_assert!(self.children.len() <= self.capacity + 1);
                }
                PutResult::Updated => return PutResult::Updated,
                PutResult::Inserted => return PutResult::Inserted,
            }
        }

        if !self.is_items_filled() || self.is_children_filled() {
            debug_assert!(self.items.len() <= self.capacity);
            return res;
        }
        let (mut left, center, mut right) = self.split_three_items();

        if parent_no_space {
            let mut left_node = self.new_node();
            let mut right_node = self.new_node();
            left_node.items.append(&mut left);
            right_node.items.append(&mut right);
            self.children.push(left_node);
            self.children.push(right_node);
            self.items.push(center);
            debug_assert!(self.items.len() <= self.capacity);
            debug_assert!(self.children.len() <= self.capacity + 1);
            return res;
        }

        PutResult::Putting(cursor, left, center, right)
    }

    pub fn split_three_items(&mut self) -> (Vec<Item>, Item, Vec<Item>) {
        let half = self.items.len() / 2;
        let at = half + 1;
        let other_len = self.items.len() - at;
        let mut left = self.new_items();
        let mut right = self.new_items();
        unsafe {
            self.items.set_len(0);
            left.set_len(other_len);
            right.set_len(other_len);

            ptr::copy_nonoverlapping(self.items.as_ptr(), left.as_mut_ptr(), other_len);
            ptr::copy_nonoverlapping(self.items.as_ptr().add(at), right.as_mut_ptr(), other_len);
            let center = ptr::read(self.items.as_ptr().add(half));
            (left, center, right)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::item::KeyValue;

    #[test]
    fn split_three_items() {
        let mut node = Node::<i64>::new(3);
        node.items = vec![2, 4, 6];
        let (left, center, right) = node.split_three_items();
        assert!(node.items.is_empty());
        assert_eq!(left, vec![2]);
        assert_eq!(center, 4);
        assert_eq!(right, vec![6]);

        node = Node::<i64>::new(5);
        node.items = vec![2, 4, 6, 8, 10];
        let (left, center, right) = node.split_three_items();
        assert!(node.items.is_empty());
        assert_eq!(left, vec![2, 4]);
        assert_eq!(center, 6);
        assert_eq!(right, vec![8, 10]);
    }

    #[test]
    fn new_node() {
        let node = Node::<i64>::new(3);
        assert!(node.children.len() == 0);
        assert!(node.items.capacity() == 3);
        assert!(node.children.capacity() == 4);
    }

    #[test]
    fn search() {
        let mut node = Node::<i64>::new(5);
        node.items = vec![2, 4, 6, 8, 10];
        assert_eq!(node.search(&1), (0, false));
        assert_eq!(node.search(&2), (0, true));
        assert_eq!(node.search(&3), (1, false));
        assert_eq!(node.search(&4), (1, true));
        assert_eq!(node.search(&5), (2, false));
        assert_eq!(node.search(&6), (2, true));
        assert_eq!(node.search(&7), (3, false));
        assert_eq!(node.search(&8), (3, true));
        assert_eq!(node.search(&9), (4, false));
        assert_eq!(node.search(&10), (4, true));
        assert_eq!(node.search(&11), (5, false));
    }

    #[test]
    fn is_items_filled() {
        let mut node = Node::<i64>::new(3);
        assert_eq!(node.is_items_filled(), false);
        node.items = vec![2, 4, 6, 8];
        assert_eq!(node.is_items_filled(), true);
    }

    #[test]
    fn is_children_filled() {
        let mut node = Node::<i64>::new(3);
        node.children = vec![
            Node::<i64>::new(3),
            Node::<i64>::new(3),
            Node::<i64>::new(3),
        ];
    }

    #[test]
    fn put_when_capacity_is_3() {
        let mut node = Node::<KeyValue<&[u8], &[u8]>>::new(3);

        let kv1 = KeyValue {
            key: "hello1".as_bytes(),
            value: "rust1".as_bytes(),
        };
        node.put(kv1.clone(), true);
        assert_eq!(node.items, vec![kv1.clone()]);

        let kv2 = KeyValue {
            key: "hello2".as_bytes(),
            value: "rust2".as_bytes(),
        };
        node.put(kv2.clone(), true);
        assert_eq!(node.items, vec![kv1.clone(), kv2.clone()]);

        let kv3 = KeyValue {
            key: "hello3".as_bytes(),
            value: "rust3".as_bytes(),
        };
        node.put(kv3.clone(), true);
        assert_eq!(node.items, vec![kv2.clone()]);
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].items, vec![kv1.clone()]);
        assert_eq!(node.children[1].items, vec![kv3.clone()]);

        let kv4 = KeyValue {
            key: "hello4".as_bytes(),
            value: "rust4".as_bytes(),
        };
        node.put(kv4.clone(), true);
        assert_eq!(node.items, vec![kv2.clone()]);
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].items, vec![kv1.clone()]);
        assert_eq!(node.children[1].items, vec![kv3.clone(), kv4.clone()]);

        let newkv2 = KeyValue {
            key: "hello2".as_bytes(),
            value: "new_rust2".as_bytes(),
        };
        node.put(newkv2.clone(), true);
        assert_eq!(node.items, vec![newkv2.clone()]);
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].items, vec![kv1.clone()]);
        assert_eq!(node.children[1].items, vec![kv3.clone(), kv4.clone()]);

        let kv5 = KeyValue {
            key: "hello5".as_bytes(),
            value: "rust5".as_bytes(),
        };
        node.put(kv5.clone(), true);
        assert_eq!(node.items, vec![newkv2.clone(), kv4.clone()]);
        assert_eq!(node.children.len(), 3);
        assert_eq!(node.children[0].items, vec![kv1.clone()]);
        assert_eq!(node.children[1].items, vec![kv3.clone()]);
        assert_eq!(node.children[2].items, vec![kv5.clone()]);
        let kv6 = KeyValue {
            key: "hello6".as_bytes(),
            value: "rust6".as_bytes(),
        };
        node.put(kv6.clone(), true);
        assert_eq!(node.items, vec![newkv2.clone(), kv4.clone()]);
        assert_eq!(node.children.len(), 3);
        assert_eq!(node.children[0].items, vec![kv1.clone()]);
        assert_eq!(node.children[1].items, vec![kv3.clone()]);
        assert_eq!(node.children[2].items, vec![kv5.clone(), kv6.clone()]);
        let kv7 = KeyValue {
            key: "hello7".as_bytes(),
            value: "rust7".as_bytes(),
        };

        node.put(kv7.clone(), true);
        assert_eq!(node.items, vec![newkv2.clone(), kv4.clone(), kv6.clone()]);
        assert_eq!(node.children.len(), 4);
        assert_eq!(node.children[0].items, vec![kv1.clone()]);
        assert_eq!(node.children[1].items, vec![kv3.clone()]);
        assert_eq!(node.children[2].items, vec![kv5.clone()]);
        assert_eq!(node.children[3].items, vec![kv7.clone()]);
    }

    #[test]
    fn put_when_capacity_is_5() {
        let mut node = Node::<KeyValue<&[u8], &[u8]>>::new(5);

        let kv1 = KeyValue {
            key: "hello".as_bytes(),
            value: "rust".as_bytes(),
        };
        node.put(kv1.clone(), true);
        assert_eq!(node.items, vec![kv1.clone()]);

        let kv2 = KeyValue {
            key: "hello2".as_bytes(),
            value: "rust2".as_bytes(),
        };
        node.put(kv2.clone(), true);
        assert_eq!(node.items, vec![kv1.clone(), kv2.clone()]);

        let kv3 = KeyValue {
            key: "hello3".as_bytes(),
            value: "rust3".as_bytes(),
        };
        node.put(kv3.clone(), true);
        assert_eq!(node.items, vec![kv1.clone(), kv2.clone(), kv3.clone()]);

        let kv4 = KeyValue {
            key: "hello4".as_bytes(),
            value: "rust4".as_bytes(),
        };
        node.put(kv4.clone(), true);
        assert_eq!(
            node.items,
            vec![kv1.clone(), kv2.clone(), kv3.clone(), kv4.clone()]
        );

        let kv5 = KeyValue {
            key: "hello5".as_bytes(),
            value: "rust5".as_bytes(),
        };
        node.put(kv5.clone(), true);
        assert_eq!(node.items, vec![kv3.clone()]);
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].items, vec![kv1.clone(), kv2.clone()]);
        assert_eq!(node.children[1].items, vec![kv4.clone(), kv5.clone()]);
    }

    #[test]
    fn get_when_capacity_is_5() {
        let mut node = Node::<KeyValue<String, String>>::new(5);

        let kv_list: Vec<KeyValue<String, String>> = (0..100)
            .map(|i| KeyValue {
                key: format!("hello{}", i),
                value: format!("rust{}", i),
            })
            .collect();

        for kv in &kv_list {
            node.put(kv.clone(), true);
        }
        for kv in &kv_list {
            assert_eq!(node.get(&kv), Some(kv));
        }

        assert_eq!(
            node.get(&KeyValue {
                key: "hello".to_string(),
                value: "not_found".to_string(),
            }),
            None,
        );
    }
}
