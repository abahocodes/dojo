package main

import "container/heap"

// farHeap is a max-heap of {squared distance, index} pairs.
type farHeap [][2]int

func (h farHeap) Len() int           { return len(h) }
func (h farHeap) Less(i, j int) bool { return h[i][0] > h[j][0] }
func (h farHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *farHeap) Push(x any)        { *h = append(*h, x.([2]int)) }
func (h *farHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}

func kClosest(points [][]int, k int) [][]int {
	h := &farHeap{}
	for i, p := range points {
		d := p[0]*p[0] + p[1]*p[1]
		if h.Len() < k {
			heap.Push(h, [2]int{d, i})
		} else if d < (*h)[0][0] {
			(*h)[0] = [2]int{d, i}
			heap.Fix(h, 0)
		}
	}
	result := make([][]int, 0, h.Len())
	for _, e := range *h {
		result = append(result, points[e[1]])
	}
	return result
}
