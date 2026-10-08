package main

func maximumCandies(candies []int, k int) int {
	lo, hi := 0, 0
	for _, c := range candies {
		if c > hi {
			hi = c
		}
	}
	for lo < hi {
		mid := lo + (hi-lo+1)/2
		shares := 0
		for _, c := range candies {
			shares += c / mid
			if shares >= k {
				break
			}
		}
		if shares >= k {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	return lo
}
