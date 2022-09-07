#![allow(unused_variables, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
struct ItemPtr {
    page_no: i32,
    offset: usize,
}

impl Display for ItemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ItemPtr [page_no={} offset={}]",
            self.page_no, self.offset
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Node<T> {
    leaf: bool,
    order: u32,
    loc: ItemPtr,
    link: ItemPtr,
    high_key: T,
    keys: Vec<T>,
    children: Vec<ItemPtr>,
}

impl<'a, T> Node<T>
where
    T: Deserialize<'a> + Serialize,
{
    pub fn encode(self) -> Option<Vec<u8>> {
        let item = bincode::serialize(&self);
        match item {
            Ok(item) => Some(item),
            Err(_) => None,
        }
    }

    pub fn decode(bytes: &'a Vec<u8>) -> Option<Self> {
        let cloned: Vec<u8> = bytes.clone();
        let page = bincode::deserialize(&bytes[..]);
        match page {
            Ok(page) => Some(page),
            Err(_) => None,
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node [leaf={} order={} loc={} link={} high_key={} \nkeys={:#?} \nchildren={:#?}\n]",
            self.leaf, self.order, self.loc, self.link, self.high_key, self.keys, self.children
        )
    }
}

type Page = Arc<Mutex<[u8; 512]>>;

fn main() {
    let first = ItemPtr {
        page_no: 1,
        offset: 2,
    };

    let second = ItemPtr {
        page_no: 2,
        offset: 3,
    };

    let third = ItemPtr {
        page_no: 3,
        offset: 4,
    };

    println!("{} \n{}", first, second);

    let test_node: Node<u32> = Node {
        leaf: false,
        order: 2,
        loc: first,
        link: second,
        high_key: 3,
        keys: vec![1, 2, 3],
        children: vec![third],
    };

    println!("{}", test_node);

    if let Some(encoded) = test_node.encode() {
        println!("{:#?}", encoded);
        if let Some(decoded) = Node::<u32>::decode(&encoded) {
            println!("DECODED: {}", decoded);
        }
    }
}
