package main

import "math"

func minSpeedOnTime(dist []int, hour float64) int {
	total := int(math.Round(hour * 100))
	last := dist[len(dist)-1] * 100
	onTime := func(speed int) bool {
		whole := 0
		for i := 0; i < len(dist)-1; i++ {
			whole += (dist[i] + speed - 1) / speed
		}
		rest := total - whole*100
		return rest >= 0 && (rest >= last || last <= rest*speed)
	}
	lo, hi := 1, 10000000
	if !onTime(hi) {
		return -1
	}
	for lo < hi {
		mid := lo + (hi-lo)/2
		if onTime(mid) {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
