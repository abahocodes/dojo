def max_chunks_to_sorted(arr: list[int]) -> int:
    stack = []  # maximum of each chunk, non-decreasing
    for x in arr:
        if not stack or x >= stack[-1]:
            stack.append(x)
        else:
            biggest = stack[-1]
            while stack and stack[-1] > x:
                stack.pop()
            stack.append(biggest)
    return len(stack)
