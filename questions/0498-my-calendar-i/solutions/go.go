package main

import "sort"

func bookCalendar(bookings [][]int) []bool {
	// Accepted bookings are disjoint, so sorting by start also sorts the ends.
	starts, ends := []int{}, []int{}
	result := make([]bool, 0, len(bookings))
	for _, b := range bookings {
		start, end := b[0], b[1]
		// i = number of accepted bookings that begin before `end`.
		i := sort.SearchInts(starts, end)
		if i > 0 && ends[i-1] > start {
			result = append(result, false)
			continue
		}
		starts = append(starts, 0)
		copy(starts[i+1:], starts[i:])
		starts[i] = start
		ends = append(ends, 0)
		copy(ends[i+1:], ends[i:])
		ends[i] = end
		result = append(result, true)
	}
	return result
}
