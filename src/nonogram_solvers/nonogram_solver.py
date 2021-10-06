# https://www.codewars.com/kata/5a5072a6145c46568800004d/train/python
import itertools;
from enum import Enum, IntEnum
import unittest;


# TODO: benchmark and try call rust
# https://stackoverflow.com/questions/41770791/arrays-in-python-are-assigned-by-value-or-by-reference
class Bit(IntEnum):
    FILLED = 1
    EMPTY = 0


class FlatClues:
    def __init__(self, stack):
        self.stack = stack
        self.index = 0


class ShiftType(Enum):
    Available = 1,
    Mandatory = 2,
    Banned = 3,


class Shift:
    def __init__(self, type: ShiftType, size=0):
        self.type = type
        self.size = size


def get_next_possible_bit_shifts(processed_top_clues):
    def get_next_bit_shifts(clues):
        if len(clues.stack) <= clues.index:
            return Shift(ShiftType.Banned)

        rest_len = len(clues.stack) - clues.index - 1
        current_index = clues.index - 1
        if len(clues.stack) == current_index or clues.stack[current_index] == Bit.EMPTY:
            return Shift(ShiftType.Available, rest_len)

        return Shift(ShiftType.Mandatory, rest_len)

    return [get_next_bit_shifts(processed_top_clue)
            for processed_top_clue in processed_top_clues]


# def can_be_applied(processed_top_clues, permutation):
#     for (permutation_bit, clues) in zip(permutation, processed_top_clues):
#         if len(clues.stack) > clues.index:
#             clue_current_bit = clues.stack[clues.index]
#             if clue_current_bit == permutation_bit:
#                 continue
#             if clue_current_bit == Bit.EMPTY:
#                 return False
#             if clue_current_bit == Bit.FILLED:
#                 if len(clues.stack) > clues.index - 1:
#                     clue_previous_bit = clues.stack[clues.index - 1]
#                     if clue_previous_bit != Bit.EMPTY:
#                         return False
#         else:
#             if permutation_bit != 0:
#                 return False
#
#     return True


def apply_permutation(processed_top_clues, permutation):
    def apply(clues, permutation_bit):
        if len(clues.stack) > 1 \
                and len(clues.stack) > clues.index \
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
        clues_len = len(left_clues)  # NOTE: T
        current_clues_index = len(permutation_stack)
        next_possible_bits = get_next_possible_bit_shifts(top_clues)

        # def has_not_enough_len(shift):
        #     has_valid_type = shift.type == ShiftType.Available or shift.type == ShiftType.Mandatory
        #     return has_valid_type and shift.size > clues_len - current_clues_index
        #
        # if any(map(has_not_enough_len, next_possible_bits)):
        #     return False

        for permutation in get_permutations(next_possible_bits, left_clues[current_clues_index], clues_len):
            altered_bits = apply_permutation(top_clues, permutation)
            permutation_stack.append(tuple(permutation))
            if len(permutation_stack) == clues_len or solve_rec(top_clues, left_clues, permutation_stack):
                return True

            undo_permutation(top_clues, altered_bits)
            permutation_stack.pop()

        return False

    if solve_rec(processed_top_clues, left_clues, permutation_stack):
        return tuple(permutation_stack)
    else:
        raise BaseException("Solution not found")


def get_permutations(next_possible_shifts, clues, size):
    def get_permutations_rec(permutation, clues, init_offset: int, size):
        if len(clues) == 0:
            yield permutation
            return

        current_clue = clues[0]
        clues_sum = sum(clues)
        clues_borders = len(clues) - 1

        for offset in range(1 + size - init_offset - clues_sum - clues_borders):
            new_offset = init_offset + offset

            zeroes_range = range(init_offset, new_offset)
            has_zeroes_valid = all(
                [next_possible_shifts[index].type in (ShiftType.Available, ShiftType.Banned)
                 for index in zeroes_range])  # TODO: add range

            ones_range = range(new_offset, new_offset + current_clue)
            has_ones_valid = all(
                [next_possible_shifts[index].type in (ShiftType.Available, ShiftType.Mandatory)
                 for index in ones_range])

            has_last_zero_valid = next_possible_shifts[new_offset + current_clue].type in (
            ShiftType.Available, ShiftType.Banned) \
                if current_clue + new_offset < len(next_possible_shifts) \
                else True

            if has_zeroes_valid and has_ones_valid and has_last_zero_valid:
                new_permutation = list(permutation)  # TODO: use list comprehension
                for i in ones_range:
                    new_permutation[i] = 1

                for perm in get_permutations_rec(new_permutation, clues[1:], 1 + new_offset + current_clue, size):
                    yield perm

    return get_permutations_rec([0 for _ in range(size)], clues, 0, size)


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
        permutations = list(get_permutations([Shift(ShiftType.Available)] * 15, [1, 2, 3, 1], 15))
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
