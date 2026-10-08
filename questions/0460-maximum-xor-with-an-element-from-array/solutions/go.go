package main

import "sort"

func maximizeXor(nums []int, queries [][]int) []int {
	const bits = 30
	sorted := append([]int{}, nums...)
	sort.Ints(sorted)
	order := make([]int, len(queries))
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool { return queries[order[a]][1] < queries[order[b]][1] })

	child := make([][2]int, len(sorted)*bits+1)
	nodes := 1
	answer := make([]int, len(queries))
	j := 0

	for _, qi := range order {
		x, limit := queries[qi][0], queries[qi][1]
		for j < len(sorted) && sorted[j] <= limit {
			v, node := sorted[j], 0
			for b := bits - 1; b >= 0; b-- {
				bit := (v >> b) & 1
				if child[node][bit] == 0 {
					child[node][bit] = nodes
					nodes++
				}
				node = child[node][bit]
			}
			j++
		}
		if j == 0 {
			answer[qi] = -1
			continue
		}
		node, best := 0, 0
		for b := bits - 1; b >= 0; b-- {
			want := ((x >> b) & 1) ^ 1
			if child[node][want] != 0 {
				best |= 1 << b
				node = child[node][want]
			} else {
				node = child[node][want^1]
			}
		}
		answer[qi] = best
	}
	return answer
}
