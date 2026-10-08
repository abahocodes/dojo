package main

func canPartition(nums []int) bool {
	total := 0
	for _, x := range nums {
		total += x
	}
	if total%2 != 0 {
		return false
	}
	half := total / 2
	// bit s of reachable is set when some subset weighs exactly s
	reachable := make([]uint64, half/64+1)
	reachable[0] = 1
	for _, x := range nums {
		// reachable |= reachable << x, word by word from the top down
		q, r := x/64, uint(x%64)
		for i := len(reachable) - 1; i >= q; i-- {
			shifted := reachable[i-q] << r
			if r > 0 && i-q-1 >= 0 {
				shifted |= reachable[i-q-1] >> (64 - r)
			}
			reachable[i] |= shifted
		}
	}
	return reachable[half/64]>>(half%64)&1 == 1
}
