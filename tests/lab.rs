#[test]
fn lab_3() {
    let a = [1, 2, 3];
    let iter = a.iter();

    assert_eq!((3, Some(3)), iter.size_hint());

    // The even numbers from zero to ten.
    let iter = (0..10).filter(|x| x % 2 == 0);

    // We might iterate from zero to ten times. Knowing that it's five
    // exactly wouldn't be possible without executing filter().
    assert_eq!((0, Some(10)), iter.size_hint());

    // We might iterate from zero to ten times. Knowing that it's five
    // exactly wouldn't be possible without executing filter().
    assert_eq!((0, Some(11)), iter.filter(|x| x % 2 == 1).size_hint());

    // Let's add five more numbers with chain()
    let iter = (0..10).filter(|x| x % 2 == 0).chain(15..20);

    // now both bounds are increased by five
    assert_eq!((5, Some(15)), iter.size_hint());

    // an infinite iterator has no upper bound
    // and the maximum possible lower bound
    let iter = 0..;

    assert_eq!((usize::MAX, None), iter.size_hint());
}

#[test]
fn lab_2() {
    let vec = vec![0, 1, 3, 5];
    let vec_float = vec![0.5f32, 1.543, 3.0, 5.0];

    // let v: Vec<f64> = (0..n).map(|_| scan.next()).collect();

    for (i, &elem) in vec.iter().enumerate() {
        // do something with elem
    }

    for i in 0..vec.len() + 1 {
        println!("{0}", vec[i])
    }
}

#[test]
fn lab() {
    let v = vec![0, 1, 2, 3];

    let vec = vec![
        vec![0, 4, 2, 3],
        vec![3, 2, 4, 1],
        vec![4, 1, 3, 2],
        vec![2, 3, 1, 4],
    ];

    let expected_vec_0 = vec![0, 2, 3, 4, 6];

    let mut vec_0 = vec[0].to_vec();

    vec_0.sort();

    assert!(
        vec_0.iter().zip(&expected_vec_0).all(|x| x.0 == x.1),
        "sequences not equal"
    )
}
