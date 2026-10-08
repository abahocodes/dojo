package main

func corpFlightBookings(bookings [][]int, n int) []int {
	diff := make([]int, n+1)
	for _, b := range bookings {
		diff[b[0]-1] += b[2]
		diff[b[1]] -= b[2]
	}
	totals := make([]int, n)
	running := 0
	for i := 0; i < n; i++ {
		running += diff[i]
		totals[i] = running
	}
	return totals
}
