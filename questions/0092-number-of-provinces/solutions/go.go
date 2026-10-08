package main

func findCircleNum(isConnected [][]int) int {
	n := len(isConnected)
	parent := make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	find := func(x int) int {
		for parent[x] != x {
			parent[x] = parent[parent[x]]
			x = parent[x]
		}
		return x
	}
	provinces := n
	for i := 0; i < n; i++ {
		row := isConnected[i]
		for j := i + 1; j < n; j++ {
			if row[j] == 1 {
				ri, rj := find(i), find(j)
				if ri != rj {
					parent[ri] = rj
					provinces--
				}
			}
		}
	}
	return provinces
}
