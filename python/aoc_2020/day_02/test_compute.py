import os

import pytest

from .compute import (
    Day02,
    Rule,
)


@pytest.fixture(scope='session')
def small_ex_txt():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        'small_ex.txt',
    )


@pytest.fixture(scope='session')
def input_txt():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        'input.txt',
    )


class TestRule:

    @pytest.mark.parametrize('line, rule', (
        ('1-3 a: abcde', Rule(1, 3, 'a', 'abcde')),
        ('1-3 b: cdefg', Rule(1, 3, 'b', 'cdefg')),
        ('2-9 c: ccccccccc', Rule(2, 9, 'c', 'ccccccccc')),
    ))
    def test_from_line(self, line, rule):
        assert Rule.from_line(line) == rule

    @pytest.mark.parametrize('rule, expected', (
        (Rule(1, 3, 'a', 'abcde'), True),
        (Rule(1, 3, 'b', 'cdefg'), False),
        (Rule(2, 9, 'c', 'ccccccccc'), True),
    ))
    def test_respect_policy_q1(self, rule, expected):
        assert rule.respect_policy_q1() is expected

    @pytest.mark.parametrize('rule, expected', (
        (Rule(1, 3, 'a', 'abcde'), True),
        (Rule(1, 3, 'b', 'cdefg'), False),
        (Rule(2, 9, 'c', 'ccccccccc'), False),
    ))
    def test_respect_policy_q2(self, rule, expected):
        assert rule.respect_policy_q2() is expected


class TestDay02:

    def test_load_file(self, small_ex_txt):
        assert Day02.load_file(small_ex_txt) == [
            Rule(1, 3, 'a', 'abcde'),
            Rule(1, 3, 'b', 'cdefg'),
            Rule(2, 9, 'c', 'ccccccccc'),
        ]

    def test_compute_q1_example(self, small_ex_txt):
        assert Day02.compute_q1(
            Day02.load_file(small_ex_txt),
        ) == 2

    def test_compute_q1(self, input_txt):
        assert Day02.compute_q1(
            Day02.load_file(input_txt),
        ) == 422

    def test_compute_q2_example(self, small_ex_txt):
        assert Day02.compute_q2(
            Day02.load_file(small_ex_txt),
        ) == 1

    def test_compute_q2(self, input_txt):
        assert Day02.compute_q2(
            Day02.load_file(input_txt),
        ) == 451
