package main

func kthSmallestPrimeFraction(arr []int, k int) []int {
	n := len(arr)
	lo, hi := 0.0, 1.0
	for {
		mid := (lo + hi) / 2
		count, p, q, i := 0, 0, 1, 0
		for j := 1; j < n; j++ {
			for i < j && float64(arr[i]) < mid*float64(arr[j]) {
				i++
			}
			count += i
			if i > 0 && arr[i-1]*q > p*arr[j] {
				p, q = arr[i-1], arr[j]
			}
		}
		if count == k {
			return []int{p, q}
		}
		if count < k {
			lo = mid
		} else {
			hi = mid
		}
	}
}
