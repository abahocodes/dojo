package main

import "container/heap"

// cellHeap is a min-heap of [height, row, col] ordered by height.
type cellHeap [][3]int

func (h cellHeap) Len() int           { return len(h) }
func (h cellHeap) Less(i, j int) bool { return h[i][0] < h[j][0] }
func (h cellHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *cellHeap) Push(x any)        { *h = append(*h, x.([3]int)) }
func (h *cellHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}

func swimInWater(grid [][]int) int {
	n := len(grid)
	seen := make([][]bool, n)
	for i := range seen {
		seen[i] = make([]bool, n)
	}
	seen[0][0] = true
	h := &cellHeap{{grid[0][0], 0, 0}}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	level := 0
	for h.Len() > 0 {
		top := heap.Pop(h).([3]int)
		r, c := top[1], top[2]
		level = max(level, top[0])
		if r == n-1 && c == n-1 {
			return level
		}
		for _, d := range dirs {
			nr, nc := r+d[0], c+d[1]
			if nr >= 0 && nr < n && nc >= 0 && nc < n && !seen[nr][nc] {
				seen[nr][nc] = true
				heap.Push(h, [3]int{grid[nr][nc], nr, nc})
			}
		}
	}
	return level
}
