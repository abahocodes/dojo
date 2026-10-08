def remove_element(nums: list[int], val: int) -> list[int]:
    write = 0
    for x in nums:
        if x != val:
            nums[write] = x
            write += 1
    return nums[:write]
