package main

import "sort"

func numRescueBoats(people []int, limit int) int {
	p := append([]int(nil), people...)
	sort.Ints(p)
	i, j, boats := 0, len(p)-1, 0
	for i <= j {
		if p[i]+p[j] <= limit {
			i++
		}
		j--
		boats++
	}
	return boats
}
