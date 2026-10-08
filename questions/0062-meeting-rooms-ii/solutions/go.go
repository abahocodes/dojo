package main

import "sort"

func minMeetingRooms(intervals [][]int) int {
	starts := make([]int, len(intervals))
	ends := make([]int, len(intervals))
	for i, iv := range intervals {
		starts[i], ends[i] = iv[0], iv[1]
	}
	sort.Ints(starts)
	sort.Ints(ends)
	rooms := 0
	j := 0 // ends[j] is the earliest end time that hasn't freed a room yet
	for _, s := range starts {
		if s >= ends[j] {
			j++
		} else {
			rooms++
		}
	}
	return rooms
}
