package main

import "container/heap"

// intHeap is a min-heap of ints.
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

func findOrder(numCourses int, prerequisites [][]int) []int {
	unlocks := make([][]int, numCourses)
	indegree := make([]int, numCourses)
	for _, p := range prerequisites {
		unlocks[p[1]] = append(unlocks[p[1]], p[0])
		indegree[p[0]]++
	}

	// A min-heap always hands out the smallest course that is ready now,
	// which gives the lexicographically smallest valid order.
	ready := &intHeap{}
	for i := 0; i < numCourses; i++ {
		if indegree[i] == 0 {
			*ready = append(*ready, i)
		}
	}
	heap.Init(ready)
	order := make([]int, 0, numCourses)
	for ready.Len() > 0 {
		current := heap.Pop(ready).(int)
		order = append(order, current)
		for _, next := range unlocks[current] {
			indegree[next]--
			if indegree[next] == 0 {
				heap.Push(ready, next)
			}
		}
	}
	if len(order) != numCourses {
		return nil
	}
	return order
}
