package main

import "sort"

func removeCoveredIntervals(intervals [][]int) int {
	// Start ascending; for equal starts the longer interval comes first.
	ordered := append([][]int(nil), intervals...)
	sort.Slice(ordered, func(i, j int) bool {
		if ordered[i][0] != ordered[j][0] {
			return ordered[i][0] < ordered[j][0]
		}
		return ordered[i][1] > ordered[j][1]
	})
	remaining, maxEnd := 0, -1
	for _, iv := range ordered {
		if iv[1] > maxEnd {
			remaining++
			maxEnd = iv[1]
		}
	}
	return remaining
}
