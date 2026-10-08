package main

func smallestDivisor(nums []int, threshold int) int {
	total := func(d int) int {
		s := 0
		for _, x := range nums {
			s += (x + d - 1) / d
		}
		return s
	}
	lo, hi := 1, 1
	for _, x := range nums {
		if x > hi {
			hi = x
		}
	}
	for lo < hi {
		mid := lo + (hi-lo)/2
		if total(mid) <= threshold {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
