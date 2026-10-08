package main

func numEquivDominoPairs(dominoes [][]int) int {
	var seen [100]int
	pairs := 0
	for _, d := range dominoes {
		a, b := d[0], d[1]
		if a > b {
			a, b = b, a
		}
		key := 10*a + b
		pairs += seen[key]
		seen[key]++
	}
	return pairs
}
