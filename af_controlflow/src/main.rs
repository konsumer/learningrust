fn main() {
    // IF

    let number = 3;

    // pretty much like every other language, can use if, else, and == and !=, <, >, <=, >=
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // this is sort of like ternary operator
    let condition = true;
    let number = if condition { 5 } else { 6 }; // both must be same type
    println!("The value of number is: {}", number);

    // LOOPS

    let mut counter = 0;
    
    loop {
        counter += 1;
        println!("OK, already");
        if counter == 10 {
            break
        }
    }

    // use loop in assignment!
    counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {}", result); // 10 * 2

    counter = 0;
    while counter != 10 {
        counter += 1;
        println!("OK, already");
    }


    // these are same, better to use for-loops over while (for concise readability)
    let a = [10, 20, 30, 40, 50];
    
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // range-array + reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
