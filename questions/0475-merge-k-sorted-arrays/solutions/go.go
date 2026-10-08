package main

import "container/heap"

// frontHeap is a min-heap of {value, array index, position}, ordered by value.
type frontHeap [][3]int

func (h frontHeap) Len() int           { return len(h) }
func (h frontHeap) Less(i, j int) bool { return h[i][0] < h[j][0] }
func (h frontHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *frontHeap) Push(x any)        { *h = append(*h, x.([3]int)) }
func (h *frontHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}

func mergeKSortedArrays(arrays [][]int) []int {
	h := &frontHeap{}
	total := 0
	for a, arr := range arrays {
		total += len(arr)
		if len(arr) > 0 {
			*h = append(*h, [3]int{arr[0], a, 0})
		}
	}
	heap.Init(h)
	merged := make([]int, 0, total)
	for h.Len() > 0 {
		top := (*h)[0]
		merged = append(merged, top[0])
		a, p := top[1], top[2]
		if p+1 < len(arrays[a]) {
			(*h)[0] = [3]int{arrays[a][p+1], a, p + 1}
			heap.Fix(h, 0)
		} else {
			heap.Pop(h)
		}
	}
	return merged
}
