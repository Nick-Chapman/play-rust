
// Now we have working arean. Can we define a small LC interpreter?

use crate::arena::Arena;

pub fn main() {
    println!["**lam**"];

    let life = "life";

    let av = Arena::new(Val::Num(0));

    let aq = Arena::new(Env::Nil);
    let q0 =
        aq.alloc(Env::Nil).bind(&aq,life,av.alloc(Val::Num(42)));

    let f = "f";
    let x = "x";
    let ae = Arena::new(Exp::Empty);

    use Exp::*;

    let dec = ae.alloc(Lam(x,
                           ae.alloc(Sub(ae.alloc(Exp::Var(x)),
                                        ae.alloc(Num(1))))));

    let ff = ae.alloc(Exp::Var(f));
    let xx = ae.alloc(Exp::Var(x));
    let thrice = ae.alloc(Lam(f,ae.alloc(Lam(x, ae.alloc(App(ff,ae.alloc(App(ff,ae.alloc(App(ff,xx))))))))));

    let prog = ae.alloc(App(ae.alloc(App(ae.alloc(App(thrice,thrice)),dec)),
                            ae.alloc(Num(0))));

    let exp = prog;
    let env = q0;
    println!["exp = {:?}", exp];
    println!["env = {:?}", env];

    let res = eval(&aq,&av,env,exp);
    println!["res = {:?}", res];
}

fn eval<'a>(aq: &'a Arena<Env<'a>>, av: &'a Arena<Val<'a>>, env: &'a Env<'a>, exp: &'a Exp<'a>) -> &'a Val<'a> {
    //println!["eval {:?}",exp];
    let v = {
        match exp {
            Exp::Empty => panic!["eval(Empty)"],
            Exp::Var(x) => env.lookup(x),
            Exp::Lam(x,body) => av.alloc(Val::Closure(env,x,body)),
            Exp::Num(n) => av.alloc(Val::Num(*n)),
            Exp::Sub(e1,e2) => sub (av,eval(aq,av,env,e1),eval(aq,av,env,e2)),
            Exp::App(e1,e2) => apply (aq,av,eval(aq,av,env,e1),eval(aq,av,env,e2))
        }
    };
    //println!["eval {:?} --> {:?}", exp, v];
    v
}

fn apply<'a>(aq: &'a Arena<Env<'a>>, av: &'a Arena<Val<'a>>, v1: &'a Val<'a>, v2: &'a Val<'a>) -> &'a Val<'a> {
    //println!["apply {:?} {:?}",v1,v2];
    match v1 {
        Val::Num(_) => panic!["apply Number"],
        Val::Closure(env,x,body) => eval(aq,av,env.bind(aq,x,v2),body)
    }
}


fn sub<'a>(av: &'a Arena<Val<'a>>, v1: &'a Val<'a>, v2: &'a Val<'a>) -> &'a Val<'a> {
    //println!["sub {:?} {:?}",v1,v2];
    match (v1,v2) {
        (Val::Num(n1),Val::Num(n2)) => av.alloc(Val::Num(n1-n2)),
        _ => panic!["sub, wrong types... {:?} {:?}",v1,v2]
    }
}

type Ident = str;

#[derive(Debug,Copy,Clone)]
enum Exp<'a> {
    Empty,
    Var (&'a Ident),
    Lam (&'a Ident, &'a Exp<'a>),
    App (&'a Exp<'a>, &'a Exp<'a>),
    Num (i32),
    Sub (&'a Exp<'a>, &'a Exp<'a>),
}

#[derive(Debug,Copy,Clone)]
enum Val<'a> {
    Num(i32),
    Closure(&'a Env<'a>, &'a Ident, &'a Exp<'a>)
}

#[derive(Debug,Copy,Clone)]
enum Env<'a> {
    Nil,
    Cons (&'a Ident, &'a Val<'a>, &'a Env<'a>)
}

impl<'a> Env<'a> {
    fn lookup(&self, x: &Ident) -> &Val {
        //println!["Env.lookup({:?},{:?})", self, x];
        match self {
            Env::Nil => panic!["lookup {}", x],
            Env::Cons(y,v,env) => if x == *y {v} else {env.lookup(x)}
        }
    }
    fn bind(&'a self, a: &'a Arena<Env<'a>>, x: &'a Ident, v: &'a Val<'a>) -> &'a Env<'a> {
        //println!["Env.bind({:?},{:?},{:?})", self, x, v];
        a.alloc(Env::Cons(x,v,self))
    }
}
