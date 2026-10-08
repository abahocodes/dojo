package main

func findLucky(arr []int) int {
	var count [501]int
	for _, x := range arr {
		count[x]++
	}
	for v := 500; v >= 1; v-- {
		if count[v] == v {
			return v
		}
	}
	return -1
}
