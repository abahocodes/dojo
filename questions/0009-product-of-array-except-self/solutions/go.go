package main

func productExceptSelf(nums []int) []int {
	n := len(nums)
	out := make([]int, n)
	prefix := 1
	for i := 0; i < n; i++ {
		out[i] = prefix
		prefix *= nums[i]
	}
	suffix := 1
	for i := n - 1; i >= 0; i-- {
		out[i] *= suffix
		suffix *= nums[i]
	}
	return out
}
