package main

func containerWithMostWater(heights []int) int {
	left, right := 0, len(heights)-1
	best := 0
	for left < right {
		width := right - left
		if heights[left] < heights[right] {
			best = max(best, width*heights[left])
			left++
		} else {
			best = max(best, width*heights[right])
			right--
		}
	}
	return best
}
