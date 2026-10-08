package main

func duplicateZeros(arr []int) []int {
	out := append([]int{}, arr...)
	n := len(out)
	// shift = number of zeros strictly before index i: out[i] lands at i + shift.
	shift := 0
	for _, v := range out {
		if v == 0 {
			shift++
		}
	}
	for i := n - 1; i >= 0; i-- {
		if out[i] == 0 {
			shift--
			if i+shift+1 < n {
				out[i+shift+1] = 0
			}
		}
		if i+shift < n {
			out[i+shift] = out[i]
		}
	}
	return out
}
