#!/usr/bin/env python3

import sys

import numpy as np

def read(path):
    return np.loadtxt(path, dtype=int)


def count_valid_triangles(array):
    array = np.sort(array)
    return np.sum(array[:, 0] + array[:, 1] > array[:, 2])


def group_vertically(array):
    for i in range(0, array.shape[0], 3):
        array[i:i+3] = array[i:i+3].T
    return array


def main(filename):
    triangles = read(filename)
    print(count_valid_triangles(triangles))
    triangles = read(filename)
    print(count_valid_triangles(group_vertically(triangles)))


if __name__ == "__main__":
    main(sys.argv[1])
