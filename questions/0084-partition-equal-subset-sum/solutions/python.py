def can_partition(nums: list[int]) -> bool:
    total = sum(nums)
    if total % 2:
        return False
    half = total // 2
    reachable = 1  # bit s is set when some subset weighs exactly s
    for x in nums:
        reachable |= reachable << x
    return (reachable >> half) & 1 == 1
