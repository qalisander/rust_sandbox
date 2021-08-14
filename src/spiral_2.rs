fn spiralize(n: usize) -> Vec<Vec<i8>> {
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    let min = i.min(j).min(n - i - 1).min(n - j - 1);
                    (if n % 2 == 0 && i == n / 2 && j == n / 2 - 1 {
                        0
                    } else if j == min && i == min + 1 {
                        min % 2
                    } else {
                        1 - min % 2
                    }) as i8
                })
                .collect()
        })
        .collect()
}
