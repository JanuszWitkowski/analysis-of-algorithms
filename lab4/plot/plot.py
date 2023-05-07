#!/usr/bin/python3
from sys import argv
from typing import List, Tuple
from matplotlib import pyplot as plt
import numpy as np
import pandas as pd
# from delta import sort_ratios, calculate_delta_in_estimation

DIRNAME = "graphs/"

def proper_name (filename: str) -> str:
    name = filename[:-4]
    l = len(name)
    idx = 0
    while idx < l and name[-idx] != '/':
        idx += 1
    if idx == l:
        return name
    return name[-idx+1:]

def open_csv_theory(filename: str) -> Tuple[List[float], List[float], List[float]]:
    with open(filename, 'r') as file:
        lines = [line.split(';') for line in file]
    qs = [float(line[0]) for line in lines]
    nakas = [float(line[1]) for line in lines]
    gruns = [float(line[2]) for line in lines]
    return qs, nakas, gruns

def open_csv_simulation(filename: str) -> Tuple[List[float], List[float]]:
    with open(filename, 'r') as file:
        lines = [line.split(';') for line in file]
    qs = [float(line[0]) for line in lines]
    results = [float(line[1]) for line in lines]
    return qs, results

def plot_simple (titles: List[str], qss: List[List[float]], numbers: List[List[List[float]]], labels: List[str], xlabel: str, ylabel: str):
    if len(qss) != len(numbers) or len(qss) != len(titles) or len(numbers) != len(titles) or len(numbers) == 0 or len(numbers[0]) != len(labels):
        print("Error while simple-plotting!")
        return
    for i in range(len(numbers)):
        plt.figure(figsize=(16, 8))
        plt.title(titles[i])
        for j in range(len(numbers[i])):
            plt.plot(qss[i], numbers[i][j], label=labels[j])
        plt.xlabel(xlabel)
        plt.ylabel(ylabel)
        plt.legend()
        plt.savefig(DIRNAME + titles[i] + ".png")
        # plt.show()
        print(f"Done for {titles[i]}!")

# def plot_simple_multiple(title: str, filenames: List[str]):
#     plt.figure(figsize=(16, 8))
#     plt.title(title)

#     for filename in filenames:
#         ns, ests, ratios = open_csv_for_numbers(filename)
#         plt.scatter(ns, ratios, label=proper_name(filename), marker=".")

#     plt.xlabel("Wielkość multizbioru [n]")
#     plt.ylabel("estymata/n")
#     plt.legend()
#     plt.savefig(title + ".png")
#     print(f"Done for {title}!")

# def plot_hashes (title: str, bs: List[int], hashes: List[Tuple[List[float], str]]):
#     plt.figure(figsize=(16, 8))
#     plt.title(title)
#     for hash in hashes:
#         plt.scatter(bs, hash[0], label=hash[1], marker=".")
#     plt.legend()
#     plt.xlabel("Liczba bajtów")
#     plt.ylabel("Średnia różnica stosunku estymaty do rzeczywistej wartości od wartości oczekiwanej")
#     plt.savefig(title + ".png")
#     # plt.show()
#     print(f"Done for {title}!")


if __name__ == "__main__":
    if len(argv) > 2:
        if argv[1] == "-t":
            filenames = argv[2:]
            print(filenames)
            titles = []
            qss = []
            numbers = []
            labels = ["Nakamuto", "Grunspan"]
            for filename in filenames:
                title = proper_name(filename)
                qs, nakas, gruns = open_csv_theory(filename)
                titles.append(title)
                qss.append(qs)
                numbers.append([nakas, gruns])
            plot_simple(titles, qss, numbers, labels, "Szanse na wyprodukowanie bloku", "Szanse powodzenia adwersarza")
        elif argv[1] == "-s":
            filenames = argv[2:]
            print(filenames)
            titles = []
            qss = []
            numbers = []
            labels = ["Empirum"]
            for filename in filenames:
                title = proper_name(filename)
                qs, nums = open_csv_simulation(filename)
                titles.append(title)
                qss.append(qs)
                numbers.append([nums])
            plot_simple(titles, qss, numbers, labels, "Szanse na wyprodukowanie bloku", "Szanse powodzenia adwersarza")
        elif argv[1] == "-b":
            filenames = argv[2:]
            print(filenames)
            if len(filenames) % 2 != 0:
                print("Not even!")
            filenames1 = [filenames[2*i] for i in range(int(len(filenames)/2))]
            filenames2 = [filenames[2*i+1] for i in range(int(len(filenames)/2))]
            titles = []
            qss = []
            numbers = []
            labels = ["Nakamuto", "Grunspan", "Empirum"]
            for i in range(len(filenames1)):
                filename1 = filenames1[i]
                filename2 = filenames2[i]
                title = "both_" + proper_name(filename1) + "_" + proper_name(filename2)
                qs1, nakas, gruns = open_csv_theory(filename1)
                qs2, nums = open_csv_simulation(filename2)
                titles.append(title)
                qss.append(qs1)
                numbers.append([nakas, gruns, nums])
            plot_simple(titles, qss, numbers, labels, "Szanse na wyprodukowanie bloku", "Szanse powodzenia adwersarza")
        elif argv[1] == "-p":
            filenames = argv[2:]
            print(filenames)
            titles = []
            titles_stripped = []
            qss = []
            qss_stripped = []
            numbers = []
            numbers_stripped = []
            labels = ["Nakamuto", "Grunspan"]
            for filename in filenames:
                title = proper_name(filename)
                qs, nakas, gruns = open_csv_theory(filename)
                titles.append(title)
                titles_stripped.append(title + "_stripped")
                qss.append(qs)
                qss_stripped.append(qs[:-8])
                numbers.append([nakas, gruns])
                numbers_stripped.append([nakas[:-8], gruns[:-8]])
            plot_simple(titles, qss, numbers, labels, "Szanse na wyprodukowanie bloku", "Liczba wymaganych potwierdzeń")
            plot_simple(titles_stripped, qss_stripped, numbers_stripped, labels, "Szanse na wyprodukowanie bloku", "Liczba wymaganych potwierdzeń")
        else:
            print("whatever man, please add at the beginning -t for theory, -s for simulation or -b for both... (or even -p for Prob test of n)")
    else:
        print("else")

