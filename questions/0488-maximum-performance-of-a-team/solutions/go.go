package main

import (
	"container/heap"
	"sort"
)

// minHeap holds the speeds of the current team, smallest on top.
type minHeap []int

func (h minHeap) Len() int            { return len(h) }
func (h minHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h minHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *minHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *minHeap) Pop() interface{} {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func maxPerformance(n int, speed []int, efficiency []int, k int) int {
	order := make([]int, n)
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool { return efficiency[order[a]] > efficiency[order[b]] })

	h := &minHeap{}
	totalSpeed := 0
	best := 0 // up to 10^18: fits in a 64-bit int
	for _, i := range order {
		// efficiency[i] is the smallest efficiency so far: it is the team minimum.
		heap.Push(h, speed[i])
		totalSpeed += speed[i]
		if h.Len() > k {
			totalSpeed -= heap.Pop(h).(int)
		}
		if p := totalSpeed * efficiency[i]; p > best {
			best = p
		}
	}
	return best % 1000000007
}
