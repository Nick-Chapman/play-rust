
// Define a small LC interpreter, using arena allocation

use crate::arena::Arena;

type Ident = str;

#[derive(Debug,Copy,Clone)]
pub enum Exp<'a> {
    Var (&'a Ident),
    Lam (&'a Ident, &'a Exp<'a>),
    App (&'a Exp<'a>, &'a Exp<'a>),
    Num (i32),
    Sub (&'a Exp<'a>, &'a Exp<'a>),
}

#[derive(Debug,Copy,Clone)]
pub enum Val<'a> {
    Num(i32),
    Closure(&'a Env<'a>, &'a Ident, &'a Exp<'a>)
}

#[derive(Debug,Copy,Clone)]
pub enum Env<'a> {
    Nil,
    Cons (&'a Ident, &'a Val<'a>, &'a Env<'a>)
}

pub fn eval<'a>(aq: &'a Arena<Env<'a>>, av: &'a Arena<Val<'a>>, env: &'a Env<'a>, exp: &'a Exp<'a>) -> &'a Val<'a> {
    let v = {
        match exp {
            Exp::Var(x) => env.lookup(x),
            Exp::Lam(x,body) => av.alloc(Val::Closure(env,x,body)),
            Exp::Num(n) => av.alloc(Val::Num(*n)),
            Exp::Sub(e1,e2) => sub (av,eval(aq,av,env,e1),eval(aq,av,env,e2)),
            Exp::App(e1,e2) => apply (aq,av,eval(aq,av,env,e1),eval(aq,av,env,e2))
        }
    };
    v
}

fn apply<'a>(aq: &'a Arena<Env<'a>>, av: &'a Arena<Val<'a>>, v1: &'a Val<'a>, v2: &'a Val<'a>) -> &'a Val<'a> {
    match v1 {
        Val::Num(_) => panic!["apply number"],
        Val::Closure(env,x,body) => eval(aq,av,env.bind(aq,x,v2),body)
    }
}

fn sub<'a>(av: &'a Arena<Val<'a>>, v1: &'a Val<'a>, v2: &'a Val<'a>) -> &'a Val<'a> {
    match (v1,v2) {
        (Val::Num(n1),Val::Num(n2)) => av.alloc(Val::Num(n1-n2)),
        _ => panic!["sub non-numbers"]
    }
}

impl<'a> Env<'a> {
    fn lookup(&self, x: &Ident) -> &Val {
        match self {
            Env::Nil => panic!["lookup {}", x],
            Env::Cons(y,v,env) => if x == *y {v} else {env.lookup(x)}
        }
    }
    fn bind(&'a self, a: &'a Arena<Env<'a>>, x: &'a Ident, v: &'a Val<'a>) -> &'a Env<'a> {
        a.alloc(Env::Cons(x,v,self))
    }
}
