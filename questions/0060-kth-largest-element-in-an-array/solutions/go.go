package main

import "container/heap"

// minHeap is a min-heap of ints.
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

func findKthLargest(nums []int, k int) int {
	// Min-heap holding the k largest values seen so far; h[0] is the smallest of them.
	h := make(minHeap, k)
	copy(h, nums[:k])
	heap.Init(&h)
	for _, x := range nums[k:] {
		if x > h[0] {
			h[0] = x
			heap.Fix(&h, 0)
		}
	}
	return h[0]
}
