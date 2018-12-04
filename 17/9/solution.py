#!/usr/bin/env python3

import sys
import re
from collections import namedtuple, defaultdict
import operator as op

Instruction = namedtuple('Instruction', ['tgt', 'op', 'n', 'chk', 'cmp', 'th'])

def read(path):
    pattern = re.compile(r"^(\w+) (\w+) (-?\d+) if (\w+) ([<>=!]+) (-?\d+)$")
    ops = {'inc': op.add, 'dec': op.sub}
    cmps = {
        '==': op.eq,
        '!=': op.ne,
        '<': op.lt,
        '<=': op.le,
        '>': op.gt,
        '>=': op.ge
    }
    instructions = []
    with open(path, 'r') as f:
        for line in f:
            m = re.match(pattern, line)
            m = Instruction(*m.group(1, 2, 3, 4, 5, 6))
            parsed = Instruction(
                m.tgt, 
                ops[m.op],
                int(m.n),
                m.chk,
                cmps[m.cmp],
                int(m.th)
            )
            instructions.append(parsed)
    return instructions


def execute(d, i):
    if i.cmp(d[i.chk], i.th):
        d[i.tgt] = i.op(d[i.tgt], i.n)


def main(filename):
    instructions = read(filename)
    registers = defaultdict(lambda: 0)
    top = 0
    for l in instructions:
        execute(registers, l)
        cur_top = registers[max(registers, key=lambda k: registers[k])]
        if cur_top > top:
            top = cur_top
    print(cur_top)
    print(top)


if __name__ == "__main__":
    main(sys.argv[1])
