def xor_queries(arr: list[int], queries: list[list[int]]) -> list[int]:
    prefix = [0] * (len(arr) + 1)
    for i, x in enumerate(arr):
        prefix[i + 1] = prefix[i] ^ x
    return [prefix[r + 1] ^ prefix[l] for l, r in queries]
