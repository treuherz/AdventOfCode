#!/usr/bin/env python3

import sys
from collections import namedtuple, Counter
from operator import itemgetter

import attr
import numpy as np


@attr.s
class Room():
    name = attr.ib(type=str)
    sector = attr.ib(type=int)
    checksum = attr.ib(type=str)

    ALPHABET = 'abcdefghijklmnopqrstuvwxyz'

    @classmethod
    def from_str(cls, string):
        string = string.strip()
        *name, sector_checksum = string.split('-')
        name = '-'.join(name)
        sector, _, checksum = sector_checksum.partition('[')
        checksum = checksum.rstrip(']')
        return cls(name, int(sector), checksum)

    def is_valid(self):
        c = Counter(self.name)
        del c['-']
        c = c.most_common()
        c = sorted(c, key=itemgetter(0))
        c = sorted(c, key=itemgetter(1), reverse=True)
        c = c[:5]
        c = [t[0] for t in c]
        c = ''.join(c)
        return c == self.checksum

    @classmethod
    def rot(cls, letter, n):
        if letter == '-':
            return '-'
        index = (cls.ALPHABET.find(letter) + n) % 26
        return cls.ALPHABET[index]


    def decrypt(self):
        return ''.join(self.rot(l, self.sector) for l in self.name)


def read(path):
    with open(path, 'r') as f:
        return [Room.from_str(l) for l in f.readlines()]


def main(filename):
    rooms = read(filename)
    print(sum(r.sector if r.is_valid() else 0 for r in rooms))
    for r in rooms:
        string = r.decrypt()
        if string.find('north') >= 0:
            print(r.decrypt(), r.sector)


if __name__ == "__main__":
    main(sys.argv[1])
