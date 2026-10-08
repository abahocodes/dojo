package main

import "sort"

func carFleet(target int, position []int, speed []int) int {
	order := make([]int, len(position))
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(a, b int) bool { return position[order[a]] > position[order[b]] })
	fleets := 0
	leadDist, leadSpeed := 0, 1
	for _, i := range order {
		dist := target - position[i]
		if dist*leadSpeed > leadDist*speed[i] {
			fleets++
			leadDist, leadSpeed = dist, speed[i]
		}
	}
	return fleets
}
