import sys
import itertools
from pathlib import Path

sys.path.append(str(Path(__file__).absolute().parent.parent))
from helpers.python_helpers import DATA_FILE_PATH

def is_anomaly(window, next_num):
    sums = set(sum(combo) for combo in itertools.combinations(window, 2))
    return next_num not in sums

def find_anomaly_in_data(data, preamble_size):
    for idx in range(len(data) - preamble_size - 1):
        preamble = data[idx:idx+preamble_size]
        next_num = data[idx+preamble_size]
        if is_anomaly(preamble, next_num):
            return next_num

def find_range_addends(data, target_num):
    cur_window_size = 2
    found = False
    while not found:
        for i in range(len(data)-cur_window_size):
            slice = data[i:i+cur_window_size]
            if sum(slice) == target_num:
                found = True
                break
        cur_window_size += 1
    return min(slice) + max(slice)

if __name__ == "__main__":

    with open(DATA_FILE_PATH / "day09_input.txt", "r") as f:
        data = [int(line) for line in f.read().splitlines()]
    target = find_anomaly_in_data(data, 25)
    print(target)
    weakness = find_range_addends(data, target)
    print(weakness)