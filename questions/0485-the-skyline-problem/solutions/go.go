package main

import (
	"container/heap"
	"sort"
)

// liveHeap is a max-heap of {height, right} pairs, ordered by height.
type liveHeap [][2]int

func (h liveHeap) Len() int            { return len(h) }
func (h liveHeap) Less(i, j int) bool  { return h[i][0] > h[j][0] }
func (h liveHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *liveHeap) Push(x interface{}) { *h = append(*h, x.([2]int)) }
func (h *liveHeap) Pop() interface{} {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func getSkyline(buildings [][]int) [][]int {
	// Events {x, -height, right}: starts carry -height, ends carry 0.
	// Sorted by x, then starts before ends, tallest start first.
	events := make([][3]int, 0, 2*len(buildings))
	for _, b := range buildings {
		events = append(events, [3]int{b[0], -b[2], b[1]}, [3]int{b[1], 0, 0})
	}
	sort.Slice(events, func(i, j int) bool {
		if events[i][0] != events[j][0] {
			return events[i][0] < events[j][0]
		}
		return events[i][1] < events[j][1]
	})

	live := &liveHeap{}
	result := [][]int{}
	for _, ev := range events {
		x := ev[0]
		// Lazily drop buildings that ended at or before x.
		for live.Len() > 0 && (*live)[0][1] <= x {
			heap.Pop(live)
		}
		if ev[1] != 0 {
			heap.Push(live, [2]int{-ev[1], ev[2]})
		}
		// The first event at x already settles the height at x.
		current := 0
		if live.Len() > 0 {
			current = (*live)[0][0]
		}
		if len(result) == 0 || result[len(result)-1][1] != current {
			result = append(result, []int{x, current})
		}
	}
	return result
}
