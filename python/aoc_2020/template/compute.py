"""Template on how to do a new day - copy and use
Remember to add an import in aoc_2020.__init__
"""
from aoc_2020 import BaseRunner


@BaseRunner.register
class DayXX(BaseRunner):
    day = 1

    @classmethod
    def compute(cls, cli_args):
        raise NotImplementedError()


if __name__ == '__main__':
    DayXX.compute(DayXX.build_args().parse_args())
