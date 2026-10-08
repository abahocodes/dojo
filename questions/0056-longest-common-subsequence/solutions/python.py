def longest_common_subsequence(a: str, b: str) -> int:
    prev = [0] * (len(b) + 1)
    for ca in a:
        curr = [0] * (len(b) + 1)
        for j, cb in enumerate(b, 1):
            if ca == cb:
                curr[j] = prev[j - 1] + 1
            else:
                curr[j] = max(prev[j], curr[j - 1])
        prev = curr
    return prev[-1]
