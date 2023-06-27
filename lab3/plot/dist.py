#!/usr/bin/python3
from matplotlib import pyplot as plt
from sys import argv

def proper_name (filename: str) -> str:
    name = filename[:-4]
    l = len(name)
    idx = 0
    while idx < l and name[-idx] != '/':
        idx += 1
    if idx == l:
        return name
    return name[-idx+1:]


if __name__ == "__main__":
    if len(argv) >= 2:
        filename = argv[1]
        with open(filename) as f:
            lines = [line.split() for line in f]
        # print(lines)
        numbers = [int(line[0]) for line in lines]
        print(numbers)
        plt.hist(numbers)
        plt.savefig("plot/dist/" + proper_name(filename) + ".png")

