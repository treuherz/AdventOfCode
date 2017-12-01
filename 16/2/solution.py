#!/usr/bin/env python3

import sys

import numpy as np

def read(path):
    with open(path, 'r') as f:
        l = f.readlines()
    return [s.strip() for s in l]


def move_key(position, direction):
    if direction == "U":
        move = np.array([-1, 0])
    elif direction == "R":
        move = np.array([0, 1])
    elif direction == "D":
        move = np.array([1, 0])
    elif direction == "L":
        move = np.array([0, -1])
    else:
        raise ValueError

    return position + move


def list_keys(keypad, start, instructions):
    position = start
    for l in instructions:
        for d in l:
            new_position = move_key(position, d)
            try:
                if any(new_position < 0):
                    raise IndexError
                if keypad[tuple(new_position)] != '0':
                    position = new_position
            except IndexError:
                pass
        print(keypad[tuple(position)], end='')
    print()
            

def main(filename):
    instructions = read(filename)
    keypad_a = np.arange(1,10).reshape(3,3)
    start_a = np.array([1, 1])
    list_keys(keypad_a, start_a, instructions)

    keypad_b = np.array(
        [['0', '0', '1', '0', '0'],
         ['0', '2', '3', '4', '0'],
         ['5', '6', '7', '8', '9'],
         ['0', 'A', 'B', 'C', '0'],
         ['0', '0', 'D', '0', '0']]
    )
    start_b = np.array([2,0])
    list_keys(keypad_b, start_b, instructions)


if __name__ == "__main__":
    main(sys.argv[1])
