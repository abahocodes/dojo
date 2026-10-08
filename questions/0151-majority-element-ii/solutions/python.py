def majority_element(nums: list[int]) -> list[int]:
    c1, c2, n1, n2 = 0, 1, 0, 0
    for x in nums:
        if x == c1:
            n1 += 1
        elif x == c2:
            n2 += 1
        elif n1 == 0:
            c1, n1 = x, 1
        elif n2 == 0:
            c2, n2 = x, 1
        else:
            n1 -= 1
            n2 -= 1
    f1 = f2 = 0
    for x in nums:
        if x == c1:
            f1 += 1
        elif x == c2:
            f2 += 1
    result = []
    if f1 > len(nums) // 3:
        result.append(c1)
    if f2 > len(nums) // 3:
        result.append(c2)
    return result
