package main

func hasGroupsSizeX(deck []int) bool {
	count := make([]int, 10000)
	for _, v := range deck {
		count[v]++
	}
	g := 0
	for _, c := range count {
		if c > 0 {
			a, b := g, c
			for b != 0 {
				a, b = b, a%b
			}
			g = a
		}
	}
	return g >= 2
}
