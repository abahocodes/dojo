def container_with_most_water(heights: list[int]) -> int:
    left, right = 0, len(heights) - 1
    best = 0
    while left < right:
        width = right - left
        if heights[left] < heights[right]:
            best = max(best, width * heights[left])
            left += 1
        else:
            best = max(best, width * heights[right])
            right -= 1
    return best
