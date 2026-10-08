package main

import "container/heap"

// intHeap is a min-heap of ints; store negated values for a max-heap.
type intHeap []int

func (h intHeap) Len() int            { return len(h) }
func (h intHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h intHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *intHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *intHeap) Pop() interface{} {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func medianSlidingWindow(nums []int, k int) []float64 {
	low := &intHeap{}  // smaller half, values negated (max-heap)
	high := &intHeap{} // larger half (min-heap)
	delayed := map[int]int{} // value -> copies removed but still inside a heap
	lowSize, highSize := 0, 0

	lowTop := func() int { return -(*low)[0] }
	prune := func(h *intHeap, sign int) {
		for h.Len() > 0 && delayed[sign*(*h)[0]] > 0 {
			delayed[sign*(*h)[0]]--
			heap.Pop(h)
		}
	}
	rebalance := func() {
		if lowSize > highSize+1 {
			heap.Push(high, -heap.Pop(low).(int))
			lowSize--
			highSize++
			prune(low, -1)
		} else if lowSize < highSize {
			heap.Push(low, -heap.Pop(high).(int))
			lowSize++
			highSize--
			prune(high, 1)
		}
	}
	add := func(x int) {
		if low.Len() == 0 || x <= lowTop() {
			heap.Push(low, -x)
			lowSize++
		} else {
			heap.Push(high, x)
			highSize++
		}
		rebalance()
	}
	remove := func(x int) {
		delayed[x]++
		if x <= lowTop() {
			lowSize--
			if x == lowTop() {
				prune(low, -1)
			}
		} else {
			highSize--
			if high.Len() > 0 && x == (*high)[0] {
				prune(high, 1)
			}
		}
		rebalance()
	}

	result := make([]float64, 0, len(nums)-k+1)
	for i, x := range nums {
		add(x)
		if i >= k {
			remove(nums[i-k])
		}
		if i >= k-1 {
			if k%2 == 1 {
				result = append(result, float64(lowTop()))
			} else {
				result = append(result, (float64(lowTop())+float64((*high)[0]))/2)
			}
		}
	}
	return result
}
