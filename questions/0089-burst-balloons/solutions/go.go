package main

func maxCoins(nums []int) int {
	v := make([]int, 0, len(nums)+2)
	v = append(v, 1)
	v = append(v, nums...)
	v = append(v, 1)
	n := len(v)
	// best[i][j] is the most coins from bursting everything strictly between i and j
	best := make([][]int, n)
	for i := range best {
		best[i] = make([]int, n)
	}
	for length := 2; length < n; length++ {
		for i := 0; i+length < n; i++ {
			j := i + length
			edge := v[i] * v[j]
			top := 0
			for k := i + 1; k < j; k++ {
				top = max(top, best[i][k]+edge*v[k]+best[k][j])
			}
			best[i][j] = top
		}
	}
	return best[0][n-1]
}
