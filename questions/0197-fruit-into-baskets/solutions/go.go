package main

func totalFruit(fruits []int) int {
	count := map[int]int{}
	left, best := 0, 0
	for right, f := range fruits {
		count[f]++
		for len(count) > 2 {
			g := fruits[left]
			count[g]--
			if count[g] == 0 {
				delete(count, g)
			}
			left++
		}
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}
