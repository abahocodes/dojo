package main

func plusOne(digits []int) []int {
	// Walk from the last digit: a 9 becomes 0 and carries; anything else
	// just increments and we're done.
	result := append([]int(nil), digits...)
	for i := len(result) - 1; i >= 0; i-- {
		if result[i] < 9 {
			result[i]++
			return result
		}
		result[i] = 0
	}
	// Every digit was 9: the answer is 1 followed by zeros.
	return append([]int{1}, result...)
}
