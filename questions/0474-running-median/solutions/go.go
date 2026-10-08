package main

import "container/heap"

// intHeap is a min-heap of ints; the max-heap stores negated values.
type intHeap []int

func (h intHeap) Len() int           { return len(h) }
func (h intHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h intHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *intHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *intHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}

func runningMedian(nums []int) []float64 {
	low := &intHeap{}  // smaller half, negated (max-heap)
	high := &intHeap{} // larger half (min-heap)
	medians := make([]float64, 0, len(nums))
	for _, x := range nums {
		if low.Len() == 0 || x <= -(*low)[0] {
			heap.Push(low, -x)
		} else {
			heap.Push(high, x)
		}
		// Keep low.Len() == high.Len() or low.Len() == high.Len()+1.
		if low.Len() > high.Len()+1 {
			heap.Push(high, -heap.Pop(low).(int))
		} else if high.Len() > low.Len() {
			heap.Push(low, -heap.Pop(high).(int))
		}
		if low.Len() > high.Len() {
			medians = append(medians, float64(-(*low)[0]))
		} else {
			medians = append(medians, float64(-(*low)[0]+(*high)[0])/2)
		}
	}
	return medians
}
