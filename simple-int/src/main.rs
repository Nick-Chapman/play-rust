
mod arena;
mod lam;

use crate::arena::Arena;
use crate::lam::{Exp,Val,Env};

fn main() {

    let ae = Arena::new(Exp::Num(0)); // Expression arena
    let av = Arena::new(Val::Num(0)); // Value arena
    let aq = Arena::new(Env::Nil); // Environment arena

    // Favorite example: "thrice thrice decrement 0" --> -27
    let exp = thrice_thrice_example(&ae);
    println!["exp = {:?}", exp];

    let env = aq.alloc(Env::Nil);

    let res = lam::eval(&aq,&av,env,exp);
    println!["res = {:?}", res];
}

pub fn thrice_thrice_example<'a> (ae: &'a Arena<Exp<'a>>) -> &'a Exp<'a> {
    let f = "f";
    let x = "x";
    use Exp::*;
    let decrement = ae.alloc(Lam(x, ae.alloc(Sub(ae.alloc(Exp::Var(x)),
                                                 ae.alloc(Num(1))))));
    let ff = ae.alloc(Exp::Var(f));
    let xx = ae.alloc(Exp::Var(x));
    let thrice = ae.alloc(Lam(f,ae.alloc(Lam(x, ae.alloc(App(ff,ae.alloc(App(ff,ae.alloc(App(ff,xx))))))))));
    let exp = ae.alloc(App(ae.alloc(App(ae.alloc(App(thrice,thrice)),decrement)),
                           ae.alloc(Num(0))));
    exp
}
