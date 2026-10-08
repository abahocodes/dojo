package main

func leastInterval(tasks []string, n int) int {
	counts := map[string]int{}
	for _, t := range tasks {
		counts[t]++
	}
	most := 0
	for _, c := range counts {
		most = max(most, c)
	}
	tied := 0
	for _, c := range counts {
		if c == most {
			tied++
		}
	}
	// most - 1 full rows of width n + 1, then one slot per label tied for the top count
	return max(len(tasks), (most-1)*(n+1)+tied)
}
