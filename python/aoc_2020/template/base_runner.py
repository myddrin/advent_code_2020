import abc
from argparse import ArgumentParser
from typing import (
    Dict,
    Type,
)


class BaseRunner(abc.ABC):
    registered: Dict[int, Type['BaseRunner']] = {}
    day: int = NotImplemented

    @classmethod
    @abc.abstractmethod
    def compute(cls, cli_args):
        raise NotImplementedError()

    @classmethod
    def build_args(cls):
        parser = ArgumentParser()
        parser.add_argument('--input', type=str, default='input.txt', help='Input file')
        return parser

    @classmethod
    def register(cls, sub_cls: Type['BaseRunner']):
        cls.registered[sub_cls.day] = sub_cls
        return sub_cls
