"""Template on how to test a new day - copy and use"""
import os

import pytest

from aoc_2020.day_11.compute import (
    Day11,
    SeatMap,
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


class TestSeatMap:
    def test_load(self, small_ex_txt):
        seat_map = SeatMap.from_file(small_ex_txt)
        assert len(seat_map.map) == 71
        assert seat_map.occupied_seats == 0

    def test_simulate_turn_1(self, small_ex_txt):
        seat_map = SeatMap.from_file(small_ex_txt)
        old_map = seat_map.map

        assert seat_map.simulate_people() is True
        assert seat_map.map is not old_map
        assert len(seat_map.map) == 71
        assert seat_map.occupied_seats == 71

    def test_simulate_turns(self, small_ex_txt):
        seat_map = SeatMap.from_file(small_ex_txt)

        i = 0
        for i, expected in enumerate((71, 20, 51, 30, 37), start=1):
            assert seat_map.simulate_people() is True
            assert seat_map.occupied_seats == expected, f'On turn {i}'

        i += 1
        assert seat_map.simulate_people() is False, f'On turn {i} we should be settled'
        assert seat_map.occupied_seats == 37


class TestDay11:

    def test_q1_small_ex(self, small_ex_txt):
        map = SeatMap.from_file(small_ex_txt)
        assert Day11.q1(map) == 37

    def test_q1(self, input_txt):
        map = SeatMap.from_file(input_txt)
        assert Day11.q1(map) == 2247
