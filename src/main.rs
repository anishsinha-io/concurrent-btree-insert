#![allow(unused_variables, dead_code, unused_imports)]

use derivative::Derivative;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex, RwLock};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Derivative, PartialEq, PartialOrd, Eq, Ord,
)]
#[derivative(Default)]
struct ItemPtr {
    #[derivative(Default(value = "-1"))]
    page_no: i32,
    #[derivative(Default(value = "0"))]
    offset: usize,
}

impl ItemPtr {
    pub fn new(page_no: i32, offset: usize) -> Self {
        ItemPtr { page_no, offset }
    }
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

impl<T> Node<T>
where
    T: DeserializeOwned + Serialize + Ord + Clone + Copy,
{
    pub fn encode(self) -> Option<Vec<u8>> {
        let item = bincode::serialize(&self);
        match item {
            Ok(item) => Some(item),
            Err(_) => None,
        }
    }

    pub fn decode(bytes: &Vec<u8>) -> Option<Self> {
        let owned = bytes.clone().to_owned();
        let page = bincode::deserialize(&owned[..]);
        match page {
            Ok(page) => Some(page),
            Err(_) => None,
        }
    }

    pub fn new(
        order: u32,
        loc: ItemPtr,
        link: ItemPtr,
        keys: Vec<T>,
        children: Vec<ItemPtr>,
    ) -> Self {
        let high_key = *keys.iter().max().unwrap();
        Node {
            leaf: true,
            order,
            loc,
            link,
            keys,
            children,
            high_key,
        }
    }

    pub fn into_buffer(self) -> [u8; 512] {
        let bytes: Vec<u8> = self.encode().unwrap();
        let mut node_buf = [0u8; 512];
        node_buf[..std::mem::size_of_val(&*bytes)].copy_from_slice(&bytes);
        node_buf
    }

    pub fn from_buffer(bytes: &[u8; 512]) -> Node<T> {
        Node::<T>::decode(&bytes.to_vec()).unwrap()
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

type Page = [u8; 512];

struct BufferPoolFrame {
    dirty: bool,
    pins: i32,
    page_no: i32,
    page: [u8; 512],
}

impl BufferPoolFrame {
    fn new(page_no: i32, page: [u8; 512]) -> Self {
        BufferPoolFrame {
            dirty: false,
            pins: 0,
            page_no,
            page,
        }
    }
}

impl Display for BufferPoolFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BufferPoolFrame [dirty={} pins={} page_no={} page_size={}]",
            self.dirty,
            self.pins,
            self.page_no,
            std::mem::size_of_val(&self.page)
        )
    }
}

impl PartialEq for BufferPoolFrame {
    fn eq(&self, other: &Self) -> bool {
        self.page_no == other.page_no
    }
}

type BufferPool = Vec<Mutex<BufferPoolFrame>>;

fn main() {
    let first = ItemPtr::new(1, 2);
    let second = ItemPtr::new(2, 3);
    let third = ItemPtr::new(3, 4);
    let fourth = ItemPtr::new(4, 5);
    let fifth = ItemPtr::new(6, 7);
    let sixth = ItemPtr::new(7, 8);
    let seventh = ItemPtr::new(8, 9);
    let eighth = ItemPtr::new(9, 10);

    {
        let mut test = vec![eighth, seventh, sixth, fifth, fourth, third, second, first];
        println!("{:#?}", test);
        test.sort();
        println!("{:#?}", test);
    }

    let test_node = Node::new(
        2,
        first,
        second,
        vec![1, 2, 3, 4],
        vec![third, fourth, fifth, sixth, seventh],
    );

    println!("{}", std::mem::size_of_val(&first));
    println!("{}", std::mem::size_of_val(&test_node));

    let raw: [u8; 512] = test_node.into_buffer();
    println!("{:#?}", raw);
    let raw_decoded = Node::<u32>::from_buffer(&raw);
    println!("RAW DECODED: {}", raw_decoded);
}
