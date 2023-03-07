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
    efunc = [math.e for _ in ns]
    zerofunc = [0.0 for _ in ns]
    plt.figure().set_figwidth(18)
    # plt.plot(ns, zerofunc)
    plt.plot(ns, efunc)
    plt.plot(ns, values)
    plt.ylim(bottom=0.0)
    plt.title(name)
    # plt.xlabel("Rozkład zmiennej losowej liczby slotów do wyboru lidera")
    # plt.ylabel("Częstotliwość występowania")
    # plt.savefig(name + '.png')
    plt.show()

def lambda_compare (filename, name):
    with open(filename) as file:
        data = [line.rstrip().split() for line in file][1:]
    # print(data)
    us = [int(arr[0][:-1]) for arr in data]
    ls = [int(arr[1][:-1]) for arr in data]
    lambda_value = [0.579 for _ in us]
    ntwo = [float(arr[2]) for arr in data]
    n10 = [float(arr[3]) for arr in data]
    n25 = [float(arr[4]) for arr in data]
    n50 = [float(arr[5]) for arr in data]
    n75 = [float(arr[6]) for arr in data]
    n90 = [float(arr[7]) for arr in data]
    nu = [float(arr[8]) for arr in data]
    results = [arr[2:] for arr in data]

    plt.figure(figsize=(16, 8))
    plt.plot(us, lambda_value, label='lambda')
    plt.plot(us, ntwo, label='n = 2')
    plt.plot(us, n10, label='n = 0.1u')
    plt.plot(us, n25, label='n = .25u')
    plt.plot(us, n50, label='n = 0.5u')
    plt.plot(us, n75, label='n = 0.75u')
    plt.plot(us, n90, label='n = 0.9u')
    plt.plot(us, nu, label='n = u')
    plt.ylim(bottom=0.45)
    plt.ylim(top=0.9)
    plt.title(name)
    # plt.legend(loc='upper center', bbox_to_anchor=(0.5, 1.05), ncol=3)
    plt.legend(loc='lower center', ncol=3)
    plt.savefig(name + '.png')
    plt.show()

    # plt.clf()
    # for arr in results:
    #     plt.plot(arr)
    # plt.plot(lambda_value[:6])
    # plt.show()

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
            lambda_compare(filename, name)
