import sys
from pathlib import Path
from typing import List
from collections import namedtuple

sys.path.append(str(Path(__file__).absolute().parent.parent))
from helpers.python_helpers import DATA_FILE_PATH

Instruction = namedtuple("Instruction", "code val")


class BootCode:
    def __init__(self, instructions: List[Instruction]):
        self.ROM = instructions
        self.program = self.ROM[:]
        self.cursor_idx = 0
        self.accumulator = 0
        self.visited = set()

    def nop(self, val):
        self.visited.add(self.cursor_idx)
        self.cursor_idx += 1

    def acc(self, val):
        self.visited.add(self.cursor_idx)
        self.accumulator += val
        self.cursor_idx += 1

    def jmp(self, val):
        self.visited.add(self.cursor_idx)
        self.cursor_idx += val

    def execute(self, instr: Instruction):
        getattr(self, instr.code)(instr.val)

    def run(self):
        while self.cursor_idx not in self.visited:
            self.execute(self.program[self.cursor_idx])
        print(self.accumulator)

    def run_debug(self):
        swap = {"nop": "jmp", "jmp": "nop"}
        target_exit = len(self.program)
        print("TARGET EXIT INSTR INDEX: ", target_exit)
        # ixut: "index under test"
        for ixut in range(len(self.program)):
            if self.program[ixut].code in ["nop", "jmp"]:
                old_code, val = self.program[ixut]
                new_instr = Instruction(swap[old_code], val)
                self.program[ixut] = new_instr

                while self.cursor_idx not in self.visited:
                    self.execute(self.program[self.cursor_idx])
                    if self.cursor_idx == target_exit:
                        print("SUCCESS!")
                        print(f"Faulty instruction at index [{ixut}] replaced")
                        print(f"was '{old_code} {val}' now '{new_instr.code}' {new_instr.val}")
                        print("Accumulator value: ", self.accumulator)
                        return
            self.reset()

    def reset(self):
        self.cursor_idx = 0
        self.accumulator = 0
        self.visited.clear()
        self.program = self.ROM[:]
        # print(self)

    def __repr__(self):
        lines = "\n".join(f"{i.code} {i.val}" for i in self.program)
        return f"CORE DUMP:\n  - cursor_idx: {self.cursor_idx}\n  - accumulator: {self.accumulator}\n{self.visited}\n  {lines}"


def create_instruction_from_line(line: str) -> Instruction:
    code, val = line.split()
    return Instruction(code, int(val))


if __name__ == "__main__":
    with open(DATA_FILE_PATH / "day08_input.txt", "r") as f:
        instructions = [create_instruction_from_line(line) for line in f.read().splitlines()]

    print(
        "PART 1: Run your copy of the boot code. "
        "Immediately before any instruction is executed a second time, "
        "what value is in the accumulator?"
    )
    program = BootCode(instructions)
    program.run()

    print(
        "\nPART 2: DEBUGGING - Fix the program so that it terminates normally "
        "by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator "
        "after the program terminates?"
    )
    program = BootCode(instructions)
    program.run_debug()
