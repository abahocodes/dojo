package main

func checkSubarraySum(nums []int, k int) bool {
	first := map[int]int{0: -1}
	rem := 0
	for i, x := range nums {
		rem = (rem + x) % k
		if j, ok := first[rem]; ok {
			if i-j >= 2 {
				return true
			}
		} else {
			first[rem] = i
		}
	}
	return false
}
