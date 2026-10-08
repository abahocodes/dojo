package main

func rob(nums []int) int {
	prev, curr := 0, 0
	for _, x := range nums {
		prev, curr = curr, max(curr, prev+x)
	}
	return curr
}
