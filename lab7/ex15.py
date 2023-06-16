#!/usr/bin/python3
from sys import argv

counter = 0

def f(n: int) -> int:
    s = 0
    if n == 0:
        return 1
    else:
        for i in range(n):
            s += f(i)   # line 6
            # Counting
            global counter
            counter += 1
        return s
    

if __name__ == "__main__":
    n_range = 16
    if len(argv) > 1:
        n_range = int(argv[1])
    ns = range(n_range+1)
    for n in ns:
        s_value = f(n)
        c_value = counter
        counter = 0
        print(f"n = {n}: ctr = {c_value}; s = {s_value}")

