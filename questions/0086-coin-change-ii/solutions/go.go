package main

func countCombinations(coins []int, amount int) int {
	// Intermediate counts can outgrow 64 bits, but sums wrap modulo 2^64,
	// so the final count (which fits in 32 bits) still comes out exact.
	ways := make([]int, amount+1)
	ways[0] = 1
	for _, c := range coins {
		for x := c; x <= amount; x++ {
			ways[x] += ways[x-c]
		}
	}
	return ways[amount]
}
