use crate::blox_solver::blox_solver;

#[test]
pub fn example_tests() {
    let fixed_tests = [
        vec![
            "1110000000",
            "1B11110000",
            "1111111110",
            "0111111111",
            "0000011X11",
            "0000001110",
        ],
        vec![
            "000000111111100",
            "111100111001100",
            "111111111001111",
            "1B11000000011X1",
            "111100000001111",
            "000000000000111",
        ],
        vec![
            "00011111110000",
            "00011111110000",
            "11110000011100",
            "11100000001100",
            "11100000001100",
            "1B100111111111",
            "11100111111111",
            "000001X1001111",
            "00000111001111",
        ],
        vec![
            "11111100000",
            "1B111100000",
            "11110111100",
            "11100111110",
            "10000001111",
            "11110000111",
            "11110000111",
            "00110111111",
            "01111111111",
            "0110011X100",
            "01100011100",
        ],
        vec![
            "000001111110000",
            "000001001110000",
            "000001001111100",
            "B11111000001111",
            "0000111000011X1",
            "000011100000111",
            "000000100110000",
            "000000111110000",
            "000000111110000",
            "000000011100000",
        ],
    ];
    let fixed_sols = [
        vec!["RRDRRRD", "RDDRRDR", "RDRRDDR"],
        vec!["ULDRURRRRUURRRDDDRU", "RURRRULDRUURRRDDDRU"],
        vec!["ULURRURRRRRRDRDDDDDRULLLLLLD"],
        vec!["DRURURDDRRDDDLD"],
        vec![
            "RRRDRDDRDDRULLLUULUUURRRDDLURRDRDDR",
            "RRRDDRDDRDRULLLUULUUURRDRRULDDRRDDR",
            "RRRDRDDRDDRULLLUULUUURRDRRULDDRRDDR",
        ],
    ];

    for (grid, valids) in fixed_tests.iter().zip(fixed_sols.iter()) {
        let user = blox_solver(grid);
        assert!(
            valids.iter().any(|s| s == &user),
            "You returned an invalid path"
        );
    }
}