package main

import "sort"

func isNStraightHand(hand []int, groupSize int) bool {
	if len(hand)%groupSize != 0 {
		return false
	}
	count := make(map[int]int)
	for _, x := range hand {
		count[x]++
	}
	keys := make([]int, 0, len(count))
	for x := range count {
		keys = append(keys, x)
	}
	sort.Ints(keys)
	for _, x := range keys {
		c := count[x]
		if c == 0 {
			continue
		}
		for v := x; v < x+groupSize; v++ {
			if count[v] < c {
				return false
			}
			count[v] -= c
		}
	}
	return true
}
