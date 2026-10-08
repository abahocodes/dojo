package main

import "container/heap"

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

func furthestBuilding(heights []int, bricks int, ladders int) int {
	ladderClimbs := &intHeap{}
	for i := 0; i+1 < len(heights); i++ {
		climb := heights[i+1] - heights[i]
		if climb <= 0 {
			continue
		}
		heap.Push(ladderClimbs, climb)
		if ladderClimbs.Len() > ladders {
			bricks -= heap.Pop(ladderClimbs).(int)
			if bricks < 0 {
				return i
			}
		}
	}
	return len(heights) - 1
}
