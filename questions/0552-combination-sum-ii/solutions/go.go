package main

import "sort"

func combinationSum2(candidates []int, target int) [][]int {
	c := append([]int{}, candidates...)
	sort.Ints(c) // equal values become neighbours
	result := [][]int{}
	current := []int{}

	var backtrack func(start, remaining int)
	backtrack = func(start, remaining int) {
		if remaining == 0 {
			result = append(result, append([]int{}, current...))
			return
		}
		for i := start; i < len(c); i++ {
			if c[i] > remaining {
				break // every later candidate is too big as well
			}
			if i > start && c[i] == c[i-1] {
				continue // already tried here
			}
			current = append(current, c[i])
			backtrack(i+1, remaining-c[i]) // i + 1: each card is used once
			current = current[:len(current)-1]
		}
	}

	backtrack(0, target)
	return result
}
