package main

import (
	"container/heap"
	"sort"
)

type readyHeap [][2]int // {processing time, index}

func (h readyHeap) Len() int { return len(h) }
func (h readyHeap) Less(a, b int) bool {
	if h[a][0] != h[b][0] {
		return h[a][0] < h[b][0]
	}
	return h[a][1] < h[b][1]
}
func (h readyHeap) Swap(a, b int) { h[a], h[b] = h[b], h[a] }
func (h *readyHeap) Push(x any)   { *h = append(*h, x.([2]int)) }
func (h *readyHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func getOrder(tasks [][]int) []int {
	n := len(tasks)
	byEnqueue := make([]int, n)
	for i := range byEnqueue {
		byEnqueue[i] = i
	}
	sort.Slice(byEnqueue, func(a, b int) bool {
		x, y := byEnqueue[a], byEnqueue[b]
		if tasks[x][0] != tasks[y][0] {
			return tasks[x][0] < tasks[y][0]
		}
		return x < y
	})
	ready := &readyHeap{}
	order := make([]int, 0, n)
	time, p := 0, 0
	for len(order) < n {
		if ready.Len() == 0 && time < tasks[byEnqueue[p]][0] {
			time = tasks[byEnqueue[p]][0]
		}
		for p < n && tasks[byEnqueue[p]][0] <= time {
			i := byEnqueue[p]
			heap.Push(ready, [2]int{tasks[i][1], i})
			p++
		}
		t := heap.Pop(ready).([2]int)
		time += t[0]
		order = append(order, t[1])
	}
	return order
}
