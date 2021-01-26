fn average(numbers: &Vec<i32>) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &Vec<i32>) -> i32 {
    0
}

fn mode(numbers: &Vec<i32>) -> i32 {
    0
}

#[derive(Debug)]
struct VecStats {
    mean: f32,
    median: i32,
    mode: i32
}

fn questionA(numbers: &Vec<i32>) -> VecStats {
    VecStats {
        mean: average(numbers),
        median: median(numbers),
        mode: mode(numbers)
    }
}


fn main() {
    let numbers = vec![5,6,7,3,4,9,6];
    println!("{:?}", questionA(&numbers));
}