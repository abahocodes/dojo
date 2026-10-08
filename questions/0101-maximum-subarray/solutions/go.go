package main

func maxSubArray(nums []int) int {
	current, best := nums[0], nums[0]
	for _, x := range nums[1:] {
		current = max(x, current+x)
		best = max(best, current)
	}
	return best
}
