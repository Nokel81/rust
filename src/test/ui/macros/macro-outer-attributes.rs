#![feature(custom_attribute)]

macro_rules! test { ($nm:ident,
                     #[$a:meta],
                     $i:item) => (mod $nm { #[$a] $i }); }

test!(a,
      #[cfg(qux)],
      pub fn bar() { });

test!(b,
      #[cfg(not(qux))],
      pub fn bar() { });

// test1!(#[bar])
#[qux]
fn main() {
    a::bar(); //~ ERROR cannot find function `bar` in module `a`
    b::bar();
}
