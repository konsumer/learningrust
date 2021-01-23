// SCALARS

// 4 primary scalar types: (u)integer, float, boolean, character

// int: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize - last 2 are size depending on arch, and should generally only be used if you are trying to indexing a collection (and need to know the max index size)

/*
Number literals	Example
Decimal	98_222
Hex	0xff
Octal	0o77
Binary	0b1111_0000
Byte (u8 only)	b'A'
*/

// int-overflow (wrapping at max val, like u8 256 - which is 1 because 255 is max) while used in other langs, is considered a bug in rust 

// float: f32, f64

// boolean: true, false

// character: most primitive alphabetic type, 4 bytes in size


// COMPOUND

// tup is a group of multiple values in multiple types.
// array is a group with only 1 type

fn main() {
    // bool type, no annotation needed
    let x = true;
    println!("X: {}", x);
    
    // use 32bit as goto for all platforms because it's generally fastest, unless you need a specific size
    let x = 0; // defaults to i32
    println!("X: {}", x);
    let x: u32 = 0;
    println!("X: {}", x);
    
    // 64bit as goto for floats, because it's same speed on modern hardware, but capable of more precision
    let x = 0.0; // defaults to f64 because of .
    println!("X: {}", x);

    // math between types works fine, but stay in basic scalar fam
    let y: f32 = 2.3;
    let x = 5.0 - y; // this is f64 - f32, because f64 is default. do not do 5 (int) - y (float)
    println!("X: {}", x);

    // char-type
    let heart_eyed_cat = '😻';
    // tup-type
    let t = (x, heart_eyed_cat);
    println!("tuple: {} {}", t.0, t.1); // access elements with . not []

    // destructure tuple
    let (z, h) = t;
    println!("tuple: {} {}", z, h);

    // array, because all are same type
    let a = [1,2,3,4,5];
    println!("array: {} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);
    let a: [u32; 5] = [1, 2, 3, 4, 5]; // force type in annotation
    println!("array: {} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);

    // initialize array with all 3's, like [3, 3, 3, 3, 3]
    let a = [3; 5];
    println!("array: {} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);

}
