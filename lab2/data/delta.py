#!/usr/bin/python3
from sys import argv
from typing import List
from math import sqrt

def open_csv_for_numbers(filename: str) -> (List[int], List[float]):
    with open(filename, 'r') as file:
        lines = [line.split(';') for line in file]
    ns = [int(line[0]) for line in lines]
    ratios = [float(line[1]) for line in lines]
    return ns, ratios

def sort_ratios (ratios: List[float]) -> List[float]:
    return sorted(ratios, key=lambda x:x)

def calculate_delta_czybyszew (k: int, alpha: float) -> float:
    return sqrt(1 / (k * alpha))

def calculate_variance (ratios: List[float], ev: float) -> float:
    sum = 0.0
    for r in ratios:
        sum += (r - ev)**2
    return sum / len(ratios)

def calculate_delta_in_estimation (sorted_ratios: List[float], alpha: float) -> float:
    ln = len(sorted_ratios)
    window = int((1.0-alpha)*ln)
    delta = 100.0
    delta_candidate = -100.0
    left = 0
    right = window - 1
    while right < ln:
        delta_candidate = max(abs(1-sorted_ratios[left]), abs(sorted_ratios[right]-1))
        delta = min(delta, delta_candidate)
        left += 1
        right += 1
    return delta


if __name__ == "__main__":
    alpha1 = 0.05
    alpha2 = 0.01
    alpha3 = 0.005
    k = 400

    if len(argv) > 1:
        filename = argv[1]
        print(f"Given file: {filename}")
        ns, ratios = open_csv_for_numbers(filename)
        sorted_ratios = sort_ratios(ratios)

        delta1 = calculate_delta_in_estimation(sorted_ratios, alpha1)
        delta2 = calculate_delta_in_estimation(sorted_ratios, alpha2)
        delta3 = calculate_delta_in_estimation(sorted_ratios, alpha3)
        print(f"alpha = {alpha1} | delta = {delta1}")
        print(f"alpha = {alpha2} | delta = {delta2}")
        print(f"alpha = {alpha3} | delta = {delta3}")

        delta_czybyszew_1 = calculate_delta_czybyszew(k, alpha1)
        delta_czybyszew_2 = calculate_delta_czybyszew(k, alpha2)
        delta_czybyszew_3 = calculate_delta_czybyszew(k, alpha3)
        print(f"alpha = {alpha1} | czybyszew = {delta_czybyszew_1}")
        print(f"alpha = {alpha2} | czybyszew = {delta_czybyszew_2}")
        print(f"alpha = {alpha3} | czybyszew = {delta_czybyszew_3}")
