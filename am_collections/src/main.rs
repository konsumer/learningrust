/*
Some commonly used collections:

- A vector allows you to store a variable number of values next to each other. It's a growable array.
- A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
- A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

More info about others here: https://doc.rust-lang.org/std/collections/index.html

String is to str like Vector is to Array

See also an_strings & ao_hashmaps for more common collections

*/

fn main() {
    // empty vector
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // loaded vector
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    // 2 ways to read value
    // could panic if there is no value at that index
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // match to gaurd against panic of no value in an index
    // .get returns None for non-existent index, so it works with this:
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // dropping vector drops it's elements, so v goes out of scope here and gets deleted
}
