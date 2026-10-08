package main

func leastBricks(wall [][]int) int {
	seams := map[int]int{}
	best := 0
	for _, row := range wall {
		pos := 0
		for i := 0; i < len(row)-1; i++ {
			pos += row[i]
			seams[pos]++
			if seams[pos] > best {
				best = seams[pos]
			}
		}
	}
	return len(wall) - best
}
