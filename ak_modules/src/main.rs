use std::collections::HashMap;

// merge uses:
// use std::{cmp::Ordering, io}; // puts Ordering and io in scope

// This line brings std::io and std::io::Write into scope.
// use std::io::{self, Write};

// glob -  brings all public items defined in std::collections into the current scope:
// use std::collections::*; 

// if you have 2 things with same name, use parent scope or "as" keyword
// use std::{ fmt, io }
// later: fmt::Result. io::Result
// OR
// use std::fmt::Result;
// use std::io::Result as IoResult;

// this re-exposes hosting
// the mod must also be public
pub use ak_modules::front_of_house::hosting;

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);

  // you can grab modules from current crate directly (this is in lib.rs)
  ak_modules::eat_at_restaurant();

  hosting::add_to_waitlist();
}