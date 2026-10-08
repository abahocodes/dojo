package main

func maxRunTime(n int, batteries []int) int {
	total := 0
	for _, b := range batteries {
		total += b
	}
	canRun := func(minutes int) bool {
		usable := 0
		for _, b := range batteries {
			if b < minutes {
				usable += b
			} else {
				usable += minutes
			}
		}
		return usable >= n*minutes
	}

	lo, hi := 0, total/n
	for lo < hi {
		mid := lo + (hi-lo+1)/2
		if canRun(mid) {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	return lo
}
