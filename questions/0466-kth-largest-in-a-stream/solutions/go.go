package main

import "container/heap"

// minHeap implements heap.Interface with the smallest value on top.
type minHeap []int

func (h minHeap) Len() int            { return len(h) }
func (h minHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h minHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *minHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *minHeap) Pop() interface{} {
	old := *h
	v := old[len(old)-1]
	*h = old[:len(old)-1]
	return v
}

func kthLargestStream(k int, nums []int, adds []int) []int {
	// Min-heap holding the k largest values seen so far; its top is the answer.
	h := &minHeap{}
	add := func(v int) {
		if h.Len() < k {
			heap.Push(h, v)
		} else if v > (*h)[0] {
			(*h)[0] = v
			heap.Fix(h, 0)
		}
	}
	for _, v := range nums {
		add(v)
	}
	result := make([]int, len(adds))
	for i, v := range adds {
		add(v)
		result[i] = (*h)[0]
	}
	return result
}
