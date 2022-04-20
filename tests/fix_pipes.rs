use rust_sandbox::fix_pipes::fix_pipes_my::*;

#[cfg(test)]
mod sample_tests {

    #[test]
    fn small_fixed_tests() {
        for (pmap, answer) in TEST_CASES {
            super::run_test(pmap, *answer);
        }
    }

    #[rustfmt::skip]
    const TEST_CASES: &'static [(&'static[& str], bool)] = &[
        (&["╋━━┓",
            "┃..┃",
            "┛..┣"], true),
        (&["...┏",
            "┃..┃",
            "┛..┣"], false),
        (&["...┏",
            "...┃",
            "┛..┣"], false),
        (&["...┏",
            "...┃",
            "┓..┣"], true),
        (&["╋",
            "╋",
            "╋"], true),
        (&["╋....",
            "┃..┛.",
            "┃...."], false),
        (&["....",
            ".┛┛.",
            "...."], true),
        (&["....",
            "....",
            "┓..┏"], true),
        (&["..┃.",
            "....",
            "┓..┏"], false),

        (&["┻┛...",
            "..┓..",
            ".....",
            "....┻"], false),

        (&["..",
            ".┻"], false),

        (&["┏┛..┏━┓┃",
            "┃..┫┣┓┣┏",
            "┛...┗┻╋┻",
            "......┃.",
            "......┃.",
            ".╋....┗┓",
            ".......┃",
            "......┏┻",
            "......┃.",
            "......┗━",
            "........",
            "...┏━━┓.",
            "┏┓.┣┳━┻━",
            "┛┗┳╋┛..."], false),
    ];
}

fn run_test(pmap: &[&str], answer: bool) {
    let test_result = check_pipe(pmap);
    assert!(
        test_result == answer,
        "Output: {}; expected value: {}; for input:\n{}\n",
        test_result,
        answer,
        pmap.join("\n")
    );
}