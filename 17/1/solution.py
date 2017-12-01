#!/usr/bin/env python3

import sys

import numpy as np

def read(path):
    with open(path, 'r') as f:
        line = f.read().strip()
    return [int(i) for i in line]


def check_captcha(captcha, offset=1):
    acc = 0
    for index, digit in enumerate(captcha):
        if digit == captcha[index-offset]:
            acc += captcha[index-offset]
    return acc


def main(filename):
    captcha = read(filename)
    print(check_captcha(captcha))
    print(check_captcha(captcha, len(captcha)//2))


if __name__ == "__main__":
    main(sys.argv[1])
