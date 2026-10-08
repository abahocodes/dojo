package main

import "sort"

func successfulPairs(spells []int, potions []int, success int) []int {
	sorted := append([]int(nil), potions...)
	sort.Ints(sorted)
	m := len(sorted)
	result := make([]int, len(spells))
	for i, s := range spells {
		// Smallest potion strength p with s * p >= success.
		need := (success + s - 1) / s
		result[i] = m - sort.SearchInts(sorted, need)
	}
	return result
}
