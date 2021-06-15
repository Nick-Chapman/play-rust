
//use std::borrow::Borrow;

//use std::cell::Cell;
use std::cell::RefCell;
//use std::rc::Rc;

pub fn main() {
    println!("**Simple Interpreter**");
    run()
}

fn run() {

    let e1 = Exp::Lit { int: 42 };
    let e2 = lit(13);
    let e3 = add (&e1,&e2);
    let e4 = Exp::Add { left: &e3, right: &lit(11) };
    //let e4 = add(&e3, &lit(99)); //NO, doesn't work!!
    let e5 = Exp::Pair { left: &e3, right: &e4 };

    println!["e5 = {:?}", e5];

    let a = & Allocator::new(100);

    let v5 = e5.eval(a);
    println!["v5 = {:?}", v5];
}

fn lit<'a>(int: u32) -> Exp<'a> {
    Exp::Lit { int }
}

fn add<'a>(left: &'a Exp, right: &'a Exp) -> Exp<'a> {
    Exp::Add { left, right }
}

#[derive(Debug)]
enum Exp<'a> {
    Lit { int: u32 },
    Add { left: &'a Exp<'a>, right: &'a Exp<'a> },
    Pair { left: &'a Exp<'a>, right: &'a Exp<'a> }
}


#[derive(Debug)]
struct Allocator<'a> {
    next : usize,
    //pairs : Vec<VPair<'a>>,
    a_value : Value<'a>,
    c_value : RefCell<Value<'a>>,
    //m_value : Rc<RefCell<Value<'a>>>
}


const V_NULL : Value = Value::Int { int: 999 };

impl<'a> Allocator<'a> {

    //fn new<'b>(_n: usize) -> Allocator<'b> {
    fn new(_n: usize) -> Allocator<'a> {
        //let pair = VPair(&V_NULL,&V_NULL);
        //let pairs = vec![pair;n];
        let a_value = V_NULL;
        let c_value = RefCell::new(V_NULL);
        Allocator { next: 0, a_value, c_value }
    }

    //fn alloc_value<'b>(&'b self, _init: Value<'b>) -> & Value<'b> {
    fn alloc_value(&'a self, _init: Value<'a>) -> &'a Value<'a> {
        //let _v : & Value<'_> = &self.a_value;
        //let _v : & Value<'_> = & self.m_value; //.borrow();
        //let _v : & Value<'_> = self.m_value.borrow();

        //self.a_value = _init;
        //self.c_value.set(_init);
        *self.c_value.borrow_mut() = _init;

        //let _g = & self.c_value;;
        //_g

        //& self.a_value
        //let _g : Value<'a> = self.c_value.get();

        //& _g
        panic!["alloc_value"];

        //self.c_value.borrow()


    }

}


#[derive(Debug,Copy,Clone)]
enum Value<'a> {
    Int { int: u32 },
    Pair (&'a Value<'a>,&'a Value<'a>)
}

impl<'a> Value<'a> {

    fn new_int(a: &'a Allocator<'a>, int: u32) -> &'a Value<'a> {
        a.alloc_value(Value::Int { int })
    }

    fn new_pair(a: &'a Allocator<'a>, left: &'a Value, right: &'a Value) -> &'a Value<'a> {
        a.alloc_value(Value::Pair(left,right))
    }

    fn get_int (&self) -> u32 {
        match *self {
            Value::Int {int} => int,
            _ => panic!["get_int {:?}", self]
        }
    }

}


impl Exp<'_> {

    fn eval<'a> (&'a self, a: &'a Allocator<'a>) -> &'a Value {
        match *self {
            Exp::Lit { int } => {
                let _v : &Value<'_> = Value::new_int(a,int);
                _v
            }
            Exp::Add { left: e1, right: e2 } => {
                let i1 = e1.eval(a).get_int();
                let i2 = e2.eval(a).get_int();
                let _v = Value::new_int(a, i1 + i2);
                _v

            }
            Exp::Pair { left: e1, right: e2 } => {
                let v1 = e1.eval(a);
                let v2 = e2.eval(a);
                let _v = Value::new_pair(a,&v1,&v2);
                _v
                //panic!["{:?} {:?}",v1,v2]
            }

        }
    }

}
