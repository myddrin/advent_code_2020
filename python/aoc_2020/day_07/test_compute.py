import os

import pytest

from aoc_2020.day_07.compute import (
    AllRules,
    BagRule,
    Day07,
)


@pytest.fixture(scope='session')
def small_ex_txt():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        'small_ex.txt',
    )


@pytest.fixture(scope='session')
def other_ex_txt():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        'other_ex.txt',
    )


@pytest.fixture(scope='session')
def input_txt():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        'input.txt',
    )


class TestBagRule:

    def test_from_line_single_value(self):
        assert BagRule.from_line('bright white bags contain 1 shiny gold bag.') == BagRule(
            'bright white',
            {'shiny gold': 1},
        )

    def test_from_line_multi_digit_value(self):
        assert BagRule.from_line('bright white bags contain 13 shiny gold bag.') == BagRule(
            'bright white',
            {'shiny gold': 13},
        )

    def test_from_line_multiple_values(self):
        assert BagRule.from_line('muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.') == BagRule(
            'muted yellow',
            {'shiny gold': 2, 'faded blue': 9},
        )

    def test_from_line_no_other_bags(self):
        assert BagRule.from_line('dotted black bags contain no other bags.') == BagRule(
            'dotted black',
        )


class TestAllRules:

    def test_small_ex(self, small_ex_txt):
        all_rules = AllRules.from_file(small_ex_txt)
        assert sorted(all_rules._forward.keys()) == [
            'bright white',
            'dark olive',
            'dark orange',
            'dotted black',
            'faded blue',
            'light red',
            'muted yellow',
            'shiny gold',
            'vibrant plum',
        ]
        assert all_rules.get('bright white') == BagRule(
            'bright white',
            {'shiny gold': 1},
        )
        assert all_rules.get('vibrant plum') == BagRule(
            'vibrant plum',
            {'faded blue': 5, 'dotted black': 6},
        )

    def test_get_backward(self, small_ex_txt):
        all_rules = AllRules.from_file(small_ex_txt)
        assert all_rules.get_backward('shiny gold') == {
            'bright white',
            'muted yellow',
            'dark orange',
            'light red',
        }

    def test_count_subbags_ex1(self, small_ex_txt):
        all_rules = AllRules.from_file(small_ex_txt)
        assert all_rules.count_subbags('shiny gold') == 32

    def test_count_subbags_ex2(self, other_ex_txt):
        all_rules = AllRules.from_file(other_ex_txt)
        assert all_rules.count_subbags('shiny gold') == 126


class TestDay07:

    def test_q1(self, input_txt):
        all_rules = AllRules.from_file(input_txt)
        assert len(Day07.q1(all_rules, 'shiny gold')) == 335

    def test_q2(self, input_txt):
        all_rules = AllRules.from_file(input_txt)
        assert Day07.q2(all_rules, 'shiny gold') == 2431
