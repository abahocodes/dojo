package main

func bestClosingTime(customers string) int {
	delta, bestDelta, bestHour := 0, 0, 0
	for i := 0; i < len(customers); i++ {
		if customers[i] == 'Y' {
			delta--
		} else {
			delta++
		}
		if delta < bestDelta {
			bestDelta = delta
			bestHour = i + 1
		}
	}
	return bestHour
}
