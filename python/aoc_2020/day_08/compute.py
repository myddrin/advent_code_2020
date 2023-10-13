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
    @classmethod
    def from_file(cls, filename: str):
        print(f'Loading program from {filename}')
        program: List[Instruction] = []
        with open(filename, 'r') as fin:
            for line in fin:
                program.append(Instruction.from_string(line))
        print(f'  -> {len(program)} instructions')
        return cls(program)

    def __init__(self, program: List[Instruction]):
        self.accumulator = 0
        self.program = program

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
    def q1(cls, progm: Program) -> int:
        """Run the program and return the accumulator value wen InfiniteLoop is caught"""
        try:
            progm.run()
        except InfiniteLoop as e:
            print(e)
            return progm.accumulator
        else:
            raise RuntimeError(f'Should not have finished! But accumulator is {progm.accumulator}.')

    @classmethod
    def naive_q2(cls, original_program: Program) -> Program:
        """
        naive solution: iterate all instruction, if they are JUMP create a new prog with a NOOP
        and if they are NOOP make it JUMP -> if it works then it was the right one
        """
        attempt = 0
        for inst_idx, instruction in enumerate(original_program.program):
            if instruction.operation in (Operation.Jmp, Operation.Nop):
                attempt += 1
                new_op = Operation.Jmp if instruction.operation == Operation.Nop else Operation.Nop
                new_instructions = original_program.program[:inst_idx] + [
                    Instruction(new_op, instruction.argument),
                ] + original_program.program[inst_idx + 1:]

                new_program = Program(new_instructions)
                try:
                    new_program.run()
                except:
                    pass  # failed
                else:
                    print(f'Successfully changed {inst_idx} from {instruction.operation} to {new_op} at {attempt=}')
                    return new_program
        raise RuntimeError('Tried all changes and found no runnable program')

    @classmethod
    def compute(cls, cli_args):
        program = Program.from_file(cli_args.input)

        q1 = cls.q1(program)
        print(f'Q1: Caught infinite loop when accumulator has value: {q1}')

        fixed_program = cls.naive_q2(program)
        print(f'Q2: accumulator has value: {fixed_program.accumulator}')


if __name__ == '__main__':
    Day08.compute(Day08.build_args().parse_args())
