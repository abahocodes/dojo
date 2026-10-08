package main

import "sort"

func findMinArrowShots(points [][]int) int {
	// Greedy: shoot each arrow at the right end of the balloon that ends first.
	ordered := append([][]int(nil), points...)
	sort.Slice(ordered, func(i, j int) bool { return ordered[i][1] < ordered[j][1] })
	arrows, pos := 1, ordered[0][1]
	for _, p := range ordered {
		if p[0] > pos {
			arrows++
			pos = p[1]
		}
	}
	return arrows
}
