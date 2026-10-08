package main

import "sort"

func maxFrequency(nums []int, k int) int {
	a := append([]int(nil), nums...)
	sort.Ints(a)
	left, window, best := 0, 0, 0
	for right, v := range a {
		window += v
		for v*(right-left+1)-window > k {
			window -= a[left]
			left++
		}
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}
