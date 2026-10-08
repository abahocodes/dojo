package main

import "container/heap"

// item is a server: key is its free time (busy heap) or 0 (free heap).
type item struct{ key, weight, index int }

type itemHeap []item

func (h itemHeap) Len() int { return len(h) }
func (h itemHeap) Less(a, b int) bool {
	if h[a].key != h[b].key {
		return h[a].key < h[b].key
	}
	if h[a].weight != h[b].weight {
		return h[a].weight < h[b].weight
	}
	return h[a].index < h[b].index
}
func (h itemHeap) Swap(a, b int) { h[a], h[b] = h[b], h[a] }
func (h *itemHeap) Push(x any)   { *h = append(*h, x.(item)) }
func (h *itemHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func assignTasks(servers []int, tasks []int) []int {
	free := &itemHeap{}
	for i, w := range servers {
		*free = append(*free, item{0, w, i})
	}
	heap.Init(free)
	busy := &itemHeap{}
	result := make([]int, len(tasks))
	time := 0
	for j, d := range tasks {
		if j > time {
			time = j
		}
		if free.Len() == 0 && (*busy)[0].key > time {
			time = (*busy)[0].key
		}
		for busy.Len() > 0 && (*busy)[0].key <= time {
			s := heap.Pop(busy).(item)
			heap.Push(free, item{0, s.weight, s.index})
		}
		s := heap.Pop(free).(item)
		result[j] = s.index
		heap.Push(busy, item{time + d, s.weight, s.index})
	}
	return result
}
