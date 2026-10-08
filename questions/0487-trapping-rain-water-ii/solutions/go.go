package main

import "container/heap"

// cellHeap is a min-heap of {level, row, col}, ordered by level.
type cellHeap [][3]int

func (h cellHeap) Len() int            { return len(h) }
func (h cellHeap) Less(i, j int) bool  { return h[i][0] < h[j][0] }
func (h cellHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *cellHeap) Push(x interface{}) { *h = append(*h, x.([3]int)) }
func (h *cellHeap) Pop() interface{} {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func trapRainWater2d(heightMap [][]int) int {
	m, n := len(heightMap), len(heightMap[0])
	visited := make([][]bool, m)
	for r := range visited {
		visited[r] = make([]bool, n)
	}
	h := &cellHeap{}
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if r == 0 || c == 0 || r == m-1 || c == n-1 {
				heap.Push(h, [3]int{heightMap[r][c], r, c})
				visited[r][c] = true
			}
		}
	}

	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	total := 0
	for h.Len() > 0 {
		cell := heap.Pop(h).([3]int)
		level := cell[0]
		for _, d := range dirs {
			nr, nc := cell[1]+d[0], cell[2]+d[1]
			if nr < 0 || nc < 0 || nr >= m || nc >= n || visited[nr][nc] {
				continue
			}
			visited[nr][nc] = true
			height := heightMap[nr][nc]
			next := level
			if height < level {
				total += level - height
			} else {
				next = height
			}
			heap.Push(h, [3]int{next, nr, nc})
		}
	}
	return total
}
