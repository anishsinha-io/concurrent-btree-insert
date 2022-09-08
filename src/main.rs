//!
//!  Driver code for generating Lehman and Yao trees as well as running several benchmarks
//!
//! Lehman and Yao Trees are variants of B+Trees used in database systems for high concurrency workloads. For example, PostgreSQL
//! uses a slightly modified version of a Lehman and Yao tree as its default index on primary keys.
//!
//! This repository creates Lehman and Yao trees and implements the search and insert functions. Deletion is not supported here, but
//! for resources on deletion, consult [`A Symmetric Concurrent B-Tree Algorithm`]: https://dl.acm.org/doi/pdf/10.5555/324493.324589 by
//! Vladimir Lanin and Dennis Shasha, which is used for deletion logic within PostgreSQL default indexes.
//!
//! This project was created by Anish Sinha and Dr. Xiang Huang in 2022.
//!

#![allow(unused_variables, dead_code, unused_imports)]
#![warn(missing_docs)]

use buffers::node::Node;
use buffers::ptr::ItemPtr;

mod buffers;

fn main() {
    let first = ItemPtr::new(1);
    let second = ItemPtr::new(2);
    let third = ItemPtr::new(3);
    let fourth = ItemPtr::new(4);
    let fifth = ItemPtr::new(5);
    let sixth = ItemPtr::new(6);
    let seventh = ItemPtr::new(7);
    let eighth = ItemPtr::new(8);

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
