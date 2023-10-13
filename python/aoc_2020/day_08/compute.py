import dataclasses
from enum import Enum
from typing import List

from aoc_2020 import BaseRunner


class Operation(Enum):
    Acc = 'acc'
    Jmp = 'jmp'
    Nop = 'nop'


@dataclasses.dataclass(frozen=True)
class Instruction:
    operation: Operation
    argument: int

    @classmethod
    def from_string(cls, value: str) -> 'Instruction':
        op_name, str_arg = value.split(' ')
        return cls(
            Operation(op_name),
            int(str_arg),
        )


class InfiniteLoop(RuntimeError):
    """Infinite loop detected"""
    pass


class OutOfBounds(RuntimeError):
    """Ran outside the program"""
    pass


class Program:
    def __init__(self, filename: str):
        self.accumulator = 0

        print(f'Loading program from {filename}')
        self.program: List[Instruction] = []
        with open(filename, 'r') as fin:
            for line in fin:
                self.program.append(Instruction.from_string(line))
        print(f'  -> {len(self.program)} instructions')

    def _process(self, instruction: Instruction) -> int:
        """Process an instruction and return next instruction modifier"""
        if instruction.operation == Operation.Acc:
            self.accumulator += instruction.argument
            return 1
        elif instruction.operation == Operation.Jmp:
            return instruction.argument
        elif instruction.operation == Operation.Nop:
            return 1
        else:
            raise ValueError(f'Unsupported OP: {instruction.operation}')

    def run(self):
        self.accumulator = 0  # reset accumulator
        current_instruction = 0
        visited_instructions = set()
        exec = 0

        while True:
            exec += 1
            if current_instruction == len(self.program):
                return  # reached end of program
            elif current_instruction < 0 or current_instruction > len(self.program):
                raise OutOfBounds(f'{current_instruction=}')

            action = self.program[current_instruction]
            if current_instruction in visited_instructions:
                raise InfiniteLoop(f'Found at {exec=} {current_instruction=}: {action}')
            visited_instructions.add(current_instruction)

            current_instruction += self._process(action)


@BaseRunner.register
class Day08(BaseRunner):
    day = 8

    @classmethod
    def q1(cls, progm: Program):
        try:
            progm.run()
        except InfiniteLoop as e:
            print(e)
            return progm.accumulator
        else:
            raise RuntimeError('Should not have finished')

    @classmethod
    def compute(cls, cli_args):
        program = Program(cli_args.input)

        q1 = cls.q1(program)
        print(f'Q1: Caught infinite loop when accumulator has value: {q1}')


if __name__ == '__main__':
    Day08.compute(Day08.build_args().parse_args())
