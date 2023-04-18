#!/usr/bin/python3
from sys import argv
from typing import List, Tuple
from matplotlib import pyplot as plt
import numpy as np
import pandas as pd
from delta import sort_ratios, calculate_delta_in_estimation, calculate_delta_czybyszew, calculate_delta_chernoff

def proper_name (filename: str) -> str:
    name = filename[:-4]
    l = len(name)
    idx = 0
    while idx < l and name[-idx] != '/':
        idx += 1
    if idx == l:
        return name
    return name[-idx+1:]

def open_csv_for_numbers(filename: str) -> Tuple[List[int], List[float]]:
    with open(filename, 'r') as file:
        lines = [line.split(';') for line in file]
    ns = [int(line[0]) for line in lines]
    ratios = [float(line[1]) for line in lines]
    return ns, ratios

def plot_simple (title: str, ns: List[int], ratios: List[float]):
    if len(ns) == len(ratios):
        plt.figure(figsize=(16, 8))
        plt.title(title)
        plt.scatter(ns, ratios)
        plt.xlabel("Wielkość multizbioru [n]")
        plt.ylabel("estymata/n")
        plt.savefig(title + ".png")
        # plt.show()
        print(f"Done for {title}!")
    else:
        print("ERROR: Different sizes of ns and ratios.")

def plot_hashes (title: str, bs: List[int], hashes: List[Tuple[List[float], str]]):
    plt.figure(figsize=(16, 8))
    plt.title(title)
    for hash in hashes:
        plt.scatter(bs, hash[0], label=hash[1], marker=".")
    plt.legend()
    plt.xlabel("Liczba bajtów")
    plt.ylabel("Średnia różnica stosunku estymaty do rzeczywistej wartości od wartości oczekiwanej")
    plt.savefig(title + ".png")
    # plt.show()
    print(f"Done for {title}!")

def plot_deltas (title: str, ns: List[int], ratios: List[float], deltas: List[Tuple[float, str, str]]):
    ln = len(ns)
    if len(ns) == len(ratios):
        filename = title
        plt.figure(figsize=(16, 8))
        plt.title(title)
        plt.scatter(ns, ratios)
        for delta in deltas:
            plt.plot(ns, [(1.0 - delta[0]) for _ in range(ln)], label=delta[1], color=delta[2])
            plt.plot(ns, [(1.0 + delta[0]) for _ in range(ln)], color=delta[2])
            filename = filename + "_" + delta[1]
        plt.xlabel("Wielkość multizbioru [n]")
        plt.ylabel("estymata/n")
        plt.legend()
        plt.savefig(filename + ".png")
        # plt.show()
        print(f"Done for {filename}!")
    else:
        print("ERROR: Different sizes of ns and ratios.")


if __name__ == "__main__":
    if len(argv) > 1:
        filename = argv[1]
        if "6.csv" in filename:
            with open(filename, 'r') as file:
                lines = [line.split(';') for line in file]
            bs = [int(line[0]) for line in lines[1:]]
            # print(bs)
            names = [name for name in lines[0][1:]]
            # print(names)
            # print(len(names))
            # print(len(lines[1:]))
            hashes = [([float(lines[j+1][i+1]) for j in range(len(lines[1:]))], names[i]) for i in range(len(names))]
            # print(hashes)
            plot_hashes("hashes", bs, hashes)
        else:
            title = proper_name(filename)
            ns, ratios = open_csv_for_numbers(filename)
            plot_simple(title, ns, ratios)
    else:
        filename = "../results/5b_k400.csv"
        # alpha1 = 0.05
        # alpha2 = 0.01
        # alpha3 = 0.005
        alpha = 0.005
        k = 400
        # title = proper_name(filename)
        title = proper_name(filename) + "_alpha" + str(alpha)
        ns, ratios = open_csv_for_numbers(filename)
        sorted_ratios = sort_ratios([r for r in ratios])
        # delta1 = calculate_delta_in_estimation(sorted_ratios, alpha1)
        # delta2 = calculate_delta_in_estimation(sorted_ratios, alpha2)
        # delta3 = calculate_delta_in_estimation(sorted_ratios, alpha3)
        # deltas = [(delta1, "alpha-05", "red"),
        #             (delta2, "alpha-01", "green"),
        #             (delta3, "alpha-005", "black")]
        deltas = [(calculate_delta_in_estimation(sorted_ratios, alpha), "empirum", "red"),
                    (calculate_delta_czybyszew(k, alpha), "chebyschev", "green"),
                    (calculate_delta_chernoff(k, alpha), "chernoff", "blue")]
        plot_deltas(title, ns, ratios, deltas)



# import pandas as pd
# from matplotlib import pyplot as plt
# plt.rcParams["figure.figsize"] = [7.00, 3.50]
# plt.rcParams["figure.autolayout"] = True
# columns = ["Name", "Marks"]
# df = pd.read_csv("input.csv", usecols=columns)
# print("Contents in csv file:
# ", df)
# plt.plot(df.Name, df.Marks)
# plt.show()