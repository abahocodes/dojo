package main

import "container/heap"

// maxHeap implements heap.Interface with the largest value on top.
type maxHeap []int

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i] > h[j] }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	v := old[len(old)-1]
	*h = old[:len(old)-1]
	return v
}

func lastStoneWeight(stones []int) int {
	h := maxHeap(append([]int{}, stones...))
	heap.Init(&h)
	for h.Len() > 1 {
		y := heap.Pop(&h).(int)
		x := heap.Pop(&h).(int)
		if y != x {
			heap.Push(&h, y-x)
		}
	}
	if h.Len() == 0 {
		return 0
	}
	return h[0]
}
