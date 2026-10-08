package main

func minimumTime(time []int, totalTrips int) int {
	// Stop as soon as the target is reached so the running count cannot overflow.
	enough := func(t int) bool {
		done := 0
		for _, x := range time {
			done += t / x
			if done >= totalTrips {
				return true
			}
		}
		return false
	}
	fastest := time[0]
	for _, x := range time {
		if x < fastest {
			fastest = x
		}
	}
	lo, hi := 1, fastest*totalTrips
	for lo < hi {
		mid := lo + (hi-lo)/2
		if enough(mid) {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
