package main

func countPalindromePaths(parent []int, s string) int {
	n := len(parent)
	children := make([][]int, n)
	for v := 1; v < n; v++ {
		children[parent[v]] = append(children[parent[v]], v)
	}
	mask := make([]int, n)
	order := make([]int, 1, n)
	for i := 0; i < len(order); i++ {
		v := order[i]
		for _, c := range children[v] {
			mask[c] = mask[v] ^ (1 << (s[c] - 'a'))
			order = append(order, c)
		}
	}
	seen := make(map[int]int, n)
	total := 0
	for v := 0; v < n; v++ {
		m := mask[v]
		total += seen[m]
		for b := 0; b < 26; b++ {
			total += seen[m^(1<<b)]
		}
		seen[m]++
	}
	return total
}
