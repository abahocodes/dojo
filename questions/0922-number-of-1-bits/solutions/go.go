package main

func hammingWeight(n int) int {
	count := 0
	for n != 0 {
		n &= n - 1
		count++
	}
	return count
}
