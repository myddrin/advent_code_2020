import dataclasses
from typing import (
    List,
    Set,
    Tuple,
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

        print(f'   -> Loaded {len(numbers)} numbers')
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

    def find_continuous_set_that_sums_to(self, target: int) -> Tuple[int, int]:
        """Returns first and last index of the set"""
        start_index = 0
        end_index = 1
        current_sum = sum(self.numbers[start_index:end_index + 1])

        iteration = 0
        while end_index < len(self.numbers) - 1:
            iteration += 1

            # print(
            #     f'Debug: [{start_index}]{self.numbers[start_index]} [{end_index}]{self.numbers[end_index]} '
            #     f'-> {current_sum=} {target=}',
            # )

            if current_sum == target:
                print(f'Found sum in {iteration} iterations')
                return start_index, end_index
            elif current_sum > target:
                # print('Debug: shrink')
                current_sum -= self.numbers[start_index]
                start_index += 1
            elif current_sum < target:
                # print('Debug: grow')
                end_index += 1
                current_sum += self.numbers[end_index]

            if start_index == end_index:
                raise RuntimeError('Cannot happen?')

        raise RuntimeError('Did not find a pair')

    def metric_from_range(self, start_index: int, end_index: int) -> int:
        """Returns min and max numbers from a range"""
        range = sorted(self.numbers[start_index:end_index + 1])
        return range[0] + range[-1]

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

        first_invalid = data.find_first_invalid(cli_args.preamble_size)
        print(f'Q1: first invalid number is {first_invalid}')

        range = data.find_continuous_set_that_sums_to(first_invalid)
        metric = data.metric_from_range(*range)
        print(f'Q2: found sum using range {range} which sums to {metric}')


if __name__ == '__main__':
    Day09.compute(Day09.build_args().parse_args())
