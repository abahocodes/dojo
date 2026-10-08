def next_greater_element(nums1: list[int], nums2: list[int]) -> list[int]:
    nxt = {}
    stack = []  # values still waiting for a greater one, decreasing
    for x in nums2:
        while stack and stack[-1] < x:
            nxt[stack.pop()] = x
        stack.append(x)
    return [nxt.get(x, -1) for x in nums1]
