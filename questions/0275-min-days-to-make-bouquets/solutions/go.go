package main

func minDays(bloomDay []int, m int, k int) int {
	if m*k > len(bloomDay) {
		return -1
	}
	bouquets := func(day int) int {
		made, run := 0, 0
		for _, b := range bloomDay {
			if b <= day {
				run++
				if run == k {
					made++
					run = 0
				}
			} else {
				run = 0
			}
		}
		return made
	}
	lo, hi := bloomDay[0], bloomDay[0]
	for _, b := range bloomDay {
		if b < lo {
			lo = b
		}
		if b > hi {
			hi = b
		}
	}
	for lo < hi {
		mid := lo + (hi-lo)/2
		if bouquets(mid) >= m {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
