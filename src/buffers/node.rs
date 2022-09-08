use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::buffers::encoding;
use crate::buffers::ptr::ItemPtr;

///
/// Node represents how Lehman and Yao tree nodes are represented internally. Nodes are parameterized and can accept generic keys types.
/// Each reference to another node is given as a struct of type `ItemPtr` from `create::buffers::ptr::ItemPtr`. Each node also keeps track
/// of whether it's a leaf, its order, its high key, its keys (a vector of a generic type), and its children (a vector holding ItemPtrs).
///
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
    ///
    /// This function encodes a Node to raw bytes that are easy to write to files
    ///
    pub fn encode(self) -> Option<Vec<u8>> {
        encoding::bytes::encode(&self)
    }

    ///
    /// This function decodes a Node from a raw byte buffer
    ///
    pub fn decode(bytes: &Vec<u8>) -> Option<Self> {
        encoding::bytes::decode(bytes.to_vec())
    }

    ///
    /// This function constructs a new Node
    ///
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

    ///
    /// This function converts a node directly into a [u8; 512] that can be assigned to a frame's page. This function is less careful
    /// than the above functions and simply unwraps the result of `self.encode()` which may not be what you want. It's a shortcut for
    /// directly getting a slice of a specific size that contains the nodes logical contents as bytes.
    ///
    pub fn into_buffer(self) -> [u8; 512] {
        let bytes: Vec<u8> = self.encode().unwrap();
        let mut node_buf = [0u8; 512];
        node_buf[..std::mem::size_of_val(&*bytes)].copy_from_slice(&bytes);
        node_buf
    }

    ///
    /// This function creates, from a slice of bytes, a Node parameterized by a specific type. Like the function above it, this function
    /// is less careful and unwraps the result of Node::<T>::decode() which may not be what you want. It's a shortcut for directly turning
    /// a frame's `page` field into a Node.
    ///
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
