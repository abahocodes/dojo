package main

import "sort"

func subsetsWithDup(nums []int) [][]int {
	sorted := append([]int{}, nums...)
	sort.Ints(sorted) // equal values become neighbours
	result := [][]int{}
	current := []int{}

	var backtrack func(start int)
	backtrack = func(start int) {
		result = append(result, append([]int{}, current...))
		for i := start; i < len(sorted); i++ {
			if i > start && sorted[i] == sorted[i-1] {
				continue // already tried here
			}
			current = append(current, sorted[i])
			backtrack(i + 1)
			current = current[:len(current)-1]
		}
	}

	backtrack(0)
	return result
}
