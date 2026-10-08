def find_shortest_sub_array(nums: list[int]) -> int:
    first = {}
    count = {}
    degree = 0
    best = 0
    for i, x in enumerate(nums):
        first.setdefault(x, i)
        count[x] = count.get(x, 0) + 1
        span = i - first[x] + 1
        if count[x] > degree or (count[x] == degree and span < best):
            degree = count[x]
            best = span
    return best
