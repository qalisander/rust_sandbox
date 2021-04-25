fn main() {
    println!("{}", -11_isize.rem_euclid(10))
}
// // Functional approach
// let sum_of_squared_odd_numbers: u32 = (0..)
//     .map(|n| n * n) // All natural numbers squared
//     .take_while(|&n_squared| n_squared < upper) // Below upper limit
//     .filter(|&n_squared| is_odd(n_squared)) // That are odd
//     .fold(0, |acc, n_squared| acc + n_squared); // Sum them
// println!("functional style: {}", sum_of_squared_odd_numbers);
