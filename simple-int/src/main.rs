
fn main() {
    println!("**Simple Interpreter**");
    run()
}

fn run() {

    let e1 = Exp::Lit { int: 42 };
    let e2 = Exp::Lit { int: 13 };
    let e3 = Exp::Add { left: &e1, right: &e2 };
    let e4 = Exp::Add { left: &e3, right: &e2 };

    println!["e3 = {:?}", e3];
    println!["e4 = {:?}", e4];

    let v3 = e3.eval();
    let v4 = e4.eval();

    println!["v3 = {}", v3];
    println!["v4 = {}", v4];
}

#[derive(Debug)]
enum Exp<'a> {
    Lit { int: u32 },
    Add { left: &'a Exp<'a>, right: &'a Exp<'a> }
}

type Value = u32;

impl Exp<'_> {

    fn eval (&self) -> Value {
        use Exp::*;
        match *self {
            Lit { int } => int,
            Add { left, right } => left.eval() + right.eval()
        }
    }

}
