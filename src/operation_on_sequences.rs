use itertools::Itertools;

// https://www.codewars.com/kata/5e4bb05b698ef0001e3344bc/solutions/rust
fn solve(arr: Vec<i128>) -> (i128, i128) {
    arr.into_iter()
        .tuples::<(_, _)>()
        .reduce(|x, y| {
            (
                (x.0 * y.0 + x.1 * y.1),
                (x.0 * y.1 - y.0 * x.1).abs(),
            )
        })
        .unwrap()
}

// fn solve(arr: Vec<i128>) -> (i128, i128) {
//     arr[2..].chunks(2)
//             .fold((arr[0],arr[1]), |(a,b),x|(a*x[0]+b*x[1], (a*x[1]-b*x[0]).abs()))
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn calc(arr: &Vec<i128>) -> i128 {
        let mut p: i128 = 1;
        let mut i = 0;
        while i < (arr.len() - 1) {
            p *= arr[i] * arr[i] + arr[i + 1] * arr[i + 1];
            i = i + 2;
        }
        return p;
    }
    fn check(arr: &Vec<i128>, res: (i128, i128)) -> bool {
        if res.0 < 0 || res.1 < 0 {
            println!("A and B should be nonnegative integers");
            return false;
        } else {
            let p = res.0 * res.0 + res.1 * res.1;
            let pp = calc(arr);
            if p != pp {
                println!("Incorrect sum of squares");
                return false;
            } else {
                return true;
            }
        }
    }
    fn dotest(arr: &Vec<i128>) -> () {
        let ans = solve(arr.to_vec());
        let bl: bool = check(arr, ans);
        assert_eq!(bl, true, "Testing array {:?}", arr);
    }

    #[test]
    fn basic_tests() {
        let mut a = vec![1, 3, 1, 2, 1, 5, 1, 9];
        dotest(&a);
        a = vec![0, 7, 0, 0];
        dotest(&a);
        a = vec![2, 1, 3, 4];
        dotest(&a);
        a = vec![3, 9, 8, 4, 6, 8, 7, 8, 4, 8, 5, 6, 6, 4, 4, 5];
        dotest(&a);
    }
}
