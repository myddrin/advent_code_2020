"""Template on how to test a new day - copy and use"""
import os

import pytest

from aoc_2020.day_09.compute import DataTransmission


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


class TestDataTransmission:

    def test_get_numbers_before(self):
        data = DataTransmission(
            list(range(1, 26)),
        )
        assert data.get_numbers_before(5, 5) == set(range(1, 6))

    @pytest.mark.parametrize('number, expected', (
        (26, True),
        (49, True),
        (100, False),
        (50, False),
    ))
    def test_is_valid(self, number, expected):
        assert DataTransmission.is_valid(number, set(range(1, 26))) is expected

    def test_find_first_invalid_small_ex(self, small_ex_txt):
        res = DataTransmission.from_file(small_ex_txt).find_first_invalid(5)
        assert res == 127

    def test_find_first_invalid_input(self, input_txt):
        res = DataTransmission.from_file(input_txt).find_first_invalid(25)
        assert res == 1930745883

    def test_find_continuous_small_ex(self, small_ex_txt):
        data = DataTransmission.from_file(small_ex_txt)
        res = data.find_continuous_set_that_sums_to(127)
        assert res == (2, 5)
        assert data.metric_from_range(*res) == 62

    def test_find_continuous_input(self, input_txt):
        data = DataTransmission.from_file(input_txt)
        res = data.find_continuous_set_that_sums_to(1930745883)
        assert res == (553, 569)
        assert data.metric_from_range(*res) == 268878261
