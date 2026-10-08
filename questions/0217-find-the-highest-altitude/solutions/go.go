package main

func largestAltitude(gain []int) int {
	altitude, best := 0, 0
	for _, g := range gain {
		altitude += g
		if altitude > best {
			best = altitude
		}
	}
	return best
}
