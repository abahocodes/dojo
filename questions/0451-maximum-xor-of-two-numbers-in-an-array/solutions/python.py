BITS = 31  # every value is below 2^31


def find_maximum_xor(nums: list[int]) -> int:
    # Binary trie in flat arrays: child[2 * node + bit] is the child index, 0 = none.
    child = [0] * (2 * (len(nums) * BITS + 1))
    size = 1
    best = 0
    for idx, x in enumerate(nums):
        # Insert x.
        node = 0
        for b in range(BITS - 1, -1, -1):
            slot = 2 * node + ((x >> b) & 1)
            if child[slot] == 0:
                child[slot] = size
                size += 1
            node = child[slot]
        # Query: walk toward the opposite bit wherever possible.
        node = 0
        cur = 0
        for b in range(BITS - 1, -1, -1):
            bit = (x >> b) & 1
            want = child[2 * node + (bit ^ 1)]
            if want:
                cur |= 1 << b
                node = want
            else:
                node = child[2 * node + bit]
        best = max(best, cur)
    return best
