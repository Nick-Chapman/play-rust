
use std::sync::Mutex;

pub fn main() {
    println!("**fib tree**");
    let a : Arena<Tree> = Arena::new(Tree::Empty);
    let tree = make_fib_tree(&a,10);
    //println!["tree: {:?}", tree];
    let res = sum_tree(tree);
    println!["res: {}", res];
}

fn make_fib_tree<'a>(a: &'a Arena<Tree<'a>>, n: u32) -> &'a Tree<'a> {
    use Tree::*;
    match n {
        0 => a.alloc(Leaf(0)),
        1 => a.alloc(Leaf(1)),
        n => {
            let left = make_fib_tree(a,n-2);
            let right = make_fib_tree(a,n-1);
            a.alloc(Node(left,right))
        }
    }
}

#[derive(Debug,Copy,Clone)]
enum Tree<'a> {
    Empty,
    Leaf(u32),
    Node(&'a Tree<'a>,&'a Tree<'a>)
}

fn sum_tree(tree: &Tree) -> u32 {
    match *tree {
        Tree::Empty => 0,
        Tree::Leaf(n) => n,
        Tree::Node(t1,t2) => sum_tree(t1) + sum_tree(t2)
    }
}

struct Arena<T> {
    buffer: Mutex<Buffer<T>>,
}

impl<T:Copy> Arena<T> {
    fn new(t0: T) -> Arena<T> {
        Arena {
            buffer: Mutex::new(Buffer::new(t0)),
        }
    }
    fn alloc(&self, thing: T) -> &T {
        let buffer = &mut self.buffer.lock().unwrap();
        let next = buffer.next;
        buffer.next += 1;
        let hold = &mut buffer.array[next];
        *hold = thing;
        unsafe { & * (hold as *const T) }
    }
}

const BUFFER_SIZE : usize = 200;

struct Buffer<T> {
    next: usize,
    array: [T;BUFFER_SIZE],
}

impl<T:Copy> Buffer<T> {
    fn new(t0: T) -> Buffer<T> {
        Buffer { next: 0, array: [t0;BUFFER_SIZE] }
    }
}
