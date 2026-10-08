package main

func findBestValue(arr []int, target int) int {
	capped := func(v int) int {
		s := 0
		for _, a := range arr {
			s += min(a, v)
		}
		return s
	}
	lo, hi := 0, 0
	for _, a := range arr {
		hi = max(hi, a)
	}
	if capped(hi) < target {
		return hi
	}
	for lo < hi {
		mid := (lo + hi) / 2
		if capped(mid) >= target {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	if lo > 0 && target-capped(lo-1) <= capped(lo)-target {
		return lo - 1
	}
	return lo
}
