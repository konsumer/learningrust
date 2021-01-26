use std::collections::HashMap;

// this isn't really pig-latin, as I know it, but it returns what they asked for
fn pig_latin(text: &String)-> String {
    let mut out = Vec::new();
    let vowels = ["a", "e", "i", "o", "u"];
    for word in text.split(" ") {
        let first_letter = &word[..1];
        if vowels.contains(&first_letter) {
            out.push(format!("{}-hay", &word));
        } else {
            out.push(format!("{}-{}ay", &word[1..], &first_letter));
        }
    }
    out.join(" ")
}

#[derive(Debug)]
struct VecStats {
    mean: f32,
    median: i32,
    mode: i32
}

impl VecStats {
    fn new(numbers: &Vec<i32>) -> VecStats {
        VecStats {
            mean: numbers.iter().sum::<i32>() as f32 / numbers.len() as f32,

            median: {
                let mut vec = numbers.clone();
                vec.sort();
                vec[ vec.len() / 2 ]
            },

            mode: {
                let mut map = HashMap::new();
                let mut current_high_count = i32::MIN;
                let mut current_high_val = 0;
                for n in numbers {
                    let count = map.entry(n).or_insert(0);
                    *count += 1;
                    if count > &mut current_high_count {
                        current_high_count = *count;
                        current_high_val = n.clone();
                    }
                }
                current_high_val
            }
        }
    }
}

fn main() {
    // Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let numbers = vec![5, 6, 7, 3, 4, 9, 6, 7, 7];
    println!("{:?}", VecStats::new(&numbers));

    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    let s = String::from("the first apple is the best apple");
    println!("{}", pig_latin(&s)); // he-tay irst-fay apple-hay is-hay he-tay est-bay apple-hay

    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

}