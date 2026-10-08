def tuple_same_product(nums: list[int]) -> int:
    seen = {}
    total = 0
    n = len(nums)
    for i in range(n):
        a = nums[i]
        for j in range(i + 1, n):
            p = a * nums[j]
            count = seen.get(p, 0)
            total += 8 * count
            seen[p] = count + 1
    return total
