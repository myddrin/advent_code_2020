"""Template on how to do a new day - copy and use
Remember to add an import in aoc_2020.__init__
"""
import dataclasses
import re
from typing import (
    ClassVar,
    Dict,
    Optional,
    Set,
)

from aoc_2020 import BaseRunner


@dataclasses.dataclass(frozen=True)
class BagRule:
    rule_regex: ClassVar = re.compile(r'^(.+) bags contain (.+)\.$')
    contained_rule: ClassVar = re.compile(r'(\d+) (.+) bags?')
    NO_OTHER: ClassVar[str] = 'no other bags'

    bag_colour: str
    # dict[bag colour -> number of bags]
    contains: Dict[str, int] = dataclasses.field(default_factory=dict, hash=False)

    @property
    def is_empty(self) -> bool:
        return not bool(self.contains)

    @classmethod
    def from_line(cls, line: str) -> 'BagRule':
        bag_colour, contains_str = cls.rule_regex.match(line).groups()  # raises if does not match as None.groups()
        contains = {}

        for value in contains_str.split(', '):
            rule_match = cls.contained_rule.match(value)
            if rule_match is not None:  # skip 'no other bags'
                count_str, colour = rule_match.groups()
                contains[colour] = int(count_str)
        return cls(bag_colour, contains)


class AllRules:

    def __init__(self):
        # dict[bag colour -> BagRule]
        self._forward: Dict[str, BagRule] = {}
        # dict[bag colour -> list of bags that can contain it]
        # not necessarily fully populated
        self._backward_cache: Dict[str, Set[str]] = {}

    def get(self, colour: str) -> Optional[BagRule]:
        return self._forward.get(colour)

    def count_subbags(self, colour: str) -> Optional[int]:
        bag_rule = self._forward.get(colour)
        if not bag_rule:
            return None

        all_bags = []
        for sub_bag_colour, n_sub_bags in bag_rule.contains.items():
            all_bags.append(n_sub_bags + n_sub_bags * self.count_subbags(sub_bag_colour))

        return sum(all_bags)

    def get_backward(self, colour: str) -> Set[str]:
        if colour in self._backward_cache:
            return self._backward_cache[colour]

        found = set()

        for rule in self._forward.values():
            if colour in rule.contains:
                found.add(rule.bag_colour)
                found.update(self.get_backward(rule.bag_colour))

        self._backward_cache[colour] = found
        return found

    @classmethod
    def from_file(cls, filename: str) -> 'AllRules':
        print(f'Loading from {filename}')
        all_rules = cls()
        with open(filename, 'r') as fin:
            for line in fin:
                bag_rule = BagRule.from_line(line)
                if found := all_rules.get(bag_rule.bag_colour):
                    if not found.is_empty:
                        raise RuntimeError(f'Cannot replace {found} with {bag_rule}')

                all_rules._forward[bag_rule.bag_colour] = bag_rule

                for key in bag_rule.contains.keys():
                    if key not in all_rules._forward:
                        all_rules._forward[key] = BagRule(key)  # empty rule

        return all_rules


@BaseRunner.register
class Day07(BaseRunner):
    day = 7

    @classmethod
    def build_args(cls):
        parser = super().build_args()
        parser.add_argument('--search-colour', default='shiny gold')
        return parser

    @classmethod
    def q1(cls, all_rules: AllRules, colour: str) -> Set[str]:
        return all_rules.get_backward(colour)

    @classmethod
    def q2(cls, all_rule: AllRules, colour: str) -> int:
        return all_rule.count_subbags(colour)

    @classmethod
    def compute(cls, cli_args):
        all_rules = AllRules.from_file(cli_args.input)

        colours = cls.q1(all_rules, cli_args.search_colour)
        print(f'Q1: {len(colours)} different bag colours can contain a {cli_args.search_colour} bag')

        n_subbags = cls.q2(all_rules, cli_args.search_colour)
        print(f'Q2: {n_subbags} bags must be carried to have 1 {cli_args.search_colour} bag!')


if __name__ == '__main__':
    Day07.compute(Day07.build_args().parse_args())
