#!/usr/bin/env python3

import sys
import re
from collections import namedtuple, Counter

import numpy as np
import networkx as nx

Disc = namedtuple('Disc', ['name', 'weight', 'children'])

def read(path):
    discs = []
    node_pattern = re.compile(r'(\w+) \((\d+)\)')
    child_pattern = re.compile(r'(\w+)(?=, |$)')
    with open(path, 'r') as f:
        for line in f:
            node = re.match(node_pattern, line)
            children = re.findall(child_pattern, line)
            discs.append(Disc(node.group(1), int(node.group(2)), children))
    return discs


def construct_tree(discs):
    tree = nx.DiGraph()
    for d in discs:
        tree.add_node(d.name, w=d.weight)
    for d in discs:
        for c in d.children:
            tree.add_edge(d.name, c)
    return tree


def sum_weights(tree, node):
    weight = tree.node[node]['w']
    if 's' in tree.node[node]:
        s = tree.node[node]['s']
    else:
        s = sum(sum_weights(tree, n) for n in tree.successors(node))
        tree.node[node]['s'] = s
    return weight + s


def validate_node(tree, node, expected=0):
    child_weights = {c: sum_weights(tree, c) for c in tree.successors(node)}
    weight_counter = Counter(child_weights.values())
    if len(weight_counter) > 1:
        majority = weight_counter.most_common()[0][0]
        minority = weight_counter.most_common()[-1][0]
        culprit = [n for n, w in child_weights.items() if w == minority][0]
        return validate_node(tree, culprit, expected=majority)
    else:
        return expected - tree.node[node]['s']


def main(filename):
    discs = read(filename)
    tree = construct_tree(discs)
    root = next(nx.topological_sort(tree))
    print(root)
    sum_weights(tree, root)
    print(validate_node(tree, root))


if __name__ == "__main__":
    main(sys.argv[1])
