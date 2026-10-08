package main

import "sort"

func longestIncreasingSubsequence(nums []int) int {
	var tails []int
	for _, x := range nums {
		pos := sort.SearchInts(tails, x)
		if pos == len(tails) {
			tails = append(tails, x)
		} else {
			tails[pos] = x
		}
	}
	return len(tails)
}
