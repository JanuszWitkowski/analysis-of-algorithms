#!/usr/bin/python3
from sys import argv
from typing import List, Tuple
from matplotlib import pyplot as plt
import numpy as np
import pandas as pd
# from delta import sort_ratios, calculate_delta_in_estimation

DIRNAME = "graphs/"
FILES_THEORY_N = ["../results/theory_n1.csv", 
                    "../results/theory_n3.csv", 
                    "../results/theory_n6.csv", 
                    "../results/theory_n12.csv", 
                    "../results/theory_n24.csv", 
                    "../results/theory_n48.csv"]
FILES_THEORY_P = ["../results/theory_p0.001.csv",
                  "../results/theory_p0.01.csv",
                  "../results/theory_p0.1.csv"]
FILES_SIM_N = ["../results/sim_n1.csv",
                    "../results/sim_n3.csv",
                    "../results/sim_n6.csv",
                    "../results/sim_n12.csv",
                    "../results/sim_n24.csv",
                    "../results/sim_n48.csv"]

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


def plotting_theory_n(filenames: List[str]):
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

def plotting_theory_p(filenames: List[str]):
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

def plotting_simulation(filenames: List[str]):
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

def plotting_both(filenames1: List[str], filenames2: List[str]):
    if len(filenames1) != len(filenames2):
        print("Error: Numbers of files in two groups are different!")
        return
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


if __name__ == "__main__":
    if len(argv) > 2:
        if argv[1] == "-t":
            # filenames = argv[2:]
            plotting_theory_n(argv[2:])
            
        elif argv[1] == "-s":
            # filenames = argv[2:]
            plotting_simulation(argv[2:])
            
        elif argv[1] == "-b":
            filenames = argv[2:]
            print(filenames)
            if len(filenames) % 2 != 0:
                print("Not even!")
            filenames1 = [filenames[2*i] for i in range(int(len(filenames)/2))]
            filenames2 = [filenames[2*i+1] for i in range(int(len(filenames)/2))]
            plotting_both(filenames1, filenames2)
            
        elif argv[1] == "-p":
            # filenames = argv[2:]
            plotting_theory_p(argv[2:])
            
        else:
            print("whatever man, please add at the beginning -t for theory, -s for simulation or -b for both... (or even -p for Prob test of n)")
    elif len(argv) == 2:
        if argv[1] == "-t":
            plotting_theory_n(FILES_THEORY_N)
            
        elif argv[1] == "-s":
            plotting_simulation(FILES_SIM_N)
            
        elif argv[1] == "-b":
            plotting_both(FILES_THEORY_N, FILES_SIM_N)
        
        elif argv[1] == "-p":
            plotting_theory_p(FILES_THEORY_P)
    else:
        print("else")

