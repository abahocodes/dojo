def longest_common_prefix_numbers(arr1: list[int], arr2: list[int]) -> int:
    prefixes = set()
    for x in arr1:
        while x > 0 and x not in prefixes:
            prefixes.add(x)
            x //= 10
    best = 0
    for y in arr2:
        while y > 0 and y not in prefixes:
            y //= 10
        if y > 0:
            best = max(best, len(str(y)))
    return best
