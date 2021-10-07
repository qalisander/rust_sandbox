# https://www.codewars.com/kata/5a5072a6145c46568800004d/train/python
import itertools
import time
from enum import IntEnum, Enum


# TODO: benchmark and try call rust
# https://stackoverflow.com/questions/41770791/arrays-in-python-are-assigned-by-value-or-by-reference

class Bit(IntEnum):
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
        if len(clues.stack) <= next_index or clues.stack[clues.index] == Bit.EMPTY:
            return ShiftType.Banned

        current_index = clues.index - 1
        if 0 > current_index or clues.stack[current_index] == Bit.EMPTY:
            return ShiftType.Available

        return ShiftType.Mandatory

    return [get_next_bit_shifts(processed_top_clue)
            for processed_top_clue in processed_top_clues]


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

        has_not_enough_len = any(
            len(clues.stack) - clues.index - 1 > clues_len - current_clues_index for clues in top_clues)
        if has_not_enough_len:
            return False

        next_possible_bits = get_next_possible_bit_shifts(top_clues)

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


# quick replacement in numpy https://www.kite.com/python/answers/how-to-replace-elements-of-a-numpy-array-based-on-a-condition-in-python

def get_permutations(next_possible_shifts, clues, size):
    def get_permutations_rec(permutation, clues, init_offset: int, size: int):
        if len(clues) == 0:
            yield permutation
            return

        current_clue = clues[0]
        clues_sum = sum(clues)
        clues_borders = len(clues) - 1

        last_zero_index = init_offset + current_clue
        for i in range(last_zero_index - current_clue, last_zero_index):
            permutation[i] = 1

        for new_offset in range(init_offset, 1 + size - clues_sum - clues_borders):
            last_zero_index = new_offset + current_clue

            has_last_zero_valid = next_possible_shifts[last_zero_index] != ShiftType.Mandatory \
                if last_zero_index < len(next_possible_shifts) else True

            def has_zeroes_valid():
                zeroes_range = slice(init_offset, new_offset)
                return all([shift != ShiftType.Mandatory for shift in next_possible_shifts[zeroes_range]])

            def has_ones_valid():
                ones_range = slice(new_offset, last_zero_index)
                return all([shift != ShiftType.Banned for shift in next_possible_shifts[ones_range]])

            if has_last_zero_valid and has_zeroes_valid() and has_ones_valid():
                for perm in get_permutations_rec(permutation, clues[1:], 1 + new_offset + current_clue, size):
                    yield perm

            permutation[new_offset] = 0
            if len(permutation) > last_zero_index:
                permutation[last_zero_index] = 1

        for i in range(last_zero_index - current_clue, len(permutation)):
            permutation[i] = 0


    return get_permutations_rec([0 for _ in range(size)], clues, 0, size)


def test_get_permutations_15():
    permutations = list(map(
        lambda p: tuple(p),
        get_permutations([(ShiftType.Available, 0)] * 15, [1, 2, 3, 1], 15)))
    print(permutations)
    assert permutations[0] == (1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0)
    assert permutations[-1] == (0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1)


def test_solve_15():
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
    print('\n', *solved_solution, sep='\n')
    assert solved_solution == expected_solution


def test_test():
    foods = [
        ["Tomato and Cucumber", "Hummus, Beetroot, and Lettuce"],
        ["Cheese", "Egg"],
        ["Ham", "Bacon", "Chicken Club", "Tuna"]
    ]

    new_foods = [food for sublist in foods for food in sublist]
    print(new_foods)

    assert 1 == Bit.FILLED


def something(duration=0.000001):
    """
    Function that needs some serious benchmarking.
    """
    time.sleep(duration)
    # You may return anything you want, like the result of a computation
    return 123


def test_my_stuff(benchmark):
    # benchmark something
    result = benchmark(something)

    # Extra code, to verify that the run completed correctly.
    # Sometimes you may want to check the result, fast functions
    # are no good if they return incorrect results :-)
    assert result == 123
