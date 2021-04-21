pub fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut spiral = (0..size)
        .map(|_| vec![0 as i8; size])
        .collect::<Vec<Vec<_>>>();

    for (x, y) in spiralize_rec((-2, 0), (1, 0), size as isize + 1) {
        spiral[y as usize][x as usize] = 1;
    }

    return spiral;

    fn spiralize_rec(ind: (isize, isize), d_ind: (isize, isize), l: isize) -> Vec<(isize, isize)> {
        return if l <= 0 {
            vec![]
        } else if l == 1 {
            vec![(ind.0 + d_ind.0, ind.1 + d_ind.1)]
        } else {
            (1..=l)
                .map(|i| {
                    (ind.0 + i * d_ind.0, ind.1 + i * d_ind.1)
                })
                .filter(|p| p.0 >= 0 && p.1 >= 0)
                .chain(spiralize_rec(
                    (ind.0 + d_ind.0 * l, ind.1 + d_ind.1 * l),
                    (-d_ind.1, d_ind.0),
                    if d_ind.1 == 0 { l - 2 } else { l },
                ))
                .collect::<Vec<(isize, isize)>>()
        };
    }
}

// missing lifetime spicifier
// https://depth-first.com/articles/2020/06/22/returning-rust-iterators/
// TODO: recursive iterators in Rust https://fasterthanli.me/articles/recursive-iterators-rust

// TODO: why we can't use it without box
// compare boxed iterators with vectors in benchmark

// return Box::new(        if l > 0 {
//     iter.chain(spiralize_rec(x, y, -dy, dx, if dy == 0 { l - 2 } else { l })).collect()
// } else {iter.into_iter()});
