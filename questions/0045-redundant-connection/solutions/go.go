package main

func findRedundantConnection(edges [][]int) []int {
	n := len(edges)
	parent := make([]int, n+1)
	size := make([]int, n+1)
	for i := range parent {
		parent[i] = i
		size[i] = 1
	}

	find := func(x int) int {
		for parent[x] != x {
			parent[x] = parent[parent[x]] // path halving
			x = parent[x]
		}
		return x
	}

	for _, e := range edges {
		ra, rb := find(e[0]), find(e[1])
		if ra == rb {
			// a and b were already connected: this edge closes the cycle.
			return []int{e[0], e[1]}
		}
		if size[ra] < size[rb] {
			ra, rb = rb, ra
		}
		parent[rb] = ra
		size[ra] += size[rb]
	}
	return nil
}
