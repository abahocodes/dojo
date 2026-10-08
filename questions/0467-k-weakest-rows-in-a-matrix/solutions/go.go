package main

import "sort"

func kWeakestRows(mat [][]int, k int) []int {
	counts := make([]int, len(mat))
	order := make([]int, len(mat))
	for i, row := range mat {
		// Rows are 1s then 0s: binary search for the first 0.
		counts[i] = sort.Search(len(row), func(j int) bool { return row[j] == 0 })
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool {
		if counts[order[a]] != counts[order[b]] {
			return counts[order[a]] < counts[order[b]]
		}
		return order[a] < order[b]
	})
	return order[:k]
}
