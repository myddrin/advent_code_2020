import os

import pytest

from aoc_2020.day_10.compute import (
    Adapters,
    Day10,
)


@pytest.fixture(scope='session')
def small_ex_txt():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        'small_ex.txt',
    )


tiny_example = [
    16,
    10,
    15,
    5,
    1,
    11,
    7,
    19,
    6,
    12,
    4,
]


@pytest.fixture(scope='session')
def input_txt():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        'input.txt',
    )


class TestAdapters:

    def test_device_joltage_tiny(self):
        obj = Adapters(sorted(tiny_example))
        assert obj.device_joltage == 22

    def test_compute_difference_tiny(self):
        obj = Adapters(sorted(tiny_example))
        assert obj.compute_differences() == {
            0: 0,
            1: 7,
            2: 0,
            3: 5,
        }

    def test_count_permutations_tiny(self):
        obj = Adapters(sorted(tiny_example))
        assert obj.count_permutations() == 8

    def test_count_permutations_small(self, small_ex_txt):
        obj = Adapters.from_file(small_ex_txt)
        assert obj.count_permutations() == 19208

    def test_small_ex(self, small_ex_txt):
        obj = Adapters.from_file(small_ex_txt)
        assert obj.device_joltage == 52
        assert obj.compute_differences() == {
            0: 0,
            1: 22,
            2: 0,
            3: 10,
        }

    def test_input_has_no_duplicates(self, input_txt):
        obj = Adapters.from_file(input_txt)
        assert obj.compute_differences()[0] == 0

    def test_q1(self, input_txt):
        obj = Adapters.from_file(input_txt)
        assert Day10.q1(obj) == 2080

    # def test_q2(self, input_txt):
    #     obj = Adapters.from_file(input_txt)
    #     assert Day10.q2(obj) == 6908379398144
