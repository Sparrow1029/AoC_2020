#!/usr/bin/env python3
actions = {
    "B": lambda x, y: ((y-x)//2 + x+1, y),
    "F": lambda x, y: (x, x + (y-x)//2),
    "R": lambda x, y: ((y-x)//2 + x+1, y),
    "L": lambda x, y: (x, x + (y-x)//2),
}

def get_seat_id(info: str) -> int:
    row = (0, 127)
    col = (0, 7)
    for r in info[:7]:
        row = actions[r](*row)
    for c in info[7:]:
        col = actions[c](*col)
    seat_id = row[0]*8 + col[0]
    # print(row, col, max(col), seat_id)
    return seat_id


with open("input.txt", "r") as f:
    data = f.read().splitlines()

highest_id = 0
for line in data:
    seat_id = get_seat_id(line)
    if seat_id > highest_id:
        highest_id = seat_id

all_ids = set(range(8, highest_id))
plane_ids = set([ get_seat_id(seat) for seat in data ])

print("PART ONE: ", highest_id)

print("PART TWO: ", all_ids.difference(plane_ids))
