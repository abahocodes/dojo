package main

func findKthNumber(n int, k int) int {
	// How many numbers in [1, n] start with the decimal digits of prefix.
	subtreeSize := func(prefix int) int {
		count, first, last := 0, prefix, prefix
		for first <= n {
			if last < n {
				count += last - first + 1
			} else {
				count += n - first + 1
			}
			first *= 10
			last = last*10 + 9
		}
		return count
	}

	current := 1
	k--
	for k > 0 {
		size := subtreeSize(current)
		if size <= k {
			k -= size
			current++
		} else {
			k--
			current *= 10
		}
	}
	return current
}
