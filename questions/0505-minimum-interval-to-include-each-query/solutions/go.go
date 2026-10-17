package main

import (
	"container/heap"
	"sort"
)

// spanHeap holds {length, right end} pairs, shortest first.
type spanHeap [][2]int

func (h spanHeap) Len() int { return len(h) }
func (h spanHeap) Less(i, j int) bool {
	if h[i][0] != h[j][0] {
		return h[i][0] < h[j][0]
	}
	return h[i][1] < h[j][1]
}
func (h spanHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *spanHeap) Push(x interface{}) { *h = append(*h, x.([2]int)) }
func (h *spanHeap) Pop() interface{} {
	old := *h
	v := old[len(old)-1]
	*h = old[:len(old)-1]
	return v
}

func minInterval(intervals [][]int, queries []int) []int {
	sorted := make([][]int, len(intervals))
	copy(sorted, intervals)
	sort.Slice(sorted, func(a, b int) bool { return sorted[a][0] < sorted[b][0] })
	order := make([]int, len(queries))
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool { return queries[order[a]] < queries[order[b]] })

	answer := make([]int, len(queries))
	for i := range answer {
		answer[i] = -1
	}
	h := &spanHeap{}
	i := 0
	for _, qi := range order {
		q := queries[qi]
		for i < len(sorted) && sorted[i][0] <= q {
			heap.Push(h, [2]int{sorted[i][1] - sorted[i][0] + 1, sorted[i][1]})
			i++
		}
		// Intervals that end before q can never contain a later (larger) query.
		for h.Len() > 0 && (*h)[0][1] < q {
			heap.Pop(h)
		}
		if h.Len() > 0 {
			answer[qi] = (*h)[0][0]
		}
	}
	return answer
}
