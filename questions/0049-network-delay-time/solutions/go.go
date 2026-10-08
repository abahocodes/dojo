package main

import "container/heap"

// timeHeap is a min-heap of {time, node} pairs ordered by time.
type timeHeap [][2]int

func (h timeHeap) Len() int           { return len(h) }
func (h timeHeap) Less(i, j int) bool { return h[i][0] < h[j][0] }
func (h timeHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *timeHeap) Push(x any)        { *h = append(*h, x.([2]int)) }
func (h *timeHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}

func networkDelayTime(times [][]int, n int, k int) int {
	graph := make([][][2]int, n+1)
	for _, t := range times {
		graph[t[0]] = append(graph[t[0]], [2]int{t[1], t[2]})
	}

	dist := make([]int, n+1)
	for i := range dist {
		dist[i] = -1
	}
	h := &timeHeap{{0, k}}
	for h.Len() > 0 {
		top := heap.Pop(h).([2]int)
		d, node := top[0], top[1]
		if dist[node] != -1 {
			continue // stale entry: already settled with a smaller time
		}
		dist[node] = d
		for _, edge := range graph[node] {
			if dist[edge[0]] == -1 {
				heap.Push(h, [2]int{d + edge[1], edge[0]})
			}
		}
	}

	best := 0
	for i := 1; i <= n; i++ {
		if dist[i] == -1 {
			return -1
		}
		best = max(best, dist[i])
	}
	return best
}
