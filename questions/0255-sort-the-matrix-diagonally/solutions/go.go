package main

import "sort"

func diagonalSort(mat [][]int) [][]int {
	m, n := len(mat), len(mat[0])
	res := make([][]int, m)
	for i := range mat {
		res[i] = append([]int(nil), mat[i]...)
	}
	sortFrom := func(si, sj int) {
		length := min(m-si, n-sj)
		values := make([]int, length)
		for k := 0; k < length; k++ {
			values[k] = res[si+k][sj+k]
		}
		sort.Ints(values)
		for k := 0; k < length; k++ {
			res[si+k][sj+k] = values[k]
		}
	}
	for i := 0; i < m; i++ {
		sortFrom(i, 0)
	}
	for j := 1; j < n; j++ {
		sortFrom(0, j)
	}
	return res
}
