package main

import "sort"

func combinationSum(candidates []int, target int) [][]int {
	sorted := append([]int(nil), candidates...)
	sort.Ints(sorted)
	var result [][]int
	var current []int

	var backtrack func(start, remaining int)
	backtrack = func(start, remaining int) {
		if remaining == 0 {
			result = append(result, append([]int(nil), current...))
			return
		}
		for i := start; i < len(sorted); i++ {
			c := sorted[i]
			if c > remaining {
				break // sorted, so every later candidate is too big as well
			}
			current = append(current, c)
			backtrack(i, remaining-c) // i, not i + 1: a candidate may be reused
			current = current[:len(current)-1]
		}
	}

	backtrack(0, target)
	return result
}
