package main

func minmaxGasDist(stations []int, k int) float64 {
	lo, hi := 0.0, 0.0
	for i := 1; i < len(stations); i++ {
		if g := float64(stations[i] - stations[i-1]); g > hi {
			hi = g
		}
	}
	fits := func(limit float64) bool {
		added := 0
		for i := 1; i < len(stations); i++ {
			added += int(float64(stations[i]-stations[i-1]) / limit)
			if added > k {
				return false
			}
		}
		return true
	}
	for iter := 0; iter < 100; iter++ {
		mid := (lo + hi) / 2
		if fits(mid) {
			hi = mid
		} else {
			lo = mid
		}
	}
	return hi
}
