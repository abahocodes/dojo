package main

import "slices"

func threeSum(nums []int) [][]int {
	sorted := slices.Clone(nums)
	slices.Sort(sorted)
	n := len(sorted)
	var result [][]int
	for i := 0; i < n-2; i++ {
		a := sorted[i]
		if a > 0 {
			break
		}
		if i > 0 && a == sorted[i-1] {
			continue
		}
		lo, hi := i+1, n-1
		for lo < hi {
			s := a + sorted[lo] + sorted[hi]
			if s < 0 {
				lo++
			} else if s > 0 {
				hi--
			} else {
				result = append(result, []int{a, sorted[lo], sorted[hi]})
				lo++
				hi--
				for lo < hi && sorted[lo] == sorted[lo-1] {
					lo++
				}
			}
		}
	}
	return result
}
