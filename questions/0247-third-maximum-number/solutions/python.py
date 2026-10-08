def third_max(nums: list[int]) -> int:
    top = []  # distinct values, largest first, at most three
    for x in nums:
        if x in top:
            continue
        top.append(x)
        top.sort(reverse=True)
        if len(top) > 3:
            top.pop()
    return top[2] if len(top) == 3 else top[0]
