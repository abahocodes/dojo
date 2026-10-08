package main

func twoSumSorted(numbers []int, target int) []int {
	lo, hi := 0, len(numbers)-1
	for lo < hi {
		total := numbers[lo] + numbers[hi]
		if total == target {
			return []int{lo + 1, hi + 1}
		}
		if total < target {
			lo++
		} else {
			hi--
		}
	}
	return []int{}
}
