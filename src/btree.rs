use std::fmt::{Debug, Formatter, Result};

use crate::{node::Node, PutResult};

pub struct Btree<Item: Ord> {
    root: Node<Item>,
    length: usize,
}

impl<Item> Debug for Btree<Item>
where
    Item: Ord + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "btree\n{:?}", self.root)
    }
}

impl<Item> Btree<Item>
where
    Item: Ord + Debug,
{
    pub fn new(size: usize) -> Self {
        Self {
            root: Node::new(size),
            length: 0,
        }
    }

    pub fn get(&mut self, item: &Item) -> Option<&Item> {
        self.root.get(item)
    }

    pub fn put(&mut self, item: Item) {
        let res = self.root.put(item, true);
        if res == PutResult::Inserted {
            self.length += 1;
        }
    }

    pub fn len(self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use crate::Btree;

    #[test]
    fn new_btree() {
        let btree = Btree::<i64>::new(3);
        assert_eq!(btree.root.items.len(), 0);
        assert_eq!(btree.root.children.len(), 0);
    }

    #[test]
    fn len() {
        let mut btree = Btree::<i64>::new(3);
        for i in 0..100 {
            btree.put(i.clone() as i64);
        }

        assert_eq!(btree.len(), 100);
    }
}
