
// Convert temperatures between Fahrenheit and Celsius.
// °C =(°F - 32) * (5/9)
fn fahrenheit_to_celsius (fdegrees: f64) -> f64 {
    (fdegrees - 32.0) * (5.0/9.0)
}

// Generate the nth Fibonacci number.
// F(n) = F(n-1) + F(n-2)
// I am  using recursion, even though they probly intended me to use a loop (as that was the subject of this chapter) because I liek recursion, and it matchjed the algo better
fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 || n == 2 {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn twelvedays() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "nineth",
        "tenth",
        "eleventh",
        "twelveth"
    ];
    
    let things = [
        "partridge in a pear tree.",
        "Two turtle doves,",
        "Three french hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

    for n in 1..13 {
        println!("On the {} day of Christmas my true love sent to me", days[n-1]);
        for d in (1..(n+1)).rev() {
            println!("{}{}", if n != 1 && d == 1 { "And a " } else { if n == 1 { "A " } else { "" } }, things[d-1]);
        }
        println!();
    }
}

fn main() {
    println!("32°F = {}°C", fahrenheit_to_celsius(32.0));
    println!("fib(30): {}", fib(30));
    println!();
    twelvedays();
}
