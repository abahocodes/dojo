package main

import "sort"

func arrayRankTransform(arr []int) []int {
	sorted := append([]int(nil), arr...)
	sort.Ints(sorted)
	rank := make(map[int]int, len(sorted))
	for _, x := range sorted {
		if _, ok := rank[x]; !ok {
			rank[x] = len(rank) + 1
		}
	}
	result := make([]int, len(arr))
	for i, x := range arr {
		result[i] = rank[x]
	}
	return result
}
