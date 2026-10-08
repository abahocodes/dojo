package main

func findErrorNums(nums []int) []int {
	n := len(nums)
	seen := make([]bool, n+1)
	dup, total := 0, 0
	for _, x := range nums {
		if seen[x] {
			dup = x
		}
		seen[x] = true
		total += x
	}
	missing := n*(n+1)/2 - (total - dup)
	return []int{dup, missing}
}
