package main

func countBadPairs(nums []int) int {
	seen := map[int]int{}
	good := 0
	for j, x := range nums {
		key := x - j
		good += seen[key]
		seen[key]++
	}
	n := len(nums)
	return n*(n-1)/2 - good
}
