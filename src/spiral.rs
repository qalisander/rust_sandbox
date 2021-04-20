pub fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut spiral = (0..size)
        .map(|_| vec![0 as i8; size])
        .collect::<Vec<Vec<_>>>();

    // missing lifetime spicifier
    // https://depth-first.com/articles/2020/06/22/returning-rust-iterators/

    // TODO: recursive iterators in Rust https://fasterthanli.me/articles/recursive-iterators-rust

    // TODO: use size - 2, not x = -2
    for (x, y) in spiralize_rec(-2, 0, 1, 0, size as isize + 1) {
        spiral[y as usize][x as usize] = 1;
    }

    return spiral;

    fn spiralize_rec(
        x: isize, // TODO: use tuples
        y: isize,
        dx: isize, // tuples
        dy: isize,
        l: isize,
    ) -> impl Iterator<Item = (isize, isize)> {
        // TODO: why we can't use it without box
        // compare boxed iterators with vectors in benchmark
        return if l <= 0 {
            vec![].into_iter()
        } else if l == 1 {
            vec![(x + dx, y + dy)].into_iter()
        } else {
            (1..=l)
                .map(|i| {
                    (x + i * dx, y + i * dy) // BUG
                })
                .filter(|p| p.0 >= 0 && p.1 >= 0)
                .chain(spiralize_rec(
                    x + dx * l,
                    y + dy * l,
                    -dy,
                    dx,
                    if dy == 0 { l - 2 } else { l },
                ))
                .collect::<Vec<(isize, isize)>>()
                .into_iter()
        };
        // return Box::new(        if l > 0 {
        //     iter.chain(spiralize_rec(x, y, -dy, dx, if dy == 0 { l - 2 } else { l })).collect()
        // } else {iter.into_iter()});
    }
}
