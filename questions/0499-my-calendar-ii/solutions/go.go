package main

func bookCalendarDouble(bookings [][]int) []bool {
	var booked, overlaps [][2]int // accepted bookings; stretches already covered twice
	result := make([]bool, 0, len(bookings))
	for _, b := range bookings {
		start, end := b[0], b[1]
		ok := true
		for _, o := range overlaps {
			if max(start, o[0]) < min(end, o[1]) {
				ok = false
				break
			}
		}
		if ok {
			for _, p := range booked {
				lo, hi := max(start, p[0]), min(end, p[1])
				if lo < hi {
					overlaps = append(overlaps, [2]int{lo, hi})
				}
			}
			booked = append(booked, [2]int{start, end})
		}
		result = append(result, ok)
	}
	return result
}
