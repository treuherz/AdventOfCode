#!/usr/bin/env python3

import sys

import numpy as np

def read(path):
    passphrases = []
    with open(path, 'r') as f:
        for line in f:
            passphrases.append(line.strip().split(' '))
    return passphrases


def sort_substrs(phrase):
    return [''.join(sorted(s)) for s in phrase]


def no_dupes(phrase):
    deduped = set(phrase)
    return len(deduped) == len(phrase)


def main(filename):
    passphrases = read(filename)
    print(sum(no_dupes(p) for p in passphrases))
    print(sum(no_dupes(sort_substrs(p)) for p in passphrases))


if __name__ == "__main__":
    main(sys.argv[1])
