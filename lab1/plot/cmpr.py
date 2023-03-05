#!/usr/bin/python3
from numpy import *
import math
import sys
import matplotlib.pyplot as plt
from data import proper_name

def simple_compare (filename, name):
    with open(filename) as file:
        data = [line.rstrip().split() for line in file]
    # print(data)
    ns = [arr[0] for arr in data]
    # ns.insert(0, 0)
    values = [arr[1] for arr in data]
    # values.insert(0, 0.0)
    # efunc = [math.e for _ in ns]
    # zerofunc = [0.0 for _ in ns]
    plt.figure().set_figwidth(18)
    # plt.plot(ns, zerofunc, 'b')
    # plt.plot(ns, efunc, 'r')
    plt.plot(ns, values, 'g')
    # plt.ylim(bottom=0)
    plt.title(name)
    # plt.xlabel("Rozkład zmiennej losowej liczby slotów do wyboru lidera")
    # plt.ylabel("Częstotliwość występowania")
    # plt.savefig(name + '.png')
    plt.show()

if __name__ == "__main__":
    args = sys.argv
    if len(args) > 1:
        filename = args[1]
        name = proper_name(filename)
        if name[0] == 'e':
            print("Excpected Value")
            simple_compare(filename, name)
        elif name[0] == 'v':
            print("Variance")
            simple_compare(filename, name)
        elif name[0] == 'l':
            print("Lambda")
