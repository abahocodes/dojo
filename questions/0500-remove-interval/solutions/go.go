package main

func removeInterval(intervals [][]int, toBeRemoved []int) [][]int {
	cutLo, cutHi := toBeRemoved[0], toBeRemoved[1]
	result := [][]int{}
	for _, iv := range intervals {
		a, b := iv[0], iv[1]
		if b <= cutLo || a >= cutHi {
			result = append(result, []int{a, b}) // untouched
			continue
		}
		if a < cutLo {
			result = append(result, []int{a, cutLo}) // piece left of the cut
		}
		if b > cutHi {
			result = append(result, []int{cutHi, b}) // piece right of the cut
		}
	}
	return result
}
