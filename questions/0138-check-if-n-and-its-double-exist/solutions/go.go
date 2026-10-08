package main

func checkIfExist(arr []int) bool {
	seen := map[int]bool{}
	for _, x := range arr {
		if seen[2*x] || (x%2 == 0 && seen[x/2]) {
			return true
		}
		seen[x] = true
	}
	return false
}
