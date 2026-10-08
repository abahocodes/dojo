package main

func sortArray(nums []int) []int {
	n := len(nums)
	a := append([]int(nil), nums...)
	buf := make([]int, n)
	for width := 1; width < n; width *= 2 {
		for lo := 0; lo < n; lo += 2 * width {
			mid := min(lo+width, n)
			hi := min(lo+2*width, n)
			i, j, k := lo, mid, lo
			for i < mid && j < hi {
				if a[i] <= a[j] {
					buf[k] = a[i]
					i++
				} else {
					buf[k] = a[j]
					j++
				}
				k++
			}
			k += copy(buf[k:], a[i:mid])
			copy(buf[k:], a[j:hi])
		}
		a, buf = buf, a
	}
	return a
}
