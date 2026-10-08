package main

func numRabbits(answers []int) int {
	var counts [1000]int
	for _, x := range answers {
		counts[x]++
	}
	total := 0
	for x := 0; x < 1000; x++ {
		if counts[x] == 0 {
			continue
		}
		size := x + 1
		groups := (counts[x] + size - 1) / size
		total += groups * size
	}
	return total
}
