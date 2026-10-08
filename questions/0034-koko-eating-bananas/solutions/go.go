package main

func minEatingSpeed(piles []int, h int) int {
	lo, hi := 1, 0
	for _, p := range piles {
		hi = max(hi, p)
	}
	for lo < hi {
		v := lo + (hi-lo)/2
		hours := 0
		for _, p := range piles {
			hours += (p + v - 1) / v
		}
		if hours <= h {
			hi = v
		} else {
			lo = v + 1
		}
	}
	return lo
}
