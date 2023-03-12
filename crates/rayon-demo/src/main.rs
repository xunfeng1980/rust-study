use rayon::prelude::*;

fn main() {
    let v: Vec<_> = (1..10).collect();
    println!("{}", sum_of_squares(v.as_slice()));
}

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
        .map(|&i| i * i)
        .sum()
}