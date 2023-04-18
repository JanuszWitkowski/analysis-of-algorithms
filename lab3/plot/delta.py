#!/usr/bin/python3
from sys import argv
from typing import List, Tuple
# import numpy.nextafter
from math import sqrt, exp, inf

def open_csv_for_numbers(filename: str) -> Tuple[List[int], List[float], List[float]]:
    with open(filename, 'r') as file:
        lines = [line.split(';') for line in file]
    ns = [int(line[0]) for line in lines]
    ests = [float(line[1]) for line in lines]
    ratios = [float(line[2]) for line in lines]
    return ns, ests, ratios

def sort_ratios (ratios: List[float]) -> List[float]:
    return sorted(ratios, key=lambda x:x)

def calculate_expected_value(ratios: List[float]) -> float:
    n = len(ratios)
    sum = 0
    for r in ratios:
        sum += r
    return 1.0 * sum / n

def calculate_variance (ratios: List[float], ev: float) -> float:
    sum = 0.0
    for r in ratios:
        sum += (r - ev)**2
    return sum / len(ratios)

def calculate_delta_czybyszew (k: int, alpha: float) -> float:
    return sqrt(1 / (k * alpha))

def chernoff_help (delta: float, k: int):
    fk = lambda arg: exp(arg * k) * ((1.0 - arg)**k)
    eps1 = delta / (1.0 - delta)
    eps2 = delta / (1.0 + delta)
    return fk(eps2) + fk(-eps1)

# def calculate_delta_chernoff (k: int, alpha: float) -> float:
#     left = 0.0
#     right = 1.0
#     while left <= right:
#         mid = (left + right) / 2.0
#         res = chernoff_help(mid, k)
#         if res <= alpha:
#             right = nextafter(mid, 0.0)
#         else:
#             left = nextafter(mid, inf)
#     return left

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
        ns, ests, ratios = open_csv_for_numbers(filename)

        ev = calculate_expected_value(ratios)
        var = calculate_variance(ratios, ev)

        sorted_ratios = sort_ratios(ratios)

        delta1 = calculate_delta_in_estimation(sorted_ratios, alpha1)
        delta2 = calculate_delta_in_estimation(sorted_ratios, alpha2)
        delta3 = calculate_delta_in_estimation(sorted_ratios, alpha3)

        delta_czybyszew_1 = calculate_delta_czybyszew(k, alpha1)
        delta_czybyszew_2 = calculate_delta_czybyszew(k, alpha2)
        delta_czybyszew_3 = calculate_delta_czybyszew(k, alpha3)

        # delta_chernoff1 = calculate_delta_chernoff(k, alpha1)
        # delta_chernoff2 = calculate_delta_chernoff(k, alpha2)
        # delta_chernoff3 = calculate_delta_chernoff(k, alpha3)

        print(f"EV = {ev}")
        print(f"Var = {var}")
        print(f"alpha = {alpha1} | delta = {delta1}")
        print(f"alpha = {alpha1} | czybyszew = {delta_czybyszew_1}")
        # print(f"alpha = {alpha1} | czernoff = {delta_chernoff1}")
        print(f"alpha = {alpha2} | delta = {delta2}")
        print(f"alpha = {alpha2} | czybyszew = {delta_czybyszew_2}")
        # print(f"alpha = {alpha2} | czernoff = {delta_chernoff2}")
        print(f"alpha = {alpha3} | delta = {delta3}")
        print(f"alpha = {alpha3} | czybyszew = {delta_czybyszew_3}")
        # print(f"alpha = {alpha3} | czernoff = {delta_chernoff3}")
