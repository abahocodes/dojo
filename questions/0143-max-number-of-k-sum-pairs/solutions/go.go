package main

func maxOperations(nums []int, k int) int {
	waiting := map[int]int{}
	ops := 0
	for _, x := range nums {
		partner := k - x
		if waiting[partner] > 0 {
			waiting[partner]--
			ops++
		} else {
			waiting[x]++
		}
	}
	return ops
}
