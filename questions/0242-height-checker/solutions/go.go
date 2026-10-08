package main

func heightChecker(heights []int) int {
	var count [101]int
	for _, h := range heights {
		count[h]++
	}
	mismatches, expected := 0, 1
	for _, h := range heights {
		for count[expected] == 0 {
			expected++
		}
		if h != expected {
			mismatches++
		}
		count[expected]--
	}
	return mismatches
}
