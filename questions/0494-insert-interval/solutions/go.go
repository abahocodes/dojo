package main

func insertInterval(intervals [][]int, newInterval []int) [][]int {
	result := [][]int{}
	start, end := newInterval[0], newInterval[1]
	i, n := 0, len(intervals)
	// Intervals that end strictly before the new one starts.
	for i < n && intervals[i][1] < start {
		result = append(result, intervals[i])
		i++
	}
	// Intervals that overlap or touch the new one: absorb them.
	for i < n && intervals[i][0] <= end {
		start = min(start, intervals[i][0])
		end = max(end, intervals[i][1])
		i++
	}
	result = append(result, []int{start, end})
	// Intervals that start strictly after the merged one ends.
	return append(result, intervals[i:]...)
}
