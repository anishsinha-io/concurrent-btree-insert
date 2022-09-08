#![allow(unused_variables, dead_code, unused_imports)]

use buffers::node::Node;
use buffers::ptr::ItemPtr;

mod buffers;

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
