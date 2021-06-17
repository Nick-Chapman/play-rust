
use std::fmt;
use crate::arena::Arena;

pub fn main() {
    println!("**arena experiments** (after reading https://os.phil-opp.com)");
    play_safe();
}

pub fn play_safe() {
    let a : Arena<Thing> = Arena::new(Thing::new(0));
    let x = a.alloc(Thing::new(100));
    let y = a.alloc(Thing::new(200));
    let z = a.alloc(Thing::new(300));
    println!["elems: {} {} {}", x, y, z];
}

impl Thing {
    fn new(x:u128) -> Thing { Thing { x } }
}

#[derive(Debug,Copy,Clone)]
pub struct Thing {
    x: u128
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write![f,"{}",self.x]
    }
}
