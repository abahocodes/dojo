package main

func wonderfulSubstrings(word string) int {
	var seen [1024]int
	seen[0] = 1
	mask := 0
	total := 0
	for i := 0; i < len(word); i++ {
		mask ^= 1 << (word[i] - 'a')
		total += seen[mask]
		for k := 0; k < 10; k++ {
			total += seen[mask^(1<<k)]
		}
		seen[mask]++
	}
	return total
}
