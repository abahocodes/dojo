package main

func findWinners(matches [][]int) [][]int {
	maxID := 0
	for _, m := range matches {
		maxID = max(maxID, m[0], m[1])
	}
	losses := make([]int, maxID+1)
	for i := range losses {
		losses[i] = -1 // never played
	}
	for _, m := range matches {
		if losses[m[0]] < 0 {
			losses[m[0]] = 0
		}
		losses[m[1]] = max(losses[m[1]], 0) + 1
	}
	never := []int{}
	once := []int{}
	for p := 1; p <= maxID; p++ {
		if losses[p] == 0 {
			never = append(never, p)
		} else if losses[p] == 1 {
			once = append(once, p)
		}
	}
	return [][]int{never, once}
}
