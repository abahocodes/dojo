package main

import (
	"container/heap"
	"sort"
)

type maxHeap []int

func (h maxHeap) Len() int           { return len(h) }
func (h maxHeap) Less(a, b int) bool { return h[a] > h[b] }
func (h maxHeap) Swap(a, b int)      { h[a], h[b] = h[b], h[a] }
func (h *maxHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *maxHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func findMaximizedCapital(k int, w int, profits []int, capital []int) int {
	order := make([]int, len(profits))
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool { return capital[order[a]] < capital[order[b]] })
	affordable := &maxHeap{}
	p := 0
	for round := 0; round < k; round++ {
		for p < len(order) && capital[order[p]] <= w {
			heap.Push(affordable, profits[order[p]])
			p++
		}
		if affordable.Len() == 0 {
			break
		}
		w += heap.Pop(affordable).(int)
	}
	return w
}
