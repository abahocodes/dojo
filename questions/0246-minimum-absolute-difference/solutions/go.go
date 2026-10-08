package main

import "sort"

func minimumAbsDifference(arr []int) [][]int {
	a := append([]int(nil), arr...)
	sort.Ints(a)
	best := a[1] - a[0]
	for i := 1; i+1 < len(a); i++ {
		if d := a[i+1] - a[i]; d < best {
			best = d
		}
	}
	out := [][]int{}
	for i := 0; i+1 < len(a); i++ {
		if a[i+1]-a[i] == best {
			out = append(out, []int{a[i], a[i+1]})
		}
	}
	return out
}
