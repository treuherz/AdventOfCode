#!/usr/bin/env python3

import sys

import numpy as np

def read(path):
    with open(path, 'r') as f:
        return [int(l.strip()) for l in f]


def jump_around(jumps, inc=lambda x: x + 1):
    jumps = list(jumps)
    cursor = 0
    count = 0
    while 0 <= cursor < len(jumps):
        jump = jumps[cursor]
        jumps[cursor] = inc(jump)
        cursor += jump
        count += 1
    return(count)

def weird_inc(n):
    return n + (-1 if n >= 3 else 1)


def main(filename):
    jumps = read(filename)
    print(jump_around(jumps))
    print(jump_around(jumps, weird_inc))


if __name__ == "__main__":
    main(sys.argv[1])
