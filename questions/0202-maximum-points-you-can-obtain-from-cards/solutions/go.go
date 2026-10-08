package main

func maxScoreCards(cardPoints []int, k int) int {
	n := len(cardPoints)
	current := 0
	for i := 0; i < k; i++ {
		current += cardPoints[i]
	}
	best := current
	for i := 1; i <= k; i++ {
		current += cardPoints[n-i] - cardPoints[k-i]
		if current > best {
			best = current
		}
	}
	return best
}
