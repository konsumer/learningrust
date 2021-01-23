/*

Function bodies contain statements & expressions.
Semicolan determines which it is
Expressions don't return (like other languages) so this doesn't work:

let x = (let y = 6);

*/

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // expression is the final product, so this is 4, because it ends adding 1 to x:
    let a = {
        let b = 3;
        b + 1
    };

    // here, you can access a, but not b, due to block-scope
    println!("The value of a is: {}", a);
}

// silly example that always returns 5 (as i32)
fn five() -> i32 {
    5
}

fn plus_one(x: u32) -> u32 {
    x + 1 // no semicolan, so this is an expression. adding a semi makles this an error, because the function needs to return i32, not nothing (as a statement does)
}

fn main() {
    another_function(5, 6);
    println!("five() is {}", five());
    println!("plus_one(10) is {}", plus_one(10));
 }

