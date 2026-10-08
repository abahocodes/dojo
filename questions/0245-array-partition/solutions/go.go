package main

import "sort"

func arrayPairSum(nums []int) int {
	a := append([]int(nil), nums...)
	sort.Ints(a)
	total := 0
	for i := 0; i < len(a); i += 2 {
		total += a[i]
	}
	return total
}
