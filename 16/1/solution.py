#!/usr/bin/env python3
import sys
from collections import Counter

import numpy as np

def turn_left(vector):
    if not isinstance(vector, np.ndarray) or vector.shape != (2, 1):
        raise ValueError
    rotation_mat = np.array([[0, -1], [1, 0]])
    return rotation_mat @ vector


def turn_right(vector):
    if not isinstance(vector, np.ndarray) or vector.shape != (2, 1):
        raise ValueError
    rotation_mat = np.array([[0, 1], [-1, 0]])
    return rotation_mat @ vector


def read(path):
    with open(path, 'r') as f:
        line = f.readline()
    instructions = line.split(", ")
    return [(s[0], int(s[1:])) for s in instructions]


def follow_route(position, direction, instructions):
    positions = set()
    for turn, steps in instructions:
        if turn == "R":
            direction = turn_right(direction)
        elif turn == "L":
            direction = turn_left(direction)
        else:
            raise ValueError
        for _ in range(steps):
            position += direction
            if str(position) in positions:
                return position
            else:
                positions.add(str(position))
    return position


def main(filename):
    instructions = read(filename)

    start_position = np.array([[0], [0]])
    start_direction = np.array([[0], [1]])
    finish_position = follow_route(start_position, start_direction, instructions)

    print(np.sum(np.abs(finish_position.flatten())))


if __name__ == "__main__":
    main(sys.argv[1])
