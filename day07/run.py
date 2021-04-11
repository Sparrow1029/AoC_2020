#!/usr/bin/env python3
import re
import sys
from pathlib import Path
from collections import namedtuple
from typing import List, Dict
from time import sleep
from pprint import pprint

sys.path.append(str(Path(__file__).absolute().parent.parent))
from helpers.python_helpers import DATA_FILE_PATH

Bag = namedtuple("Bag", "name contains")


class Collection:

    string_bag_rgx =re.compile(r"\sbags?[,.]\s?")

    def __init__(self, bag_strings: str):
        self.bags_list: List[Bag] = [self.get_bag_from_str(bag_str) for bag_str in bag_strings]
        self.bags_dict: Dict[str, Bag] = {bag.name: bag for bag in self.bags_list}
        self.bags_contains_counts = {bag.name: sum(val for val in bag.contains.values()) for bag in self.bags_list}
        self.recursive_total = 0
        self.search_set = set()

    def get_bag_from_str(self, string) -> Bag:
        bag_name, others = string.split(" bags contain ")
        if others.startswith("no"):
            contains = dict()
        else:
            others_strings = re.sub(self.string_bag_rgx, ":", others).rstrip(":").split(":")
            contains = {s[2:]: int(s[:2]) for s in others_strings}
        return Bag(bag_name, contains)

    def find_bags_that_can_contain_bag_of_type(self, search_type: str):
        can_contain_type = {bag.name for bag in self.bags_list if search_type in bag.contains}
        if not can_contain_type:
            return
        self.search_set.update(can_contain_type)
        for bag_name in can_contain_type:
            self.find_bags_that_can_contain_bag_of_type(bag_name)

    def recursive_total_for_bag(self, bag_name):
        count = 0
        top_level_bag = self.bags_dict[bag_name]
        if not top_level_bag.contains:
            return count
        else:
            for current_bag in top_level_bag.contains:
                current_bags_amt = top_level_bag.contains[current_bag]
                count += current_bags_amt
                nested_bags = self.recursive_total_for_bag(current_bag)
                count += nested_bags * current_bags_amt
        return count

    def __repr__(self):
        return '\n\n'.join(bag.__repr__() for bag in self.bags_list)


if __name__ == "__main__":
    with open(DATA_FILE_PATH / "day07_input.txt", "r") as f:
        raw_bag_strings = f.readlines()

    all_bags = Collection(raw_bag_strings)
    print("PART 1: How many bags can contain at least one 'shiny gold' bag?")
    all_bags.find_bags_that_can_contain_bag_of_type("shiny gold")
    print(len(all_bags.search_set))

    print("PART 2: How many nested bags are contained in a 'shiny gold' bag?")
    print(all_bags.recursive_total_for_bag("shiny gold"))
