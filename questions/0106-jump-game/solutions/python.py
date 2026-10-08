def can_jump(nums: list[int]) -> bool:
    furthest = 0
    last = len(nums) - 1
    for i, step in enumerate(nums):
        if i > furthest:
            return False
        furthest = max(furthest, i + step)
        if furthest >= last:
            return True
    return True
