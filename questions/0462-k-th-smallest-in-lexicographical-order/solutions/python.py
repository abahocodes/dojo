def find_kth_number(n: int, k: int) -> int:
    def subtree_size(prefix: int) -> int:
        # How many numbers in [1, n] start with the decimal digits of prefix.
        count = 0
        first, last = prefix, prefix
        while first <= n:
            count += min(n, last) - first + 1
            first *= 10
            last = last * 10 + 9
        return count

    current = 1
    k -= 1  # number of steps still to take in lexicographic order
    while k > 0:
        size = subtree_size(current)
        if size <= k:
            k -= size       # skip the whole subtree, move to the next sibling
            current += 1
        else:
            k -= 1          # step into the subtree: its first element is current * 10
            current *= 10
    return current
