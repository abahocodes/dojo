package main

func minOperationsReduceX(nums []int, x int) int {
	total := 0
	for _, v := range nums {
		total += v
	}
	target := total - x
	if target < 0 {
		return -1
	}
	best, window, left := -1, 0, 0
	for right, v := range nums {
		window += v
		for window > target {
			window -= nums[left]
			left++
		}
		if window == target && right-left+1 > best {
			best = right - left + 1
		}
	}
	if best == -1 {
		return -1
	}
	return len(nums) - best
}
