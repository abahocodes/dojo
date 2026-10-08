package main

import (
	"container/heap"
	"math"
)

// maxHeap is a max-heap of pile sizes.
type maxHeap []int

func (h maxHeap) Len() int           { return len(h) }
func (h maxHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h maxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *maxHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}

func pickGifts(gifts []int, k int) int {
	h := make(maxHeap, len(gifts))
	copy(h, gifts)
	heap.Init(&h)
	for s := 0; s < k; s++ {
		h[0] = int(math.Sqrt(float64(h[0])))
		heap.Fix(&h, 0)
	}
	total := 0
	for _, g := range h {
		total += g
	}
	return total
}
