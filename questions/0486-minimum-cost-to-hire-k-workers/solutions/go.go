package main

import (
	"container/heap"
	"math"
	"sort"
)

// maxHeap holds the qualities of the current group, largest on top.
type maxHeap []int

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i] > h[j] }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func mincostToHireWorkers(quality []int, wage []int, k int) float64 {
	n := len(quality)
	// Sort workers by wage/quality ratio, compared exactly by cross-multiplying.
	order := make([]int, n)
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool {
		i, j := order[a], order[b]
		return wage[i]*quality[j] < wage[j]*quality[i]
	})

	h := &maxHeap{}
	totalQuality := 0
	best := math.Inf(1)
	for _, i := range order {
		heap.Push(h, quality[i])
		totalQuality += quality[i]
		if h.Len() > k {
			totalQuality -= heap.Pop(h).(int) // drop the largest quality
		}
		if h.Len() == k {
			// Worker i has the largest ratio so far and sets the pay rate.
			best = math.Min(best, float64(totalQuality*wage[i])/float64(quality[i]))
		}
	}
	return best
}
