import dataclasses
from copy import deepcopy
from enum import Enum
from typing import (
    Dict,
    List,
)

from aoc_2020 import BaseRunner


class Cell(Enum):
    Floor = '.'
    EmptySeat = 'L'
    OccupiedSeat = '#'


@dataclasses.dataclass(frozen=True,repr=False)
class Coordinate:
    x: int = 0
    y: int = 0

    def all_adjacent(self) -> List['Coordinate']:
        adj = []
        for i in range(self.x - 1, self.x + 2):
            for j in range(self.y - 1, self.y + 2):
                if i == self.x and j == self.y:
                    continue
                adj.append(Coordinate(i, j))
        return adj

    def __repr__(self):
        return f'({self.x},{self.y})'


@dataclasses.dataclass
class SeatMap:
    # map of seats occupancy
    map: Dict[Coordinate, bool]
    neighbours: Dict[Coordinate, List[Coordinate]]
    occupied_threshold: int = 4

    @classmethod
    def compute_nearest_neighbours(cls, seat_map: Dict[Coordinate, bool]) -> Dict[Coordinate, List[Coordinate]]:
        direct_neighbours: Dict[Coordinate, List[Coordinate]] = {}
        for coord in seat_map.keys():
            neighbours = [n for n in coord.all_adjacent() if n in seat_map]
            direct_neighbours[coord] = neighbours
        return direct_neighbours

    @classmethod
    def compute_first_seen_neighbours(cls, seat_map: Dict[Coordinate, bool]) -> Dict[Coordinate, List[Coordinate]]:
        first_seen_neighbours: Dict[Coordinate, List[Coordinate]] = {}
        length = max((c.x for c in seat_map.keys())) + 1  # starts at 0
        height = max((c.y for c in seat_map.keys())) + 1
        for coord in seat_map.keys():
            neighbours = []
            for adj in coord.all_adjacent():
                direction = Coordinate(adj.x - coord.x, adj.y - coord.y)

                while True:
                    is_seat = adj in seat_map
                    if is_seat:
                        neighbours.append(adj)
                        break
                    # otherwise continue until the end
                    adj = Coordinate(adj.x + direction.x, adj.y + direction.y)
                    if adj.x < 0 or adj.x >= length or adj.y < 0 or adj.y > height:
                        break
            first_seen_neighbours[coord] = neighbours

        return first_seen_neighbours

    @classmethod
    def from_file(cls, filename: str, *, direct_neighbours: bool = True, occupied_threshold: int = 4) -> 'SeatMap':
        print(f'Opening {filename}')
        loaded_map = {}
        with open(filename, 'r') as fin:
            for y, line in enumerate(fin):
                for x, character in enumerate(line):
                    if character not in (Cell.EmptySeat.value, Cell.OccupiedSeat.value):
                        continue  # not interesting

                    pos = Coordinate(x, y)
                    loaded_map[pos] = character == Cell.OccupiedSeat.value

        if direct_neighbours:
            neighbours = cls.compute_nearest_neighbours(loaded_map)
        else:
            neighbours = cls.compute_first_seen_neighbours(loaded_map)
        return cls(
            map=loaded_map,
            neighbours=neighbours,
            occupied_threshold=occupied_threshold,
        )

    @property
    def occupied_seats(self) -> int:
        return sum((
            1
            for v in self.map.values()
            if v
        ))

    def count_nearby_occupied(self, where: Coordinate) -> int:
        neighbours = 0
        for o in self.neighbours[where]:
            if self.map.get(o):  # None or False are rejected
                neighbours += 1
        return neighbours

    def simulate_people(self) -> bool:
        # rules:
        # If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
        # If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
        # Otherwise, the seat's state does not change.
        new_map = {}
        changed = False
        for coord, is_occupied in self.map.items():
            neighbours = self.count_nearby_occupied(coord)
            new_state = is_occupied
            if not is_occupied and neighbours == 0:
                new_state = True
            elif is_occupied and neighbours >= self.occupied_threshold:
                new_state = False

            changed |= new_state != is_occupied
            new_map[coord] = new_state

        if changed:
            self.map = new_map
        return changed


@BaseRunner.register
class Day11(BaseRunner):
    day = 11

    @classmethod
    def simulate(cls, seats: SeatMap) -> int:
        i = 1
        while seats.simulate_people():
            if i % 10000 == 0:
                print(f'Simulated {i} rounds so far...')
            i += 1
        print(f'Stabilised in {i} rounds')
        return seats.occupied_seats

    @classmethod
    def compute(cls, cli_args):
        n_occupied_1 = cls.simulate(SeatMap.from_file(cli_args.input, direct_neighbours=True, occupied_threshold=4))
        print(f'Q1: {n_occupied_1} are occupied')

        n_occupied_2 = cls.simulate(SeatMap.from_file(cli_args.input, direct_neighbours=False, occupied_threshold=5))
        print(f'Q2: {n_occupied_2} are occupied')


if __name__ == '__main__':
    Day11.compute(Day11.build_args().parse_args())
