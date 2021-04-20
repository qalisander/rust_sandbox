use itertools;

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![1, 2, 3, 4, 5];

    // v1.iter().enumerate()

    for v in &v1 {
        println!("{}", v);
    }

    for v in v1.into_iter() {
        println!("{}", v);
    }
    // print!("{}", v1 == v2);
}
