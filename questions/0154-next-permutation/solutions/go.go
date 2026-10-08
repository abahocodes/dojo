package main

func nextPermutation(nums []int) []int {
	n := len(nums)
	i := n - 2
	for i >= 0 && nums[i] >= nums[i+1] {
		i--
	}
	if i >= 0 {
		j := n - 1
		for nums[j] <= nums[i] {
			j--
		}
		nums[i], nums[j] = nums[j], nums[i]
	}
	for lo, hi := i+1, n-1; lo < hi; lo, hi = lo+1, hi-1 {
		nums[lo], nums[hi] = nums[hi], nums[lo]
	}
	return nums
}
