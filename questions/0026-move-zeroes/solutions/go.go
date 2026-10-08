package main

func moveZeroes(nums []int) []int {
	w := 0
	for read := range nums {
		if nums[read] != 0 {
			nums[w], nums[read] = nums[read], nums[w]
			w++
		}
	}
	return nums
}
