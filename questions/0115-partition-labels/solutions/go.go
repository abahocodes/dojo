package main

func partitionLabels(s string) []int {
	last := make(map[byte]int)
	for i := 0; i < len(s); i++ {
		last[s[i]] = i
	}
	var sizes []int
	start, end := 0, 0
	for i := 0; i < len(s); i++ {
		end = max(end, last[s[i]])
		if i == end {
			sizes = append(sizes, i-start+1)
			start = i + 1
		}
	}
	return sizes
}
