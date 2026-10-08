def subarrays_div_by_k(nums: list[int], k: int) -> int:
    count = [0] * k
    count[0] = 1
    rem = 0
    result = 0
    for x in nums:
        rem = (rem + x) % k  # Python's % is already non-negative
        result += count[rem]
        count[rem] += 1
    return result
