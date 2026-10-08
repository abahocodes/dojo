from collections import Counter


def kth_distinct(arr: list[str], k: int) -> str:
    count = Counter(arr)
    for s in arr:
        if count[s] == 1:
            k -= 1
            if k == 0:
                return s
    return ""
