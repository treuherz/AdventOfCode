#!/usr/bin/env python3

import sys
from hashlib import md5

def read(path):
    with open(path, 'r') as f:
        return f.read().strip()


def digests(door_id):
    index = 0
    while True:
        digest = md5((door_id + str(index)).encode()).hexdigest()
        if digest[:5] == '00000':
            yield digest
        index += 1


def main(filename):
    door_id = read(filename)
    gen = digests(door_id)
    print([next(gen)[5] for _ in range(8)])


if __name__ == "__main__":
    main(sys.argv[1])
