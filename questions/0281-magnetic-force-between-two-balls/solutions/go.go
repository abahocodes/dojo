package main

import "sort"

func maxMinDistance(position []int, m int) int {
	pos := append([]int(nil), position...)
	sort.Ints(pos)
	// Greedily drop a ball in the leftmost basket at least gap past the last one.
	fits := func(gap int) bool {
		placed, last := 1, pos[0]
		for _, p := range pos[1:] {
			if p-last >= gap {
				placed++
				last = p
				if placed == m {
					return true
				}
			}
		}
		return false
	}
	lo, hi := 1, (pos[len(pos)-1]-pos[0])/(m-1)
	for lo < hi {
		mid := lo + (hi-lo+1)/2
		if fits(mid) {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	return lo
}
