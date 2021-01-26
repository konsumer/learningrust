/*

- Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.
- string slices are references to some UTF-8 encoded string data stored elsewhere.
- String literals, are stored in the program’s binary and are therefore string slices.
- Rust’s standard library also includes a number of other string types, such as OsString, OsStr, CString, and CStr.
*/

fn main() {
    // these all do the same basic thing (turn str into a new String)
    let mut s = String::new();
    s.push_str("initial contents");
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    
    let s = "initial contents".to_string();
    println!("{}", s);

    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    // format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // NO: can't access individual chars by index:
    // let s = s1[0];
    // println!("{}", s);

    // these are OK, though
    let s = s1.chars().nth(0).expect("First char should exist");
    println!("{}", s);
    let s = s1.bytes().nth(0).expect("First char should exist");
    println!("{}", s);

    // and slicing is fine
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // loop over chars:
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // loop over bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
