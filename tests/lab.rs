
#[test]
fn lab_2(){
    let vec = vec![0, 1, 3, 5];
    let vec_float = vec![0.5f32, 1.543, 3.0, 5.0];

    // let v: Vec<f64> = (0..n).map(|_| scan.next()).collect();

    for (i, &elem) in vec.iter().enumerate() {
        // do something with elem
    }

    for i in 0..vec.len()+1 {
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
