package main

import "sort"

func fourSum(nums []int, target int) [][]int {
	a := append([]int(nil), nums...)
	sort.Ints(a)
	n := len(a)
	res := [][]int{}
	for i := 0; i < n-3; i++ {
		if i > 0 && a[i] == a[i-1] {
			continue
		}
		for j := i + 1; j < n-2; j++ {
			if j > i+1 && a[j] == a[j-1] {
				continue
			}
			lo, hi := j+1, n-1
			for lo < hi {
				s := int64(a[i]) + int64(a[j]) + int64(a[lo]) + int64(a[hi])
				if s < int64(target) {
					lo++
				} else if s > int64(target) {
					hi--
				} else {
					res = append(res, []int{a[i], a[j], a[lo], a[hi]})
					lo++
					for lo < hi && a[lo] == a[lo-1] {
						lo++
					}
					hi--
					for lo < hi && a[hi] == a[hi+1] {
						hi--
					}
				}
			}
		}
	}
	return res
}
