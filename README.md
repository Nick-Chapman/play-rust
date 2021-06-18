# play-rust

Early stumblings with rust.

### Resources
- ["The Book"](https://doc.rust-lang.org/book/title-page.html)
- [Writing an OS in Rust; Philipp Oppermann's blog](https://os.phil-opp.com)

### Step1

Figure out how to do [arena](simple-int/src/arena.rs) memory allocation, and make use of this for a simple _lambda calculus_ [interpreter](simple-int/src/lam.rs). No Garbage collection of course! Evaluate favorite [example](simple-int/src/main.rs): `thrice thrice decrement 0` to -27.
