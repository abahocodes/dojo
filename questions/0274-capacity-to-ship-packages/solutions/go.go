package main

func shipWithinDays(weights []int, days int) int {
	daysNeeded := func(cap int) int {
		used, load := 1, 0
		for _, w := range weights {
			if load+w > cap {
				used++
				load = 0
			}
			load += w
		}
		return used
	}
	lo, hi := 0, 0
	for _, w := range weights {
		if w > lo {
			lo = w
		}
		hi += w
	}
	for lo < hi {
		mid := (lo + hi) / 2
		if daysNeeded(mid) <= days {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
