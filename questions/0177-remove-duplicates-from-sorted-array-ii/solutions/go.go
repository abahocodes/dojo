package main

func removeDuplicatesKeepTwo(nums []int) []int {
	a := append([]int(nil), nums...)
	k := 0
	for i := 0; i < len(a); i++ {
		x := a[i]
		if k < 2 || a[k-2] != x {
			a[k] = x
			k++
		}
	}
	return a[:k]
}
