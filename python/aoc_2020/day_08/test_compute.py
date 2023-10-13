import os

import pytest

from aoc_2020.day_08.compute import (
    Day08,
    Program,
)


@pytest.fixture(scope='session')
def small_ex_txt():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        'small_ex.txt',
    )


@pytest.fixture(scope='session')
def input_txt():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        'input.txt',
    )


class TestCompute:

    def test_q1_small(self, small_ex_txt):
        program = Program(small_ex_txt)
        assert Day08.q1(program) == 5

    def test_q1(self, input_txt):
        program = Program(input_txt)
        assert Day08.q1(program) == 1420
