import dataclasses
from typing import (
    List,
    Set,
)

from aoc_2020 import BaseRunner


@dataclasses.dataclass
class DataTransmission:
    numbers: List[int]

    @classmethod
    def from_file(cls, filename: str) -> 'DataTransmission':
        numbers = []
        print(f'Loading data from {filename}')
        with open(filename, 'r') as fin:
            for line in fin:
                numbers.append(int(line))

        print(f'   -> Loaded {len(numbers)}')
        return cls(numbers)

    def get_numbers_before(self, index: int, preamble_size: int) -> Set[int]:
        return set(self.numbers[index - preamble_size: index])

    @classmethod
    def is_valid(cls, number: int, preamble_numbers: Set[int]) -> bool:
        for candidate in preamble_numbers:
            difference = number - candidate
            if difference != candidate and difference in preamble_numbers:
                return True

        # Could not find it
        return False

    def find_first_invalid(self, preamble_size: int = 25) -> int:
        for current_index in range(preamble_size, len(self.numbers) + 1):
            current_number = self.numbers[current_index]
            previous = self.get_numbers_before(current_index, preamble_size)
            if not self.is_valid(current_number, previous):
                return current_number

        raise RuntimeError('Could not find an invalid number')


@BaseRunner.register
class Day09(BaseRunner):
    day = 9

    @classmethod
    def build_args(cls):
        parser = super().build_args()
        parser.add_argument('--preamble-size', type=int, default=25)
        return parser

    @classmethod
    def compute(cls, cli_args):
        data = DataTransmission.from_file(cli_args.input)

        first_invaild = data.find_first_invalid(cli_args.preamble_size)
        print(f'Q1: first invalid number is {first_invaild}')


if __name__ == '__main__':
    Day09.compute(Day09.build_args().parse_args())
