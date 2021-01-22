
// const can never change
// requires type-annotations
// mut is not allowed
// name all-upper, underscore-spacing (not required, but typical)
// can be in any scope, even globl like this one
// also you can put in underscores as commas to make numbers more readable
const MAX_POINTS: u32 = 100_000;

fn main() {
    // must use `mut` to overwrite var, suing same declaration 
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // shadowing is re-using same name
    // the first is "shadowed" by the second
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    let x = x + MAX_POINTS;
    println!("The value of x is: {}", x);

    // changing type when shadowing is fine
    let spaces = "   ";
    let spaces = spaces.len();
    println!("It has {} spaces.", spaces);

    // but mut with shadowing throws error, because you are using same var, with 2 different types
    // let mut spaces = "   ";
    // spaces = spaces.len();

}