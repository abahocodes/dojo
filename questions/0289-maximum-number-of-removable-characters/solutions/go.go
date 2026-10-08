package main

func maximumRemovals(s string, p string, removable []int) int {
	removedAt := make([]int, len(s))
	for i := range removedAt {
		removedAt[i] = len(removable)
	}
	for step, i := range removable {
		removedAt[i] = step
	}
	survives := func(k int) bool {
		j := 0
		for i := 0; i < len(s) && j < len(p); i++ {
			if removedAt[i] >= k && s[i] == p[j] {
				j++
			}
		}
		return j == len(p)
	}
	lo, hi := 0, len(removable)
	for lo < hi {
		mid := lo + (hi-lo+1)/2
		if survives(mid) {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	return lo
}
