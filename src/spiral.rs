use std::iter::{Once, Repeat};

pub fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut spiral = (0..size)
        .map(|_| vec![0 as i8; size])
        .collect::<Vec<Vec<_>>>();

    for (x, y) in spiralize_rec((-2, 0), (1, 0), size as isize + 1) {
        spiral[y as usize][x as usize] = 1;
    }

    return spiral;

    fn spiralize_rec(
        ind: (isize, isize),
        d_ind: (isize, isize),
        l: isize,
    ) -> Box<dyn Iterator<Item = (isize, isize)>> {
        return if l <= 0 {
            Box::new(std::iter::empty())
        } else if l == 1 {
            // vec![(ind.0 + d_ind.0, ind.1 + d_ind.1)]
            Box::new(std::iter::once((ind.0 + d_ind.0, ind.1 + d_ind.1)))
        } else {
            Box::new(
                (1..=l)
                    .map(move |i| (ind.0 + i * d_ind.0, ind.1 + i * d_ind.1))
                    .filter(|p| p.0 >= 0 && p.1 >= 0)
                    .chain(spiralize_rec(
                        (ind.0 + d_ind.0 * l, ind.1 + d_ind.1 * l),
                        (-d_ind.1, d_ind.0),
                        if d_ind.1 == 0 { l - 2 } else { l },
                    )),
            )
        };
    }
}

// missing lifetime spicifier
// https://depth-first.com/articles/2020/06/22/returning-rust-iterators/
// recursive iterators in Rust https://fasterthanli.me/articles/recursive-iterators-rust
