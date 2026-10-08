package main

func numSubarrayProductLessThanK(nums []int, k int) int {
	if k <= 1 {
		return 0
	}
	product, left, total := 1, 0, 0
	for right, x := range nums {
		product *= x
		for product >= k {
			product /= nums[left]
			left++
		}
		total += right - left + 1
	}
	return total
}
