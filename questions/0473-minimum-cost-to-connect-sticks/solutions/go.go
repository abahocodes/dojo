package main

import "container/heap"

// minHeap is a min-heap of stick lengths.
type minHeap []int

func (h minHeap) Len() int           { return len(h) }
func (h minHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h minHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *minHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *minHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}

func connectSticks(sticks []int) int {
	h := make(minHeap, len(sticks))
	copy(h, sticks)
	heap.Init(&h)
	total := 0
	for h.Len() > 1 {
		joined := heap.Pop(&h).(int) + heap.Pop(&h).(int)
		total += joined
		heap.Push(&h, joined)
	}
	return total
}
