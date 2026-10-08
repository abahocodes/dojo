package main

import (
	"container/heap"
	"sort"
)

type intHeap []int

func (h intHeap) Len() int           { return len(h) }
func (h intHeap) Less(a, b int) bool { return h[a] < h[b] }
func (h intHeap) Swap(a, b int)      { h[a], h[b] = h[b], h[a] }
func (h *intHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *intHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func maxScore(nums1 []int, nums2 []int, k int) int {
	order := make([]int, len(nums1))
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool { return nums2[order[a]] > nums2[order[b]] })
	chosen := &intHeap{}
	total, best := 0, 0
	for _, i := range order {
		heap.Push(chosen, nums1[i])
		total += nums1[i]
		if chosen.Len() > k {
			total -= heap.Pop(chosen).(int)
		}
		if chosen.Len() == k && total*nums2[i] > best {
			best = total * nums2[i]
		}
	}
	return best
}
