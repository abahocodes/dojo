package main

func relativeSortArray(arr1 []int, arr2 []int) []int {
	var count [1001]int
	for _, x := range arr1 {
		count[x]++
	}
	result := make([]int, 0, len(arr1))
	for _, x := range arr2 {
		for ; count[x] > 0; count[x]-- {
			result = append(result, x)
		}
	}
	for x := 0; x <= 1000; x++ {
		for ; count[x] > 0; count[x]-- {
			result = append(result, x)
		}
	}
	return result
}
