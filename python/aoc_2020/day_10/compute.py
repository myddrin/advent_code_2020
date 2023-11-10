import dataclasses
from typing import (
    Dict,
    List,
)

from aoc_2020 import BaseRunner


@dataclasses.dataclass
class Adapters:
    original: List[int]  # assumed to be sorted

    @classmethod
    def from_file(cls, filename: str) -> 'Adapters':
        print(f'Loading {filename}')
        adapters = []
        with open(filename, 'r') as fin:
            for line in fin:
                adapters.append(int(line))

        return cls(sorted(adapters))

    @property
    def device_joltage(self):
        return self.original[-1] + 3

    def compute_differences(self) -> Dict[int, int]:
        diffs = {
            0: 0,
            1: 0,
            2: 0,
            3: 0,
            # higher than 3 and we're in trouble!
        }

        current = 0
        for joltage in self.original + [self.device_joltage]:
            potential = joltage - current
            diffs[potential] += 1
            current = joltage

        return diffs


@BaseRunner.register
class Day10(BaseRunner):
    day = 10

    @classmethod
    def q1(cls, data: Adapters) -> int:
        differences = data.compute_differences()
        return differences[1] * differences[3]

    @classmethod
    def compute(cls, cli_args):

        data = Adapters.from_file(cli_args.input)

        print(f'Q1 answer is {cls.q1(data)}')


if __name__ == '__main__':
    Day10.compute(Day10.build_args().parse_args())
