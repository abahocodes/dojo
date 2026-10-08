package main

import (
	"math"
	"sort"
)

func eraseOverlapIntervals(intervals [][]int) int {
	byEnd := append([][]int(nil), intervals...)
	// keeping the interval that ends first leaves the most room for the rest
	sort.Slice(byEnd, func(a, b int) bool { return byEnd[a][1] < byEnd[b][1] })
	removed := 0
	lastEnd := math.MinInt
	for _, iv := range byEnd {
		if iv[0] >= lastEnd {
			lastEnd = iv[1]
		} else {
			removed++
		}
	}
	return removed
}
