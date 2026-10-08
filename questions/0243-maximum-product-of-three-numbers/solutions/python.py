def maximum_product_three(nums: list[int]) -> int:
    inf = float("inf")
    max1 = max2 = max3 = -inf  # three largest, max1 >= max2 >= max3
    min1 = min2 = inf          # two smallest, min1 <= min2
    for x in nums:
        if x >= max1:
            max1, max2, max3 = x, max1, max2
        elif x >= max2:
            max2, max3 = x, max2
        elif x > max3:
            max3 = x
        if x <= min1:
            min1, min2 = x, min1
        elif x < min2:
            min2 = x
    return max(max1 * max2 * max3, max1 * min1 * min2)
