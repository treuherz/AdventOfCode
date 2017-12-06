#!/usr/bin/env python3

import sys
from enum import Enum

import numpy as np
import attr
import tqdm


def read(path):
    with open(path, 'r') as f:
        return int(f.read().strip())


@attr.s
class Spiral():
    arr = attr.ib(default=np.array(1, ndmin=2))

    Dir = Enum('Dir', 'R U L D', start=0)

    def _neighbourhood(self, index):
        l = [list(index) for _ in range(8)]
        # E
        l[0][1] = l[0][1] + 1
        # N
        l[1][0] = l[1][0] - 1
        # W
        l[2][1] = l[2][1] - 1
        # S
        l[3][0] = l[3][0] + 1
        # NE
        l[4][0] = l[4][0] - 1
        l[4][1] = l[4][1] + 1
        # NW
        l[5][0] = l[5][0] - 1
        l[5][1] = l[5][1] - 1
        # SW
        l[6][0] = l[6][0] + 1
        l[6][1] = l[6][1] - 1
        # SE
        l[7][0] = l[7][0] + 1
        l[7][1] = l[7][1] + 1
        acc = 0
        for i in l:
            try:
                acc += self.arr[tuple(i)]
            except IndexError:
                pass
        return acc

    @classmethod
    def _directions(cls):
        index = 0
        d = cls.Dir.R
        while True:
            index += 1
            steps = int(np.ceil(index / 2))
            for _ in range(steps):
                yield d
            d = cls.Dir((d.value + 1) % 4)
        
    @classmethod
    def _movecursor(cls, cursor, direction):
        new_cursor = list(cursor)
        if direction == cls.Dir.R:
            new_cursor[1] += 1
        elif direction == cls.Dir.U:
            new_cursor[0] -= 1
        elif direction == cls.Dir.L:
            new_cursor[1] -= 1
        elif direction == cls.Dir.D:
            new_cursor[0] += 1
        return tuple(new_cursor)


    @classmethod
    def until(cls, n, neighbours=False):
        size = int(np.ceil(np.sqrt(n)))
        blank = np.zeros((size, size), dtype=int)
        spiral = cls(blank)
        cursor = (0, 0)
        spiral.arr[cursor] = 1
        directions = spiral._directions()
        for i, d in enumerate(directions):
            cursor = spiral._movecursor(cursor, d)
            if not neighbours:
                spiral.arr[cursor] = i + 2
            else:
                spiral.arr[cursor] = spiral._neighbourhood(cursor)
            if spiral.arr[cursor] >= n:
                break
        return spiral

    def max(self):
        return np.max(self.arr)

    def _remap(self, index):
        coord = list(index)
        for d, c in enumerate(index): 
            if c > self.arr.shape[d] // 2:
                coord[d] = c - self.arr.shape[d] 
        return tuple(coord)

    def argmax(self):
        index = np.where(self.arr == self.max())
        index = (index[0][0], index[1][0])
        index = self._remap(index)
        return index

    def dist(self, start=None, end=None):
        if start is None:
            start_idx = self.argmax()
        if end is None:
            end_idx = (0, 0)
        return abs((end_idx[0] - start_idx[0]) + (end_idx[1] - start_idx[1]))


def main(filename):
    index = read(filename)
    spiral = Spiral.until(index)
    print(spiral.dist())
    spiral2 = spiral.until(index, neighbours=True)
    print(spiral2.max())


if __name__ == "__main__":
    main(sys.argv[1])


