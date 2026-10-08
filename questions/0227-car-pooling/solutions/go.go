package main

func carPooling(trips [][]int, capacity int) bool {
	// change[x] = passengers boarding at km x minus passengers leaving at km x
	change := make([]int, 1001)
	for _, t := range trips {
		change[t[1]] += t[0]
		change[t[2]] -= t[0]
	}
	load := 0
	for _, delta := range change {
		load += delta
		if load > capacity {
			return false
		}
	}
	return true
}
