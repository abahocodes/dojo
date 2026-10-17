package main

func isHappy(n int) bool {
	// The digit-square sequence either reaches 1 or falls into a cycle.
	// Floyd's tortoise and hare finds out without remembering every value.
	step := func(v int) int {
		total := 0
		for v > 0 {
			d := v % 10
			total += d * d
			v /= 10
		}
		return total
	}
	slow, fast := n, step(n)
	for fast != 1 && slow != fast {
		slow = step(slow)
		fast = step(step(fast))
	}
	return fast == 1
}
