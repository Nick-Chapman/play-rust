
use std::sync::Mutex;
use std::marker;

pub fn main() {
    println!("**tree take2**");
    let res = {
        let a : Arena = Arena::new();
        let x = a.alloc(Tree::Leaf(11));
        let y = a.alloc(Tree::Leaf(22));
        let z = a.alloc(Tree::Node(x,y));
        let p = a.alloc(Tree::Node(y,z));
        let q = a.alloc(Tree::Node(p,p));
        sum_tree(q)
    };
    println!["res: {}", res];
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

#[derive(Debug)]
struct Arena<'a> {
    buffer: Mutex<Buffer<'a>>,
    _marker: marker::PhantomData<&'a Tree<'a>>,
}

impl<'a> Arena<'a> {
    fn new() -> Arena<'a> {
        Arena {
            buffer: Mutex::new(Buffer::new()),
            _marker: marker::PhantomData
        }
    }
    fn alloc(&self, thing: Tree<'a>) -> &'a Tree<'a> {
        let buffer = &mut self.buffer.lock().unwrap();
        let next = buffer.next;
        buffer.next += 1;
        let hold = &mut buffer.array[next];
        *hold = thing;
        unsafe { & * (hold as *const Tree) }
    }
}

const BUFFER_SIZE : usize = 10;

#[derive(Debug)]
struct Buffer<'a> {
    next: usize,
    array: [Tree<'a>;BUFFER_SIZE],
}

impl<'a> Buffer<'a> {
    fn new() -> Buffer<'a> {
        let def = Tree::Empty;
        Buffer { next: 0, array: [def;BUFFER_SIZE] }
    }
}
