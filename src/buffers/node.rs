use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::buffers::encoding;
use crate::buffers::ptr::ItemPtr;

#[derive(Serialize, Deserialize, Clone)]
pub struct Node<T> {
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
        encoding::bytes::encode(&self)
    }

    pub fn decode(bytes: &Vec<u8>) -> Option<Self> {
        encoding::bytes::decode(bytes.to_vec())
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
