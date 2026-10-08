package main

import "sort"

func findRadius(houses []int, heaters []int) int {
	hs := append([]int(nil), heaters...)
	sort.Ints(hs)
	best := 0
	for _, x := range houses {
		i := sort.SearchInts(hs, x)
		near := -1
		if i < len(hs) {
			near = hs[i] - x
		}
		if i > 0 && (near < 0 || x-hs[i-1] < near) {
			near = x - hs[i-1]
		}
		if near > best {
			best = near
		}
	}
	return best
}
