package main

import "sort"

func threeSumClosest(nums []int, target int) int {
	a := append([]int(nil), nums...)
	sort.Ints(a)
	n := len(a)
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	best := a[0] + a[1] + a[2]
	for i := 0; i < n-2; i++ {
		lo, hi := i+1, n-1
		for lo < hi {
			s := a[i] + a[lo] + a[hi]
			if abs(s-target) < abs(best-target) {
				best = s
			}
			if s < target {
				lo++
			} else if s > target {
				hi--
			} else {
				return s
			}
		}
	}
	return best
}
