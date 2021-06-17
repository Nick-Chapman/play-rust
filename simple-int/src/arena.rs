
use std::fmt;
use std::sync::Mutex;
use std::marker;

pub fn main() {
    println!("**arena experiments** (after reading https://os.phil-opp.com)");
    play_safe();
}

pub fn play_safe() {
    let a : Arena = Arena::new();
    println!["things: {}",a];
    let x = a.alloc(Thing::new(100));
    let y = a.alloc(Thing::new(200));
    let z = a.alloc(Thing::new(300));
    println!["things: {}",a];
    println!["elems: {} {} {}", x, y, z];
}

struct Arena<'a> {
    buffer: Mutex<Buffer>,
    _marker: marker::PhantomData<&'a Thing>,
}

impl<'a> Arena<'a> {
    fn new() -> Arena<'a> {
        Arena {
            buffer: Mutex::new(Buffer::new()),
            _marker: marker::PhantomData
        }
    }
    fn alloc(&'a self, thing: Thing) -> &'a Thing {
        let buffer = &mut self.buffer.lock().unwrap();
        let next = buffer.next;
        buffer.next += 1;
        let hold = &mut buffer.array[next];
        *hold = thing;
        unsafe { & * (hold as *const Thing) }
    }
}

impl fmt::Display for Arena<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write![f,"Arena({})[",self.buffer.lock().unwrap()]
    }
}

const BUFFER_SIZE : usize = 5;

#[derive(Debug)]
struct Buffer {
    next: usize,
    array: [Thing;BUFFER_SIZE],
}

impl Buffer {
    fn new() -> Buffer {
        let def = Thing::new(0);
        Buffer { next: 0, array: [def;BUFFER_SIZE] }
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write![f,"Buffer(next={})[",self.next]?;
        let mut iter = self.array.iter();
        match iter.next() {
            None => (),
            Some(elem1) => {
                write![f,"{}",elem1]?;
                for elem in iter {
                    write![f,",{}",elem]?;
                }
            }
        }
        write![f,"]"]?;
        Ok(())
    }
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

impl Thing {
    fn new(x:u128) -> Thing { Thing { x } }
}
