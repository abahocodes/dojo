def largest_rectangle_area(heights: list[int]) -> int:
    stack = []
    best = 0
    n = len(heights)
    for i in range(n + 1):
        h = heights[i] if i < n else 0
        while stack and heights[stack[-1]] >= h:
            height = heights[stack.pop()]
            left = stack[-1] if stack else -1
            area = height * (i - left - 1)
            if area > best:
                best = area
        stack.append(i)
    return best
