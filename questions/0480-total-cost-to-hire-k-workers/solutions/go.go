package main

import "container/heap"

type workerHeap [][2]int // {cost, index}

func (h workerHeap) Len() int { return len(h) }
func (h workerHeap) Less(a, b int) bool {
	if h[a][0] != h[b][0] {
		return h[a][0] < h[b][0]
	}
	return h[a][1] < h[b][1]
}
func (h workerHeap) Swap(a, b int) { h[a], h[b] = h[b], h[a] }
func (h *workerHeap) Push(x any)   { *h = append(*h, x.([2]int)) }
func (h *workerHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func totalCost(costs []int, k int, candidates int) int {
	left, right := &workerHeap{}, &workerHeap{}
	i, j := 0, len(costs)-1
	total := 0
	for round := 0; round < k; round++ {
		for left.Len() < candidates && i <= j {
			heap.Push(left, [2]int{costs[i], i})
			i++
		}
		for right.Len() < candidates && i <= j {
			heap.Push(right, [2]int{costs[j], j})
			j--
		}
		takeLeft := right.Len() == 0
		if !takeLeft && left.Len() > 0 {
			l, r := (*left)[0], (*right)[0]
			takeLeft = l[0] < r[0] || (l[0] == r[0] && l[1] < r[1])
		}
		if takeLeft {
			total += heap.Pop(left).([2]int)[0]
		} else {
			total += heap.Pop(right).([2]int)[0]
		}
	}
	return total
}
