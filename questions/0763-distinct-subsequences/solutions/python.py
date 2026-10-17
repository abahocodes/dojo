def num_distinct(s: str, t: str) -> int:
    m = len(t)
    # ways[j] = number of ways to form t[:j] from the part of s read so far.
    ways = [0] * (m + 1)
    ways[0] = 1
    for ch in s:
        # Go right to left so ways[j - 1] still holds the previous row's value.
        for j in range(m, 0, -1):
            if t[j - 1] == ch:
                ways[j] += ways[j - 1]
    return ways[m]
