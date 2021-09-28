# https://www.codewars.com/kata/5a5072a6145c46568800004d/train/python
import itertools;
from enum import Enum, IntEnum
import unittest;

# TODO: benchmark and try call rust
class Bit(IntEnum):
    FILLED = 1
    EMPTY = 0

class FlatClues:
    def __init__(self, stack):
        self.stack = stack
        self.index = 0

def get_permutations(clues, size):
    def get_permutations_rec(clues, init_offset: int, size):
        if len(clues) == 0:
            yield [0 for _ in range(size)]
            return

        current_clue = clues[0]
        clues_sum = sum(clues)
        clues_borders = len(clues) - 1

        for offset in range(1 + size - init_offset - clues_sum - clues_borders):
            new_offset = init_offset + offset
            for permutations in get_permutations_rec(clues[1:], 1 + new_offset + current_clue, size):
                for i in range(new_offset, new_offset + current_clue):
                    permutations[i] = 1

                yield permutations

    return map(tuple, get_permutations_rec(clues, 0, size))

def can_be_applied(processed_top_clues, permutation):
    for (permutation_bit, clues) in zip(permutation, processed_top_clues):
        if len(clues.stack) > clues.index:
            clue_current_bit = clues.stack[clues.index]
            if clue_current_bit == permutation_bit:
                continue
            if clue_current_bit == Bit.EMPTY:
                return False
            if clue_current_bit == Bit.FILLED:
                if len(clues.stack) > clues.index - 1:
                    clue_previous_bit = clues.stack[clues.index - 1]
                    if clue_previous_bit != Bit.EMPTY:
                        return False
        else:
            if permutation_bit != 0:
                return False

    return True

def apply_permutation(processed_top_clues, permutation):
    def apply(clues, permutation_bit):
        if len(clues.stack) > 1:
            if len(clues.stack) > clues.index \
                    and (clues.stack[clues.index], permutation_bit) == (Bit.EMPTY, Bit.EMPTY) \
                    or permutation_bit == Bit.FILLED:
                clues.index += 1
                return True
            else:
                return False

    return [apply(clues, permutation_bit)
            for (clues, permutation_bit)
            in zip(processed_top_clues, permutation)]


def undo_permutation(processed_top_clues, altered_bits):
    for (clues, altered_bit) in zip(processed_top_clues, altered_bits):
        if altered_bit:
            clues.index -= 1


def solve(clues):
    top_clues = clues[0]
    left_clues = clues[1]

    def to_bits(clue_row):
        def to_ones(clue):
            return list(itertools.repeat(1, clue)) + [0]

        return [one for clue in clue_row for one in to_ones(clue)]
        # return list(itertools.chain.from_iterable(map(to_bits, clue_row)))

    processed_top_clues = [FlatClues(to_bits(clue_row)) for clue_row in top_clues]
    permutation_stack = []

    def solve_rec(top_clues, left_clues, permutation_stack):
        clues_len = len(left_clues) # NOTE: T
        current_clues_index = len(permutation_stack)
        for permutation in get_permutations(left_clues[current_clues_index], clues_len):
            if not can_be_applied(top_clues, permutation):
                continue

            altered_bits = apply_permutation(top_clues, permutation)
            permutation_stack.append(permutation)
            if len(permutation_stack) == clues_len or solve_rec(top_clues, left_clues, permutation_stack):
                return True

            undo_permutation(top_clues, altered_bits)
            permutation_stack.pop()

        return False

    if solve_rec(processed_top_clues, left_clues, permutation_stack):
        return tuple(permutation_stack)
    else:
        raise BaseException("Solution not found")


class NonogramSolveTests(unittest.TestCase):

    def test_test(self):
        foods = [
            ["Tomato and Cucumber", "Hummus, Beetroot, and Lettuce"],
            ["Cheese", "Egg"],
            ["Ham", "Bacon", "Chicken Club", "Tuna"]
        ]

        new_foods = [food for sublist in foods for food in sublist]
        print(new_foods)

        self.assertEqual(1, Bit.FILLED)

    def test_get_permutations_15(self):
        permutations = list(get_permutations([1, 2, 3, 1], 15))
        print(permutations)
        self.assertCountEqual(permutations[0], (1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0))
        self.assertCountEqual(permutations[-1], (0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1))

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
        expected_solution = (
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
        solved_solution = solve(clues)
        print(solved_solution)
        self.assertEqual(solved_solution, expected_solution)


if __name__ == '__main__':
    unittest.main()
