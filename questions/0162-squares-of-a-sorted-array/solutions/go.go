package main

func sortedSquares(nums []int) []int {
	n := len(nums)
	out := make([]int, n)
	lo, hi := 0, n-1
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	for w := n - 1; w >= 0; w-- {
		if abs(nums[lo]) > abs(nums[hi]) {
			out[w] = nums[lo] * nums[lo]
			lo++
		} else {
			out[w] = nums[hi] * nums[hi]
			hi--
		}
	}
	return out
}
