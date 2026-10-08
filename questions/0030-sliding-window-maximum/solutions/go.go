package main

func maxSlidingWindow(nums []int, k int) []int {
	// Deque of indices backed by a slice with a moving head.
	dq := make([]int, 0, len(nums))
	head := 0
	var out []int
	for i, x := range nums {
		for len(dq) > head && nums[dq[len(dq)-1]] <= x {
			dq = dq[:len(dq)-1]
		}
		dq = append(dq, i)
		if dq[head] <= i-k {
			head++
		}
		if i >= k-1 {
			out = append(out, nums[dq[head]])
		}
	}
	return out
}
