package main

func tupleSameProduct(nums []int) int {
	seen := make(map[int]int, len(nums)*len(nums)/2)
	total := 0
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			p := nums[i] * nums[j]
			total += 8 * seen[p]
			seen[p]++
		}
	}
	return total
}
