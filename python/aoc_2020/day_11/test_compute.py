"""Template on how to test a new day - copy and use"""
import os

import pytest

from aoc_2020.day_11.compute import (
    Coordinate,
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
    @pytest.mark.parametrize('pattern, coord, expected_neighbours, expected_occupied', (
        ('seat_setup_1.txt', Coordinate(3, 4), 8, 8),
        ('seat_setup_2.txt', Coordinate(1, 1), 1, 0),
        ('seat_setup_2.txt', Coordinate(3, 1), 2, 1),
        ('seat_setup_3.txt', Coordinate(3, 3), 0, 0),
    ))
    def test_first_seen_neighbours(self, pattern, coord, expected_neighbours, expected_occupied):
        filename = os.path.join(
            os.path.dirname(os.path.realpath(__file__)),
            pattern,
        )
        seat_map = SeatMap.from_file(filename, direct_neighbours=False)
        assert seat_map.map.keys() == seat_map.neighbours.keys()

        assert coord in seat_map.neighbours
        assert len(seat_map.neighbours[coord]) == expected_neighbours
        assert seat_map.map[coord] is False
        assert seat_map.count_nearby_occupied(coord) == expected_occupied

    @pytest.mark.parametrize('direct_neighbours', (True, False))
    def test_load(self, small_ex_txt, direct_neighbours):
        seat_map = SeatMap.from_file(small_ex_txt, direct_neighbours=direct_neighbours)
        assert seat_map.neighbours is not None
        assert len(seat_map.map) == 71
        assert seat_map.occupied_seats == 0
        assert seat_map.map.keys() == seat_map.neighbours.keys()

    def test_simulate_turns(self, small_ex_txt):
        seat_map = SeatMap.from_file(small_ex_txt)

        i = 0
        for i, expected in enumerate((71, 20, 51, 30, 37), start=1):
            assert seat_map.simulate_people() is True
            assert seat_map.occupied_seats == expected, f'On turn {i}'

        i += 1
        assert seat_map.simulate_people() is False, f'On turn {i} we should be settled'
        assert seat_map.occupied_seats == 37

    def test_first_seen_neighbours_simulation(self, small_ex_txt):
        seat_map = SeatMap.from_file(small_ex_txt, direct_neighbours=False, occupied_threshold=5)

        i = 0
        for i, expected in enumerate((71, 7, 53, 18, 31, 26), start=1):
            assert seat_map.simulate_people() is True
            assert seat_map.occupied_seats == expected, f'On turn {i}'

        i += 1
        assert seat_map.simulate_people() is False, f'On turn {i} we should be settled'
        assert seat_map.occupied_seats == 26


class TestDay11:

    def test_q1_small_ex(self, small_ex_txt):
        seat_map = SeatMap.from_file(small_ex_txt)
        assert Day11.simulate(seat_map) == 37

    def test_q1(self, input_txt):
        seat_map = SeatMap.from_file(input_txt)
        assert Day11.simulate(seat_map) == 2247

    def test_q2_small_ex(self, small_ex_txt):
        seat_map = SeatMap.from_file(small_ex_txt, direct_neighbours=False, occupied_threshold=5)
        assert Day11.simulate(seat_map) == 26

    def test_q2(self, input_txt):
        seat_map = SeatMap.from_file(input_txt, direct_neighbours=False, occupied_threshold=5)
        assert Day11.simulate(seat_map) == 2011
