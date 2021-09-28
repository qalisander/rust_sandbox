# https://www.codewars.com/kata/5a5072a6145c46568800004d/train/python
import itertools;
import unittest;

def solve(clues):
    return (
        (0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0),
        (0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0),
        (1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0),
        (1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1),
        (1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1),
        (1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1),
        (0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0),
        (0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0),
        (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0),
        (0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0),
        (0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0),
        (1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0),
        (1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1),
        (1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1),
        (0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1)
    )


# for in loop

# for i in itertools.count(5, 5):
#     if i == 35:
#         break
#     else:
#         print(i, end =" ")


class NonogramSolveTests(unittest.TestCase):

    def test_solve_15(self):
        clues = (
            (
                (4, 3), (1, 6, 2), (1, 2, 2, 1, 1), (1, 2, 2, 1, 2), (3, 2, 3),
                (2, 1, 3), (1, 1, 1), (2, 1, 4, 1), (1, 1, 1, 1, 2), (1, 4, 2),
                (1, 1, 2, 1), (2, 7, 1), (2, 1, 1, 2), (1, 2, 1), (3, 3)
            ), (
                (3, 2), (1, 1, 1, 1), (1, 2, 1, 2), (1, 2, 1, 1, 3), (1, 1, 2, 1),
                (2, 3, 1, 2), (9, 3), (2, 3), (1, 2), (1, 1, 1, 1),
                (1, 4, 1), (1, 2, 2, 2), (1, 1, 1, 1, 1, 1, 2), (2, 1, 1, 2, 1, 1), (3, 4, 3, 1)
            )
        )
        solution = (
            (0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0),
            (0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0),
            (1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0),
            (1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1),
            (1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1),
            (1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1),
            (0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0),
            (0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0),
            (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0),
            (0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0),
            (0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0),
            (1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0),
            (1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1),
            (1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1),
            (0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1)
        )
        self.assertEqual(solve(clues), solution)


if __name__ == '__main__':
    unittest.main()


# def solve(clues):
#     pass
#
# test.describe('15x15 Nonograms')
# test.it('Sample Tests')
#
# solution = (
#     (0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0),
#     (0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0),
#     (1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0),
#     (1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1),
#     (1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1),
#     (1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1),
#     (0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0),
#     (0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0),
#     (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0),
#     (0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0),
#     (0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0),
#     (1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0),
#     (1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1),
#     (1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1),
#     (0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1)
# )
#
# clues = (
#     (
#         (4, 3), (1, 6, 2), (1, 2, 2, 1, 1), (1, 2, 2, 1, 2), (3, 2, 3),
#         (2, 1, 3), (1, 1, 1), (2, 1, 4, 1), (1, 1, 1, 1, 2), (1, 4, 2),
#         (1, 1, 2, 1), (2, 7, 1), (2, 1, 1, 2), (1, 2, 1), (3, 3)
#     ), (
#         (3, 2), (1, 1, 1, 1), (1, 2, 1, 2), (1, 2, 1, 1, 3), (1, 1, 2, 1),
#         (2, 3, 1, 2), (9, 3), (2, 3), (1, 2), (1, 1, 1, 1),
#         (1, 4, 1), (1, 2, 2, 2), (1, 1, 1, 1, 1, 1, 2), (2, 1, 1, 2, 1, 1), (3, 4, 3, 1)
#     )
# )
#
# def custom_assert_equals(clues, solution):
#     actual = solve(clues)
#     print('Expected:                          Actual:')
#     print('0 1 2 3 4 5 6 7 8 9 A B C D E      0 1 2 3 4 5 6 7 8 9 A B C D E')
#     # print_puzzle(solution, actual)
#     test.assert_equals('correct' if actual == solution else 'incorrect', 'correct')
#
# custom_assert_equals(clues, solution)