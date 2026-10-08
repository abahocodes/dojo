def maximal_rectangle(matrix: list[str]) -> int:
    cols = len(matrix[0])
    heights = [0] * (cols + 1)  # heights[cols] stays 0 as a sentinel
    best = 0
    for row in matrix:
        for c in range(cols):
            heights[c] = heights[c] + 1 if row[c] == "1" else 0
        stack = []
        for i in range(cols + 1):
            while stack and heights[stack[-1]] >= heights[i]:
                h = heights[stack.pop()]
                left = stack[-1] if stack else -1
                best = max(best, h * (i - left - 1))
            stack.append(i)
    return best
