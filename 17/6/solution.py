#!/usr/bin/env python3

import sys

import numpy as np

def read(path):
    with open(path, 'r') as f:
        return [int(i) for i in f.read().strip().split()]


def main(filename):
    memory = read(filename)
    history = []
    count = 0
    while memory not in history:
        history.append(list(memory))
        top = max(memory)
        cursor = memory.index(top)
        memory[cursor] = 0
        for i in range(top):
            cursor = (cursor + 1) % len(memory)
            memory[cursor] += 1
        count += 1
    print(count)
    print(len(history) - history.index(memory))


if __name__ == "__main__":
    main(sys.argv[1])
