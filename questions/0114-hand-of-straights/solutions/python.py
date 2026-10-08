from collections import Counter


def is_n_straight_hand(hand: list[int], group_size: int) -> bool:
    if len(hand) % group_size:
        return False
    count = Counter(hand)
    for x in sorted(count):
        c = count[x]
        if c == 0:
            continue
        for v in range(x, x + group_size):
            if count[v] < c:
                return False
            count[v] -= c
    return True
