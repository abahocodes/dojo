def total_fruit(fruits: list[int]) -> int:
    count = {}
    left = 0
    best = 0
    for right, f in enumerate(fruits):
        count[f] = count.get(f, 0) + 1
        while len(count) > 2:
            g = fruits[left]
            count[g] -= 1
            if count[g] == 0:
                del count[g]
            left += 1
        best = max(best, right - left + 1)
    return best
