package main

import "container/heap"

// pairHeap is a min-heap of {sum, i, j}, ordered by sum, then i, then j.
type pairHeap [][3]int

func (h pairHeap) Len() int { return len(h) }
func (h pairHeap) Less(a, b int) bool {
	if h[a][0] != h[b][0] {
		return h[a][0] < h[b][0]
	}
	if h[a][1] != h[b][1] {
		return h[a][1] < h[b][1]
	}
	return h[a][2] < h[b][2]
}
func (h pairHeap) Swap(a, b int) { h[a], h[b] = h[b], h[a] }
func (h *pairHeap) Push(x any)   { *h = append(*h, x.([3]int)) }
func (h *pairHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}

func kSmallestPairs(nums1 []int, nums2 []int, k int) [][]int {
	h := &pairHeap{}
	for i := 0; i < len(nums1) && i < k; i++ {
		*h = append(*h, [3]int{nums1[i] + nums2[0], i, 0})
	}
	heap.Init(h)
	result := [][]int{}
	for h.Len() > 0 && len(result) < k {
		top := heap.Pop(h).([3]int)
		i, j := top[1], top[2]
		result = append(result, []int{nums1[i], nums2[j]})
		if j+1 < len(nums2) {
			heap.Push(h, [3]int{nums1[i] + nums2[j+1], i, j + 1})
		}
	}
	return result
}
