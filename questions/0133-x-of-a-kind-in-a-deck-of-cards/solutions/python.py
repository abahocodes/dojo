from collections import Counter
from math import gcd


def has_groups_size_x(deck: list[int]) -> bool:
    g = 0
    for c in Counter(deck).values():
        g = gcd(g, c)
    return g >= 2
