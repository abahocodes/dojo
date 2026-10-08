package main

import "sort"

func minimumDifference(nums []int, k int) int {
	s := append([]int(nil), nums...)
	sort.Ints(s)
	best := s[k-1] - s[0]
	for i := 1; i+k-1 < len(s); i++ {
		if d := s[i+k-1] - s[i]; d < best {
			best = d
		}
	}
	return best
}
