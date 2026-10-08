package main

import "container/heap"

type entry struct{ value, list, pos int }

type entryHeap []entry

func (h entryHeap) Len() int           { return len(h) }
func (h entryHeap) Less(a, b int) bool { return h[a].value < h[b].value }
func (h entryHeap) Swap(a, b int)      { h[a], h[b] = h[b], h[a] }
func (h *entryHeap) Push(x any)        { *h = append(*h, x.(entry)) }
func (h *entryHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func smallestRange(nums [][]int) []int {
	h := &entryHeap{}
	high := nums[0][0]
	for r, lst := range nums {
		*h = append(*h, entry{lst[0], r, 0})
		if lst[0] > high {
			high = lst[0]
		}
	}
	heap.Init(h)
	best := []int{(*h)[0].value, high}
	for {
		e := heap.Pop(h).(entry)
		if high-e.value < best[1]-best[0] {
			best = []int{e.value, high}
		}
		if e.pos+1 == len(nums[e.list]) {
			return best
		}
		next := nums[e.list][e.pos+1]
		if next > high {
			high = next
		}
		heap.Push(h, entry{next, e.list, e.pos + 1})
	}
}
