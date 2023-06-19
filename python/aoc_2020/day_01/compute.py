from typing import (
    List,
    Optional,
    Tuple,
)

from aoc_2020.template.base_runner import BaseRunner


@BaseRunner.register
class Day01(BaseRunner):
    day = 1

    def __init__(self, values: List[int], *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.unique_entries = set(values)

    @classmethod
    def load_file(cls, filename: str) -> 'Day01':
        rv = []
        print(f'Loading from {filename}')
        with open(filename, 'r') as fin:
            for line in fin:
                rv.append(int(line))
        print(f'Loaded {len(rv)}')
        return cls(rv)

    def find_pairs(self, target: int) -> Optional[Tuple[int, int]]:
        # This has the unique bug if the 1/2 of the target is present once in the input we would
        # use it as a solution...
        for value in self.unique_entries:
            other = target - value
            if other in self.unique_entries:
                return value, other

    def find_triplets(self, target: int) -> Optional[Tuple[int, int, int]]:
        for value in self.unique_entries:
            other = target - value
            # This has the unique bug if the 1/3 of the target is present once in the input we would
            # use it as a solution...
            other_pair = self.find_pairs(other)
            if other_pair is not None:
                return value, other_pair[0], other_pair[1]

    @classmethod
    def multiply_all(cls, *args: int) -> int:
        if not args:
            raise ValueError('Need some values')
        rv = args[0]
        for number in args[1:]:
            rv = rv * number
        return rv

    @classmethod
    def compute(cls, cli_args):
        obj = cls.load_file(cli_args.input)
        q1_pair = obj.find_pairs(2020)
        if q1_pair is not None:
            print(f'Q1 is {q1_pair}=>{cls.multiply_all(*q1_pair)}')
        else:
            print('Q1 has no answer')

        q2_triplet = obj.find_triplets(2020)
        if q2_triplet is not None:
            print(f'Q2 is {q2_triplet}=>{cls.multiply_all(*q2_triplet)}')
        else:
            print('Q2 has no answer')


if __name__ == '__main__':
    Day01.compute(Day01.build_args().parse_args())
