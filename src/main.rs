extern crate rustc_serialize;
extern crate rand;

use rustc_serialize::hex::*;
use std::fmt::{Error, Display, Formatter};

const ID_LEN: usize = 20;

#[derive(Debug,Ord,PartialOrd,Eq,PartialEq,Copy,Clone)]
struct NodeID([u8; ID_LEN]);

fn main() {
    let j = new_random_node_id();
    let k = new_random_node_id();
    println!("{}", j);
    println!("{}", k);
    println!("{:?}", j.cmp(&k));
    println!("{:?}", j.cmp(&j));
}

fn new_node_id(input: &str) -> Result<NodeID, &str> {
    let mut input = match input.from_hex() {
        Err(_) => { return Err("Input was not valid hex"); }
        Ok(x) => { x }
    };

    input.reverse();

    if ID_LEN != input.len() {
        return Err("The length of the input doesn't correspond to the ID_LEN");
    };

    let mut res = [0; ID_LEN];
    for i in 0us..ID_LEN {
        res[i] = input[i];
    }
    Ok(NodeID(res))
}

fn new_random_node_id() -> NodeID {
    let mut res = [0; ID_LEN];
    for i in 0us..ID_LEN {
        res[i] = rand::random::<u8>();
    }
    NodeID(res)
}

impl Display for NodeID {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for x in self.0.iter().rev() {
            try!(write!(f, "{0:02x}", x));
        }
        Ok(())
    }
}

fn dist(x: &NodeID, y: &NodeID) -> NodeID{
    let mut res = [0; ID_LEN];
    for i in 0us..ID_LEN {
        res[i] = x.0[i] ^ y.0[i];
    }
    NodeID(res)
}
