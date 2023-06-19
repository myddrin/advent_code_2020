import os

import pytest

from .compute import Day01


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


def test_q1_small(small_ex_txt):
    answer = Day01.load_file(small_ex_txt).find_pairs(2020)
    assert list(sorted(answer)) == [299, 1721]
    assert Day01.multiply_all(*answer) == 514579, 'sanity'


def test_q1(input_txt):
    answer = Day01.load_file(input_txt).find_pairs(2020)
    assert list(sorted(answer)) == [144, 1876]
    assert Day01.multiply_all(*answer) == 270144, 'sanity'


def test_q2_small(small_ex_txt):
    answer = Day01.load_file(small_ex_txt).find_triplets(2020)
    assert list(sorted(answer)) == [366, 675, 979]
    assert Day01.multiply_all(*answer) == 241861950


def test_q2(input_txt):
    answer = Day01.load_file(input_txt).find_triplets(2020)
    assert list(sorted(answer)) == [512, 513, 995]
    assert Day01.multiply_all(*answer) == 261342720
