package main

import "sort"

func minOperationsContinuous(nums []int) int {
	n := len(nums)
	sorted := append([]int(nil), nums...)
	sort.Ints(sorted)
	u := sorted[:0]
	for i, v := range sorted {
		if i == 0 || v != u[len(u)-1] {
			u = append(u, v)
		}
	}
	best, j := 0, 0
	for i := range u {
		for j < len(u) && u[j] <= u[i]+n-1 {
			j++
		}
		if j-i > best {
			best = j - i
		}
	}
	return n - best
}
