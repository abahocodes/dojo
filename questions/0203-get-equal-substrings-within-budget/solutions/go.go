package main

func absDiff(a, b byte) int {
	if a > b {
		return int(a - b)
	}
	return int(b - a)
}

func equalSubstring(s string, t string, maxCost int) int {
	left, cost, best := 0, 0, 0
	for right := 0; right < len(s); right++ {
		cost += absDiff(s[right], t[right])
		for cost > maxCost {
			cost -= absDiff(s[left], t[left])
			left++
		}
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}
