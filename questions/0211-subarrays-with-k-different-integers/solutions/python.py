def subarrays_with_k_distinct(nums: list[int], k: int) -> int:
    def at_most(limit: int) -> int:
        count = [0] * (len(nums) + 1)
        distinct = 0
        left = 0
        total = 0
        for right, value in enumerate(nums):
            if count[value] == 0:
                distinct += 1
            count[value] += 1
            while distinct > limit:
                old = nums[left]
                count[old] -= 1
                if count[old] == 0:
                    distinct -= 1
                left += 1
            total += right - left + 1
        return total

    return at_most(k) - at_most(k - 1)
