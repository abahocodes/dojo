package main

func climbStairs(n int) int {
	prev, curr := 1, 1 // ways to reach step 0 and step 1
	for i := 0; i < n-1; i++ {
		prev, curr = curr, prev+curr
	}
	return curr
}
