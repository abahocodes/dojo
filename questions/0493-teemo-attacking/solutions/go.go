package main

func findPoisonedDuration(timeSeries []int, duration int) int {
	total := 0
	for i := 0; i+1 < len(timeSeries); i++ {
		// The poison runs its full course unless the next attack resets it.
		total += min(duration, timeSeries[i+1]-timeSeries[i])
	}
	return total + duration
}
