package main

func singleNumber(nums []int) int {
	result := 0
	for _, x := range nums {
		result ^= x
	}
	return result
}
