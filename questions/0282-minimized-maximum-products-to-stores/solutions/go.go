package main

func minimizedMaximum(n int, quantities []int) int {
	storesNeeded := func(cap int) int {
		s := 0
		for _, q := range quantities {
			s += (q + cap - 1) / cap
		}
		return s
	}
	lo, hi := 1, 1
	for _, q := range quantities {
		if q > hi {
			hi = q
		}
	}
	for lo < hi {
		mid := lo + (hi-lo)/2
		if storesNeeded(mid) <= n {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
