package main

func findTargetSumWays(nums []int, target int) int {
	total := 0
	for _, x := range nums {
		total += x
	}
	abs := target
	if abs < 0 {
		abs = -abs
	}
	if abs > total || (total+target)%2 != 0 {
		return 0
	}
	goal := (total + target) / 2
	ways := make([]int, goal+1)
	ways[0] = 1
	for _, x := range nums {
		for s := goal; s >= x; s-- {
			ways[s] += ways[s-x]
		}
	}
	return ways[goal]
}
