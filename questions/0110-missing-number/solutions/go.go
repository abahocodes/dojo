package main

func missingNumber(nums []int) int {
	result := len(nums)
	for i, x := range nums {
		result ^= i ^ x
	}
	return result
}
