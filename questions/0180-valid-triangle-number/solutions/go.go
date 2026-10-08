package main

import "sort"

func triangleNumber(nums []int) int {
	a := append([]int(nil), nums...)
	sort.Ints(a)
	count := 0
	for k := len(a) - 1; k >= 2; k-- {
		i, j := 0, k-1
		for i < j {
			if a[i]+a[j] > a[k] {
				count += j - i
				j--
			} else {
				i++
			}
		}
	}
	return count
}
