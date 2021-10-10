# https://www.codewars.com/kata/5a5072a6145c46568800004d/train/python
from itertools import takewhile, repeat
import time
from enum import IntEnum, Enum

# TODO: remove recursion
# TODO: try call rust
# https://stackoverflow.com/questions/41770791/arrays-in-python-are-assigned-by-value-or-by-reference

FILLED = 1
EMPTY = 0

class FlatClues:  # leave as two arrays
    def __init__(self, stack: [int]):
        self.stack = stack
        self.index = 0


class ShiftType(Enum):
    Available = 0,
    Mandatory = 1,
    Banned = 2,


def get_next_possible_bit_shifts(processed_top_clues):
    def get_next_bit_shifts(clues: FlatClues):
        next_index = clues.index
        if len(clues.stack) <= next_index or clues.stack[clues.index] == EMPTY:
            return ShiftType.Banned

        current_index = clues.index - 1
        if 0 > current_index or clues.stack[current_index] == EMPTY:
            return ShiftType.Available

        return ShiftType.Mandatory

    return [get_next_bit_shifts(processed_top_clue)
            for processed_top_clue in processed_top_clues]


def apply_permutation(processed_top_clues, permutation):
    def apply(clues, permutation_bit):
        if len(clues.stack) > 1 \
                and len(clues.stack) > clues.index \
                and (clues.stack[clues.index], permutation_bit) == (EMPTY, EMPTY) \
                or permutation_bit == FILLED:
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
    MAGIC_ROW_COUNT = 2

    def heuristics_of_first(row_count, clues):
        empty_clues_count = len([takewhile(lambda row: len(row) == 0, clues)])
        return heuristics_of(slice(0, row_count + empty_clues_count), clues)

    def heuristics_of_last(row_count, clues):
        empty_clues_count = len([takewhile(lambda row: len(row) == 0, reversed(clues))])
        return heuristics_of(slice(-1 - row_count - empty_clues_count, -1), clues)

    def heuristics_of(slice, clues):
        def heuristics_of(i, row):
            return len(row) + sum(row) * (1.6 if len(row) == 1 else 1) * (MAGIC_ROW_COUNT - i)

        return sum([heuristics_of(i, row) for i, row in enumerate(clues[slice])])

    def flip(clues_a, clues_b):
        return [tuple(reversed(row)) for row in clues_a], tuple(reversed(clues_b))

    has_flip_vertically = heuristics_of_last(MAGIC_ROW_COUNT, left_clues) > heuristics_of_first(MAGIC_ROW_COUNT,
                                                                                                left_clues)
    has_flip_horizontally = heuristics_of_last(MAGIC_ROW_COUNT, top_clues) > heuristics_of_first(MAGIC_ROW_COUNT,
                                                                                                 top_clues)
    if has_flip_vertically:
        top_clues, left_clues = flip(top_clues, left_clues)
    if has_flip_horizontally:
        left_clues, top_clues = flip(left_clues, top_clues)

    has_flip_diagonally = heuristics_of_first(MAGIC_ROW_COUNT, top_clues) > heuristics_of_first(MAGIC_ROW_COUNT,
                                                                                                left_clues)
    if has_flip_diagonally:
        top_clues, left_clues = left_clues, top_clues

    def to_bits(clue_row):
        def to_ones(clue):
            return list(repeat(1, clue)) + [0]

        return [one for clue in clue_row for one in to_ones(clue)]
        # return list(itertools.chain.from_iterable(map(to_bits, clue_row)))

    processed_top_clues = [FlatClues(to_bits(clue_row)) for clue_row in top_clues]
    permutation_stack = []

    def solve_rec(top_clues, left_clues, permutation_stack):
        clues_len = len(left_clues)  # NOTE: T
        current_clues_index = len(permutation_stack)
        rest_clue_lens = [len(clues.stack) - clues.index - 1 for clues in top_clues]

        has_not_enough_len = any(
            map(lambda rest_clue_len: rest_clue_len > clues_len - current_clues_index, rest_clue_lens))
        if has_not_enough_len:
            return False

        next_possible_bits = get_next_possible_bit_shifts(top_clues)

        for permutation in get_permutations(
                next_possible_bits, left_clues[current_clues_index], clues_len, rest_clue_lens):

            altered_bits = apply_permutation(top_clues, permutation)
            permutation_stack.append(tuple(permutation))
            if len(permutation_stack) == clues_len or solve_rec(top_clues, left_clues, permutation_stack):
                return True

            undo_permutation(top_clues, altered_bits)
            permutation_stack.pop()

        return False

    if solve_rec(processed_top_clues, left_clues, permutation_stack):
        if has_flip_diagonally:
            I, J = len(permutation_stack), len(permutation_stack[0])
            permutation_stack = [tuple(permutation_stack[i][j] for i in range(I)) for j in range(J)]
        if has_flip_vertically:
            permutation_stack = reversed(permutation_stack)
        if has_flip_horizontally:
            permutation_stack = map(tuple, map(reversed, permutation_stack))
        return tuple(permutation_stack)
    else:
        raise BaseException("Solution not found")


# quick replacement in numpy https://www.kite.com/python/answers/how-to-replace-elements-of-a-numpy-array-based-on-a-condition-in-python

def get_permutations(next_possible_shifts, clues, size, rest_clue_lens):
    def get_permutations_rec(permutation, clues, init_offset: int):
        if len(clues) == 0:
            yield permutation
            return

        current_clue = clues[0]
        clues_sum = sum(clues)
        clues_borders = len(clues) - 1

        def offset_key(offset):
            return sum(rest_clue_lens[offset:offset + current_clue])  # function to get clues slice

        # TODO: run backwards when mass of clues is higher in the mid
        for new_offset in sorted(range(init_offset, 1 + size - clues_sum - clues_borders), key=offset_key,
                                 reverse=True):  # https://www.programiz.com/python-programming/methods/list/sort
            last_zero_index = new_offset + current_clue

            def set_clue_indices(bit):
                for i in range(last_zero_index - current_clue, last_zero_index):
                    permutation[i] = bit

            def has_last_zeroes_valid():
                if len(clues) == 1:
                    return has_zeroes_valid(last_zero_index, size)
                elif last_zero_index < len(next_possible_shifts):
                    return next_possible_shifts[last_zero_index] != ShiftType.Mandatory
                else:
                    return True

            def has_zeroes_valid(init_offset, new_offset):
                zeroes_range = slice(init_offset, new_offset)
                return all([shift != ShiftType.Mandatory for shift in next_possible_shifts[zeroes_range]])

            def has_ones_valid():
                ones_range = slice(new_offset, last_zero_index)
                return all([shift != ShiftType.Banned for shift in next_possible_shifts[ones_range]])

            if has_last_zeroes_valid() and has_zeroes_valid(init_offset, new_offset) and has_ones_valid():
                set_clue_indices(FILLED)

                for perm in get_permutations_rec(permutation, clues[1:], 1 + new_offset + current_clue):
                    yield perm

                set_clue_indices(EMPTY)

    return get_permutations_rec([EMPTY] * size, clues, 0)


def test_get_permutations_15():
    clues_len = 15
    permutations = list(
        map(tuple, get_permutations([(ShiftType.Available, 0)] * clues_len, [1, 2, 3, 1], clues_len, [1] * clues_len)))
    print(permutations)
    assert permutations[0] == (1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0)
    assert permutations[-1] == (0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1)


def test_solve_15():  # 2.285
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
    pretty_print(solved_solution)
    assert solved_solution == expected_solution


def test_solve_15_2():  # 3.597 ms
    clues = (((4, 3), (1, 6, 2), (1, 2, 2, 1, 1), (1, 2, 2, 1, 2), (3, 2, 3),
              (2, 1, 3), (1, 1, 1), (2, 1, 4, 1), (1, 1, 1, 1, 2), (1, 4, 2),
              (1, 1, 2, 1), (2, 7, 1), (2, 1, 1, 2), (1, 2, 1), (3, 3)), (
                 (3, 2), (1, 1, 1, 1), (1, 2, 1, 2), (1, 2, 1, 1, 3), (1, 1, 2, 1),
                 (2, 3, 1, 2), (9, 3), (2, 3), (1, 2), (1, 1, 1, 1),
                 (1, 4, 1), (1, 2, 2, 2), (1, 1, 1, 1, 1, 1, 2), (2, 1, 1, 2, 1, 1), (3, 4, 3, 1)))
    solve_for(0, clues)

    clues = (((1,), (1, 4), (1, 3), (1, 1, 1), (2, 1, 2, 4), (1, 2, 1, 1, 3), (1, 1, 1, 1, 1, 2), (1, 1, 1, 1, 1, 3),
              (2, 1, 2, 2, 3), (2, 2, 2, 2), (2, 2, 2, 3), (2, 2, 2, 2), (2, 2, 3), (1, 2, 2), (1,)), (
                 (4,), (2, 3), (8,), (2, 3), (1, 5), (1, 1, 5), (1, 1, 3, 4), (2, 1, 3), (3, 1, 1, 4), (3, 1, 2, 3),
                 (1, 1, 3, 2), (1, 2, 3), (2, 2, 2), (2, 2), (1, 1)))
    solve_for(1, clues)

    clues = (((3, 2), (2, 1, 1), (3, 1, 1, 2), (1, 4, 3), (1, 2, 3), (3, 2, 1, 1), (1, 1, 1, 2), (1, 12), (1, 1, 1, 2),
              (3, 2, 1, 1), (1, 2, 3), (1, 4, 3), (3, 1, 1, 2), (2, 1, 1), (3, 2)), (
                 (3,), (1, 1), (1, 1), (11,), (1, 1, 1), (2, 3, 2), (1, 1, 1, 1, 1, 1, 1), (1, 3, 1, 3, 1), (5, 1, 5),
                 (1, 1, 1), (1, 1, 1), (1, 1, 1), (1, 3, 1, 3, 1), (1, 3, 3, 3, 1), (3, 7, 3)))
    solve_for(2, clues)

    clues = (((3,), (2, 2), (1, 1, 1), (1, 1, 1), (3, 1, 2), (2, 1, 1, 1), (3, 1, 1, 1), (2, 1, 2), (1, 1, 2, 1),
              (1, 5, 2), (1, 1, 1), (1, 1, 1, 2, 1), (5, 2, 1), (1, 1, 1, 1), (2,)), (
                 (3, 1, 1), (2,), (1,), (2,), (1,), (6,), (1, 1, 1), (1, 1, 1), (7, 1, 1), (2, 2, 2), (1, 3, 1, 1, 1),
                 (1, 2, 1, 1, 2, 1), (1, 6, 2, 1), (1, 1, 1, 1), (2, 2)))
    solve_for(3, clues)

    clues = (((8,), (1, 1, 1), (2, 1, 1), (1, 1, 4), (2, 1), (3, 1, 1, 1), (1, 1, 8), (1, 6), (2, 1), (2, 1, 2),
              (2, 1, 1, 1), (2, 1, 1, 1, 2, 1), (1, 6, 1, 2), (1, 1, 1), (3, 4)), (
                 (4, 4), (1, 4, 1), (1, 2, 1), (2, 3), (1, 1, 1), (1, 1, 2), (5, 1, 1, 1), (1, 1, 1, 2, 1, 1),
                 (2, 1, 8),
                 (1, 1), (1, 2, 1, 2), (1, 1, 1, 2, 1), (1, 1, 1, 1, 1), (1, 1, 1, 1, 1), (3, 3, 2)))
    solve_for(4, clues)

    clues = (((6, 2, 1), (2, 2, 1, 2), (1, 2), (4, 2), (1, 2, 1), (1, 1, 1), (1, 1, 1), (1, 2, 1), (4, 2, 1),
              (1, 1, 2, 1), (1, 4, 2), (2, 2, 1, 2), (3, 3), (1, 2, 1), (6, 4)), (
                 (6,), (1, 4), (3, 1, 2), (2, 2, 2, 1, 1), (1, 4, 2, 3), (1, 1, 2, 1), (1, 4, 1), (1, 2, 1), (2, 4),
                 (2, 2),
                 (2, 3), (2, 9, 2), (1, 1), (1, 1), (2, 1)))
    solve_for(5, clues)

    clues = (((1, 8), (3, 3, 1, 2), (1, 3, 1, 1), (1, 1, 1), (1, 1, 1), (1, 3, 1, 1), (3, 3, 1, 2), (1, 8), (1, 2),
              (1, 2, 5), (3, 1, 1, 2), (2, 1, 1, 1, 1), (1, 2, 1, 1, 1), (1, 1, 1, 2), (4, 6)), (
                 (6, 2), (2, 1, 1, 2), (2, 4, 1, 1), (1, 1, 1, 2, 1), (2, 2, 1, 1), (1, 1, 1), (2, 2, 2), (1, 3),
                 (1, 2, 5),
                 (8, 1, 1), (1, 1, 6), (1, 1, 1, 1), (1, 1, 1, 1), (2, 2, 2, 2), (6, 4)))
    solve_for(6, clues)

    clues = (((1, 1), (1, 2, 2), (5, 1, 5), (6, 1), (3, 6), (4, 2), (1, 1, 2, 1), (4, 2, 2, 1), (1, 2, 2, 1),
              (2, 1, 1, 1), (2, 2, 2, 1), (3, 1), (1, 4, 1), (2, 2, 3), (6, 1)), (
                 (1,), (2, 4), (2, 1, 1), (7,), (6, 5), (3, 4, 2), (1, 2, 2, 1), (5, 1, 2, 3), (1, 1, 1, 1, 3),
                 (1, 1, 2, 1, 1), (1, 1, 2, 1), (1, 1, 3, 1), (1, 2, 1, 1), (1, 5, 1), (1, 3, 1)))
    solve_for(7, clues)

    clues = (((1,), (4, 2), (1, 5, 1), (1, 9), (4, 4, 1), (1, 1, 2, 3), (4, 3), (3, 3), (5, 1, 2), (2, 4, 2), (7, 1, 1),
              (3, 5, 1), (2, 2), (1, 1, 1, 1), (1,)), (
                 (1,), (4, 4), (1, 1, 5), (8,), (1, 3, 1), (4, 6), (1, 1, 7), (3, 3), (2, 2, 1), (5, 2), (7, 1), (3, 6),
                 (7, 1), (2, 1), (2,)))
    solve_for(8, clues)

    clues = (((6, 4), (1, 2, 2), (4, 2), (1, 1), (2, 2, 1), (4, 1, 1, 1), (2, 1, 1, 1), (1, 1, 1, 2), (1, 1, 1),
              (1, 1, 1, 1), (2, 3, 1, 2), (5, 1, 1, 1), (2, 1, 1), (1, 1, 1), (3,)), (
                 (5,), (1, 2, 2), (1, 1, 1, 1), (1, 1, 1, 1, 1, 1), (1, 1, 2, 1), (1, 3, 2), (1, 1, 2), (2, 3, 1),
                 (1, 1),
                 (1, 7), (1, 3), (1, 1), (2, 4, 4), (2, 4), (5,)))
    solve_for(9, clues)

    clues = (((8,), (1, 1, 1), (2, 1, 1), (1, 1, 4), (2, 1), (3, 1, 1, 1), (1, 1, 8), (1, 6), (2, 1), (2, 1, 2),
              (2, 1, 1, 1), (2, 1, 1, 1, 2, 1), (1, 6, 1, 2), (1, 1, 1), (3, 4)), (
                 (4, 4), (1, 4, 1), (1, 2, 1), (2, 3), (1, 1, 1), (1, 1, 2), (5, 1, 1, 1), (1, 1, 1, 2, 1, 1),
                 (2, 1, 8),
                 (1, 1), (1, 2, 1, 2), (1, 1, 1, 2, 1), (1, 1, 1, 1, 1), (1, 1, 1, 1, 1), (3, 3, 2)))
    solve_for(10, clues)

    clues = (((2, 2), (2, 2, 2), (1, 1, 2, 1, 4), (2, 1, 1, 1), (2, 6, 3), (4, 3, 2), (1, 1, 1, 1), (4, 3),
              (2, 1, 1, 1, 1, 2), (4, 2, 2), (1, 5), (1, 2, 1), (2, 1, 2), (1, 2, 2), (1, 1)), (
                 (2, 1), (2, 2, 2), (1, 1, 2, 1, 4), (2, 1, 1, 1), (2, 6, 3), (4, 3, 2), (1, 1, 1, 1), (4, 3),
                 (2, 1, 1, 1, 1, 2), (1, 4, 2, 2), (1, 5), (1, 2, 1), (2, 1, 2), (1, 2, 2), (1, 1)))
    solve_for(11, clues)

    clues = (((), (4,), (1, 1), (1, 3, 1), (1, 5, 2), (3, 1, 1), (5, 2, 1), (6, 2, 1, 1), (5, 2, 1), (3, 1, 1),
              (1, 5, 2), (1, 3, 1), (1, 1), (4,), ()), (
                 (), (3,), (5,), (5,), (9,), (1, 3, 1), (1, 1, 1, 1, 1), (1, 2, 2, 1), (1, 2, 2, 1), (1, 5, 1, 1),
                 (3, 7),
                 (1, 1), (5,), (1, 1), (1, 1, 1)))
    solve_for(12, clues)

    clues = (((4, 4), (3, 1, 1), (2, 3, 1), (1, 6, 2), (2, 4, 1), (3, 4, 1), (3, 3, 2), (6, 4), (4, 3), (2, 3, 1),
              (2, 3, 1, 1), (1, 2, 1, 1, 1), (2, 1, 1, 1, 1), (3, 2, 1, 1, 1), (4, 1, 1)), (
                 (4, 3), (3, 1, 2), (2, 3, 1), (1, 6, 2), (2, 4, 1), (3, 4, 1), (3, 3, 2), (6, 4), (4, 3), (2, 3, 1),
                 (2, 3, 1, 1), (1, 2, 1, 1, 1), (1, 1, 1, 1, 1), (4, 2, 1, 1, 1), (4, 1, 1)))
    solve_for(13, clues)

    clues = (((1, 2), (8,), (1, 2, 2), (3, 2, 1, 1), (2, 2, 6), (3,), (2, 1), (1, 1, 1), (1, 1, 2), (2, 2), (1, 1, 1),
              (3, 5), (1, 1, 3), (2, 2), (3,)), (
                 (1,), (5,), (1, 2, 1), (4, 2, 1), (3, 2, 2, 2), (1, 1, 1, 1, 3, 1), (1, 3, 1, 2, 1), (1, 1, 1, 1),
                 (1, 2),
                 (1, 1, 1, 1, 1), (2, 3, 2), (1, 3), (1, 1), (1, 1), (2, 2)))
    solve_for(18, clues)

    # 618.23ms
    clues = (((2, 1), (3, 2), (3, 1, 1), (8,), (1, 5), (3, 3), (8,), (5,), (6, 1), (2, 1, 6), (1, 2, 4), (2, 2, 1),
              (1, 1, 3), (4, 2), ()), (
                 (2, 2), (2, 1, 1), (3, 1, 1), (1, 2, 2, 1), (3, 2), (5, 3), (2, 6, 3), (9, 1), (10,), (1, 2, 4),
                 (1, 1, 5),
                 (1, 1, 1), (1, 1, 1), (1, 1), ()))
    solve_for(14, clues)

    # 440.07ms
    clues = ((
                 (3,), (2, 1, 2), (1, 1, 5, 1), (4, 9), (2, 1, 1), (1, 4), (2, 1), (1,), (1, 6), (2, 3, 1, 1),
                 (5, 1, 1, 1),
                 (4, 2, 1, 1), (1, 1, 2, 1, 1), (2, 1, 1, 1), (3, 1)), (
                 (3, 3), (2, 1, 1, 2), (1, 1, 1, 1), (4, 4), (2,), (2, 3), (2, 3), (2, 5, 2), (5, 2), (2, 2), (2, 1, 1),
                 (4, 2, 1), (1, 1, 1, 1, 1), (3, 1, 2, 1, 1), (1, 2, 1, 1, 1, 1)))
    solve_for(15, clues)

    # 1351.59ms
    clues = (((2,), (3,), (1, 1), (1, 2, 1), (1, 1), (1, 1, 3), (2, 2), (8, 2, 1), (1, 2, 2, 1, 1, 2), (2, 1, 2, 1, 1),
              (3, 2, 1, 1), (3, 2), (1, 1), (2, 2), (4,)), (
                 (2,), (1, 2), (2, 1), (2, 1), (1, 1, 1), (2, 1), (1, 2, 4), (1, 1, 2, 2), (1, 1, 1, 1), (5, 1, 1, 1),
                 (2, 1, 2, 1, 1, 2), (2, 1, 2, 4), (3, 1), (2, 1, 1), (3,)))
    solve_for(16, clues)

    clues = (((1, 2), (2, 1, 1, 1), (2, 1, 2, 1), (1, 1, 1, 2), (1, 2, 2), (2, 2, 1, 4), (1, 5), (5,), (1, 4),
              (1, 2, 1, 4), (1, 1, 2), (2, 2, 1), (2, 3, 1, 1), (1, 1, 1, 1, 2), (1, 1, 2)), (
                 (2, 2), (1, 2, 1, 1), (1, 1, 1, 2, 1), (2, 1, 1, 2, 1), (3, 2, 1, 2), (2, 3, 1, 2), (1, 1, 1, 1, 3, 1),
                 (1, 2, 2, 1, 1, 1), (1, 2, 5, 1, 2), (2, 1, 1, 2), (1, 1), (2, 2), (1, 1), (1, 1), (1, 1)))
    solve_for(17, clues)

    clues = (((2, 1, 1, 1, 1), (4,), (2, 1), (1, 9), (1, 2, 1), (1, 1, 1, 3, 1, 1), (1, 2, 2, 1), (1, 3, 4, 1, 1),
              (1, 2, 2, 1), (1, 1, 7), (1, 1), (1, 1, 4, 1), (1, 2, 1, 1), (1, 1, 7), (1, 1, 1, 1, 1)), (
                 (3, 3), (1, 1, 1), (1, 2, 1), (1, 6, 1), (4,), (3,), (4, 2), (1, 1, 3, 2), (4, 5, 3),
                 (1, 1, 1, 1, 1, 1),
                 (1, 7, 4), (1, 1, 1, 1), (1, 1, 1, 1, 6), (1, 1), (1, 1, 1, 1, 1, 1, 1)))
    solve_for(18, clues)


def solve_for(id, clues):
    tic = time.perf_counter()
    solved_solution = solve(clues)
    toc = time.perf_counter()
    pretty_print(solved_solution)
    print(f"\n{id} completed in {(toc - tic) * 1000:0.2f} ms")


def pretty_print(solution):
    str_tuple = [''.join(['. ' if bit == 0 else '[]' for bit in tpl]) for tpl in solution]
    print('\n', *str_tuple, sep='\n')


# def something(duration=0.000001):
#     """
#     Function that needs some serious benchmarking.
#     """
#     time.sleep(duration)
#     # You may return anything you want, like the result of a computation
#     return 123
#
#
# def test_my_stuff(benchmark):
#     # benchmark something
#     result = benchmark(something)
#
#     # Extra code, to verify that the run completed correctly.
#     # Sometimes you may want to check the result, fast functions
#     # are no good if they return incorrect results :-)
#     assert result == 123


def test_solve_multisize():
    puzzles = get_puzzles()

    for puzzle in puzzles:
        args, solution, it = puzzle
        assert solve(*args), solution

def get_puzzles():

    v_clues = ((1, 1), (4,), (1, 1, 1), (3,), (1,))
    h_clues = ((1,), (2,), (3,), (2, 1), (4,))
    args = ((v_clues, h_clues), 5, 5)

    ans = ((0, 0, 1, 0, 0),
           (1, 1, 0, 0, 0),
           (0, 1, 1, 1, 0),
           (1, 1, 0, 1, 0),
           (0, 1, 1, 1, 1))

    t1 = (args, ans, '5 x 5 puzzle')



    v_clues = ((3,), (4,), (2, 2, 2), (2, 4, 2), (6,), (3,))
    h_clues = ((4,), (6,), (2, 2), (2, 2), (2,), (2,), (2,), (2,), (), (2,), (2,))
    args = ((v_clues, h_clues), 6, 11)

    ans = ((0, 1, 1, 1, 1, 0),
           (1, 1, 1, 1, 1, 1),
           (1, 1, 0, 0, 1, 1),
           (1, 1, 0, 0, 1, 1),
           (0, 0, 0, 1, 1, 0),
           (0, 0, 0, 1, 1, 0),
           (0, 0, 1, 1, 0, 0),
           (0, 0, 1, 1, 0, 0),
           (0, 0, 0, 0, 0, 0),
           (0, 0, 1, 1, 0, 0),
           (0, 0, 1, 1, 0, 0))

    t2 = (args, ans, '6 x 11 puzzle')



    v_clues = ((1, 1, 3), (3, 2, 1, 3), (2, 2), (3, 6, 3),
               (3, 8, 2), (15,), (8, 5), (15,),
               (7, 1, 4, 2), (7, 9,), (6, 4, 2,), (2, 1, 5, 4),
               (6, 4), (2, 6), (2, 5), (5, 2, 1),
               (6, 1), (3, 1), (1, 4, 2, 1), (2, 2, 2, 2))
    h_clues = ((2, 1, 1), (3, 4, 2), (4, 4, 2), (8, 3),
               (7, 2, 2), (7, 5), (9, 4), (8, 2, 3),
               (7, 1, 1), (6, 2), (5, 3), (3, 6, 3),
               (2, 9, 2), (1, 8), (1, 6, 1), (3, 1, 6),
               (5, 5), (1, 3, 8), (1, 2, 6, 1), (1, 1, 1, 3, 2))
    args = ((v_clues, h_clues), 20, 20)

    ans = ((1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1),
           (0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1),
           (1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0),
           (0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0),
           (0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1),
           (0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1),
           (0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0),
           (0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0),
           (0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0),
           (0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0),
           (0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0),
           (0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1),
           (1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1),
           (1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
           (1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0),
           (0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0),
           (0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0),
           (0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0),
           (0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1),
           (0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1))

    t3 = (args, ans, '20 x 20 puzzle')

    tests = [t1, t2, t3]
    return tests
