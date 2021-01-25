/*
Ownership
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

References
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

Slices
- [start..end] like [5..10] would be 5-10, exclusive
- Don't have ownership, much like reference/borrowing (&)
- They are references to several bytes in a collection.
- First 0 and last len is optional.
- String Literals Are Slices, so use &str as inputs for fn to work for String and str
*/

fn scope_example1() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    // since I don't return it, s goes out of scope here, and is deleted
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn main() {
    scope_example1();
    let s = String::from("hello");
    println!("{}", calculate_length(&s));

    // only 1 mutable reference to a thing, in a scope, so this fails:
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // this is fine, as {} creates new scope
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("{}", r2);

    // swapping mutability in ref is against the rules:
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    println!("{}", no_dangle());

    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("s: {}, first word position in old string: {}", s, word);

    // these point to one chunk of mem:
    let s = String::from("hello world");
    let hello = &s[..5]; // same as [0..5]
    let world = &s[6..]; // same as [6..s.len()]
    println!("{} {}", hello, world);

    let s = String::from("hello world");
    let word = first_word_slice(&s);
    // s.clear(); // error!
    println!("the first word is: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{} {}", slice[0], slice[1]);
}

// against the rules reference, as s gets dropped so ref is invalid:
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// no ownership (borrow ref) and loop over bytes until space is found
// this is dumb because the original refed string can get out of sync with the space location
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // same as [0..s.len()]
}