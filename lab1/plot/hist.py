#!/usr/bin/python3
import matplotlib.pyplot as plt
# import argparse
import sys
import math
from data import proper_name, open_numbers

def histogram (filename):
    numbers = open_numbers(filename)
    name = proper_name(filename)
    plt.hist(numbers)
    # Scale
    # scale_list = range(math.floor(min(numbers)), math.ceil(max(numbers))+1)
    # plt.xticks(scale_list)
    # Labels
    plt.title(name)
    plt.xlabel("Rozkład zmiennej losowej liczby slotów do wyboru lidera")
    plt.ylabel("Częstotliwość występowania")
    # Save and show
    plt.savefig(name + '.png')
    plt.show()

if __name__ == "__main__":
    args = sys.argv
    if len(args) > 1:
        filename = args[1]
        # print(proper_name(filename))
        histogram(filename)


# with open(filename) as file:
#     lines = [line.rstrip() for line in file]

# if __name__ == "__main__":
    # parser = argparse.ArgumentParser(description='Data processing')
    # parser.add_argument('filename')
    # parser.add_argument('-h', '--histogram', dest='histogram', type=str, help='File to process into histogram')
    # parser.add_argument('--surname', dest='surname', type=str, help='Surname of the candidate')
    # parser.add_argument('--age', dest='age', type=int, help='Age of the candidate')
    # args = parser.parse_args()
    # print(args)


# parser.add_argument('filename')           # positional argument
# parser.add_argument('-c', '--count')      # option that takes a value
# parser.add_argument('-v', '--verbose',
#                     action='store_true')  # on/off flag

