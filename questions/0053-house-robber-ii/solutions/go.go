package main

func robCircular(nums []int) int {
	line := func(lo, hi int) int {
		prev, curr := 0, 0
		for i := lo; i < hi; i++ {
			prev, curr = curr, max(curr, prev+nums[i])
		}
		return curr
	}

	n := len(nums)
	if n == 1 {
		return nums[0]
	}
	return max(line(0, n-1), line(1, n))
}
