package main

func countComponents(n int, edges [][]int) int {
	parent := make([]int, n)
	size := make([]int, n)
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

	components := n
	for _, e := range edges {
		ra, rb := find(e[0]), find(e[1])
		if ra == rb {
			continue
		}
		if size[ra] < size[rb] {
			ra, rb = rb, ra
		}
		parent[rb] = ra
		size[ra] += size[rb]
		components--
	}
	return components
}
