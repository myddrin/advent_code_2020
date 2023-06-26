"""
https://adventofcode.com/2020/day/2
"""
import dataclasses
import re
from typing import (
    ClassVar,
    List,
)

from aoc_2020.template.base_runner import BaseRunner


@dataclasses.dataclass(frozen=True)
class Rule:
    first: int
    second: int
    letter: str
    password: str

    line_re: ClassVar = re.compile(r'(\d+)-(\d+)\s(\w):\s(\w+)')

    @classmethod
    def from_line(cls, line: str) -> 'Rule':
        pattern = cls.line_re.match(line)
        if pattern is not None:
            return cls(
                first=int(pattern.group(1)),
                second=int(pattern.group(2)),
                letter=pattern.group(3),
                password=pattern.group(4),
            )
        raise ValueError('f"{line}" did not match regex')

    def respect_policy_q1(self) -> bool:
        letter_count = sum((
            1 if letter == self.letter else 0
            for letter in self.password
        ))
        return self.first <= letter_count <= self.second

    def respect_policy_q2(self) -> bool:
        if len(self.password) < max((self.first, self.second)):
            return False
        first_index_valid = self.password[self.first - 1] == self.letter
        second_index_valid = self.password[self.second - 1] == self.letter
        return first_index_valid ^ second_index_valid


@BaseRunner.register
class Day02(BaseRunner):
    day = 2

    @classmethod
    def load_file(cls, filename: str) -> List[Rule]:
        print(f'Loading rules from {filename}')
        with open(filename, 'r') as fin:
            return [
                Rule.from_line(line)
                for line in fin
            ]

    @classmethod
    def compute_q1(cls, rules: List[Rule]) -> int:
        return sum((
            1 if rule.respect_policy_q1() else 0
            for rule in rules
        ))

    @classmethod
    def compute_q2(cls, rules: List[Rule]) -> int:
        return sum((
            1 if rule.respect_policy_q2() else 0
            for rule in rules
        ))

    @classmethod
    def compute(cls, cli_args):
        rules = cls.load_file(cli_args.input)

        q1 = cls.compute_q1(rules)
        print(f'Q1: {q1} rules respect the policy')
        q2 = cls.compute_q2(rules)
        print(f'Q2: {q2} rules respect the policy')


if __name__ == '__main__':
    Day02.compute(Day02.build_args().parse_args())
