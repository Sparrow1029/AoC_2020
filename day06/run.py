#!/usr/bin/env python3
from pathlib import Path

DATA_FILE_PATH = Path(__file__).absolute().parent.parent / "data_files"

def get_groups_from_file(path):
    with open(path, "r") as f:
        return list(map(lambda s: s.strip().split("\n"), f.read().split("\n\n")))

def count_yes_answers(group: list):
    final = set()
    for string in group:
        final.update(set(string))
    return len(final)

def answers_intersection(group: list):
    final = set(group.pop())
    for string in group:
        final &= set(string)
    return len(final)

if __name__ == "__main__":
    groups = get_groups_from_file(DATA_FILE_PATH / "day06_input.txt")
    yes_answers = sum(count_yes_answers(grp) for grp in groups)
    print("PART 1: ", yes_answers)
    answered_by_all = sum(answers_intersection(grp) for grp in groups)
    print("PART 2: ", answered_by_all)