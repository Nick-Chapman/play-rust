
use std::cell::RefCell;
use std::cell::Ref;
//use std::cell::Cell;

pub fn main() {
    println!("**tree experiments**");

    //let mut ts : Vec<Tree> = Vec::new();
    let collector = Collector::new();

    let t1 = Tree::Leaf(1);
    collector.push(t1);
    //ts.push(t1);
    //let t1 = &ts[0];
    let t1 = &t1;

    let t2 = Tree::Leaf(2);
    collector.push(t2);
    //ts.push(t2);
    //let t2 = &ts[1];
    let t2 = &t2;

    let t3 = Tree::Node(t1,t2);
    let t3 = &t3;

    println!("tree: {:?} --> {}",t1, sum_tree(t1));
    println!("tree: {:?} --> {}",t3, sum_tree(t3));
    println!("tree: {:?} --> {}",t2, sum_tree(t2));
}

struct Collector<'a> {
    a_tree : Tree<'a>,
    //rc : RefCell<Vec<Tree<'a>>>,
    rc : RefCell<Vec<RefCell<Tree<'a>>>>
}

impl<'a> Collector<'a> {
    fn new() -> Collector<'a> {
        let a_tree = Tree::Leaf(99);
        //let rc = RefCell::new(Vec::new());
        let rc = RefCell::new(Vec::new());
        Collector { a_tree, rc }
    }
    fn push(&self, tree: Tree<'a>) {
        self.rc.borrow_mut().push(RefCell::new(tree));
    }
    fn ref0(&'a self) -> &'a Tree<'a> {
        & self.a_tree
        //let _vec = self.rc.borrow();
        //& _vec[0]
        //self.rc.borrow().get(0).unwrap()
        //panic![""]
    }
    
    fn ref1(&'a self) -> Ref<'a,Tree<'a>> {
        //let v = self.rc.(); //.borrow();
        //let c = v[0].borrow();
        //c
        panic![""]
    }
}


#[derive(Debug,Copy,Clone)]
enum Tree<'a> {
    Leaf(u32),
    Node(&'a Tree<'a>,&'a Tree<'a>)
}

fn sum_tree(tree: &Tree) -> u32 {
    match *tree {
        Tree::Leaf(n) => n,
        Tree::Node(t1,t2) => sum_tree(t1) + sum_tree(t2)
    }
}
