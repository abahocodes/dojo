package main

func maxSatisfied(customers []int, grumpy []int, minutes int) int {
	base, gain, best := 0, 0, 0
	for i, c := range customers {
		if grumpy[i] == 0 {
			base += c
		} else {
			gain += c
		}
		if i >= minutes && grumpy[i-minutes] == 1 {
			gain -= customers[i-minutes]
		}
		if gain > best {
			best = gain
		}
	}
	return base + best
}
