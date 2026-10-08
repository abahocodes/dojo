package main

func rotate(nums []int, k int) []int {
	n := len(nums)
	k %= n
	reverse := func(lo, hi int) {
		for lo < hi {
			nums[lo], nums[hi] = nums[hi], nums[lo]
			lo++
			hi--
		}
	}
	reverse(0, n-1)
	reverse(0, k-1)
	reverse(k, n-1)
	return nums
}
