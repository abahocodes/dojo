package main

func subarraySum(nums []int, k int) int {
	seen := map[int]int{0: 1}
	running, count := 0, 0
	for _, x := range nums {
		running += x
		count += seen[running-k]
		seen[running]++
	}
	return count
}
