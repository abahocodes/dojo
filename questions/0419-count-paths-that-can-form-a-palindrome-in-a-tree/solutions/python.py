from collections import defaultdict


def count_palindrome_paths(parent: list[int], s: str) -> int:
    n = len(parent)
    children = [[] for _ in range(n)]
    for v in range(1, n):
        children[parent[v]].append(v)
    # mask[v] = parity of each letter on the path root -> v
    mask = [0] * n
    order = [0]
    for v in order:  # BFS: order grows while we walk it
        for c in children[v]:
            mask[c] = mask[v] ^ (1 << (ord(s[c]) - 97))
            order.append(c)
    seen = defaultdict(int)
    total = 0
    for v in range(n):
        m = mask[v]
        total += seen[m]
        for b in range(26):
            total += seen.get(m ^ (1 << b), 0)
        seen[m] += 1
    return total
