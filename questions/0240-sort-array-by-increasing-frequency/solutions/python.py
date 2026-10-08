def frequency_sort_numbers(nums: list[int]) -> list[int]:
    count = {}
    for x in nums:
        count[x] = count.get(x, 0) + 1
    # rarer values first; among equally frequent values, larger first
    return sorted(nums, key=lambda x: (count[x], -x))
