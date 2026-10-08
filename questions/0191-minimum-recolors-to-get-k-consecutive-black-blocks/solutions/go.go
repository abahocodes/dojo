package main

func minimumRecolors(blocks string, k int) int {
	whites := 0
	for i := 0; i < k; i++ {
		if blocks[i] == 'W' {
			whites++
		}
	}
	best := whites
	for i := k; i < len(blocks); i++ {
		if blocks[i] == 'W' {
			whites++
		}
		if blocks[i-k] == 'W' {
			whites--
		}
		if whites < best {
			best = whites
		}
	}
	return best
}
