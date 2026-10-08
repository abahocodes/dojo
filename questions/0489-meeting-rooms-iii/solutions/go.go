package main

import (
	"container/heap"
	"sort"
)

// roomHeap is a min-heap of free room numbers.
type roomHeap []int

func (h roomHeap) Len() int            { return len(h) }
func (h roomHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h roomHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *roomHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *roomHeap) Pop() interface{} {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

// busyHeap holds {end time, room}, earliest end first, then lowest room.
type busyHeap [][2]int

func (h busyHeap) Len() int { return len(h) }
func (h busyHeap) Less(i, j int) bool {
	if h[i][0] != h[j][0] {
		return h[i][0] < h[j][0]
	}
	return h[i][1] < h[j][1]
}
func (h busyHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *busyHeap) Push(x interface{}) { *h = append(*h, x.([2]int)) }
func (h *busyHeap) Pop() interface{} {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func mostBooked(n int, meetings [][]int) int {
	sorted := make([][]int, len(meetings))
	copy(sorted, meetings)
	sort.Slice(sorted, func(i, j int) bool { return sorted[i][0] < sorted[j][0] })

	free := &roomHeap{}
	for r := 0; r < n; r++ {
		heap.Push(free, r)
	}
	busy := &busyHeap{}
	count := make([]int, n)

	for _, m := range sorted {
		start, end := m[0], m[1]
		for busy.Len() > 0 && (*busy)[0][0] <= start {
			heap.Push(free, heap.Pop(busy).([2]int)[1])
		}
		var room int
		if free.Len() > 0 {
			room = heap.Pop(free).(int)
			heap.Push(busy, [2]int{end, room})
		} else {
			// Wait for the earliest room; it keeps the meeting's duration.
			next := heap.Pop(busy).([2]int)
			room = next[1]
			heap.Push(busy, [2]int{next[0] + end - start, room})
		}
		count[room]++
	}

	best := 0
	for r := 1; r < n; r++ {
		if count[r] > count[best] {
			best = r
		}
	}
	return best
}
