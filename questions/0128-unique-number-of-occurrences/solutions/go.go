package main

func uniqueOccurrences(arr []int) bool {
	count := make(map[int]int)
	for _, x := range arr {
		count[x]++
	}
	seen := make(map[int]bool)
	for _, c := range count {
		if seen[c] {
			return false
		}
		seen[c] = true
	}
	return true
}
