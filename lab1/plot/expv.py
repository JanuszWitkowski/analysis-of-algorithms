#!/usr/bin/python3
import sys
from data import proper_name, open_numbers

def expected_value (numbers):
    n = len(numbers)
    sum = 0
    for num in numbers:
        sum += num
    return 1.0 * num / n

def variance (numbers, mean):
    n = len(numbers)
    sum = 0
    for num in numbers:
        sum += (num - mean) ** 2.0
    return sum / n

if __name__ == "__main__":
    args = sys.argv
    if len(args) > 1:
        filename = args[1]
        numbers = open_numbers(filename)
        ev = expected_value(numbers)
        var = variance(numbers, ev)
        print(f"{len(numbers)}: {ev}; {var}")
