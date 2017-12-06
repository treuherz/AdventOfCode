#!/usr/bin/env python3

import sys

import numpy as np

def read(path):
    return np.loadtxt(path, dtype=int)


def checksum(array):
    return np.sum(np.amax(array, 1) - np.amin(array, 1))


def divisors(n):
    for i in range(2, n):
        if n % i == 0:
            yield i


def division_in_row(arr):
    for i in np.nditer(arr):
        for d in divisors(i):
            if d in arr:
                return int(i / d)


def division_checksum(arr):
    return np.sum(np.apply_along_axis(division_in_row, 1, arr))


def main(filename):
    spreadsheet = read(filename)
    print(checksum(spreadsheet))
    print(division_checksum(spreadsheet))


if __name__ == "__main__":
    main(sys.argv[1])
