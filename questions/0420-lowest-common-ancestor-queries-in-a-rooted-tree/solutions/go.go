package main

func lcaQueries(parent []int, queries [][]int) []int {
	n := len(parent)
	children := make([][]int, n)
	root := 0
	for v, p := range parent {
		if p == -1 {
			root = v
		} else {
			children[p] = append(children[p], v)
		}
	}

	depth := make([]int, n)
	order := make([]int, 0, n)
	order = append(order, root)
	for head := 0; head < len(order); head++ {
		u := order[head]
		for _, c := range children[u] {
			depth[c] = depth[u] + 1
			order = append(order, c)
		}
	}

	log := 1
	for (1 << log) < n {
		log++
	}
	up := make([][]int, log)
	up[0] = make([]int, n)
	for v, p := range parent {
		if p == -1 {
			up[0][v] = root
		} else {
			up[0][v] = p
		}
	}
	for k := 1; k < log; k++ {
		up[k] = make([]int, n)
		for v := 0; v < n; v++ {
			up[k][v] = up[k-1][up[k-1][v]]
		}
	}

	answers := make([]int, len(queries))
	for i, q := range queries {
		u, v := q[0], q[1]
		if depth[u] < depth[v] {
			u, v = v, u
		}
		diff := depth[u] - depth[v]
		for k := 0; diff > 0; k, diff = k+1, diff>>1 {
			if diff&1 == 1 {
				u = up[k][u]
			}
		}
		if u != v {
			for k := log - 1; k >= 0; k-- {
				if up[k][u] != up[k][v] {
					u = up[k][u]
					v = up[k][v]
				}
			}
			u = up[0][u]
		}
		answers[i] = u
	}
	return answers
}
