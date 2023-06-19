"""Advent of code 2020: https://adventofcode.com/2020/"""
import sys

from aoc_2020.template.base_runner import BaseRunner
import aoc_2020.day_01.compute  # noqa - make it available


def compute():
    registered_days = list(sorted(BaseRunner.registered.keys()))
    if len(sys.argv) < 2:
        raise RuntimeError(f'Need at least the day: select from {registered_days}')
    try:
        selected_day = int(sys.argv[1])
    except ValueError:
        print(f'First parameter is the day, select from {registered_days}')
    else:
        print(f'Selected day {selected_day:02d}')

        runner = BaseRunner.registered[selected_day]
        cli_args = runner.build_args().parse_args(args=sys.argv[2:])
        runner.compute(cli_args)
