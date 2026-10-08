package main

func numJewelsInStones(jewels string, stones string) int {
	var kinds [128]bool
	for i := 0; i < len(jewels); i++ {
		kinds[jewels[i]] = true
	}
	count := 0
	for i := 0; i < len(stones); i++ {
		if kinds[stones[i]] {
			count++
		}
	}
	return count
}
