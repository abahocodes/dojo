package main

import "sort"

func minSwapsToSort(nums []int) int {
	n := len(nums)
	// order[k] = index of the k-th smallest value
	order := make([]int, n)
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool { return nums[order[a]] < nums[order[b]] })
	seen := make([]bool, n)
	swaps := 0
	for i := 0; i < n; i++ {
		length := 0
		for j := i; !seen[j]; j = order[j] {
			seen[j] = true
			length++
		}
		if length > 0 {
			swaps += length - 1
		}
	}
	return swaps
}
