package main

func intervalIntersection(first [][]int, second [][]int) [][]int {
	result := [][]int{}
	i, j := 0, 0
	for i < len(first) && j < len(second) {
		lo := max(first[i][0], second[j][0])
		hi := min(first[i][1], second[j][1])
		if lo <= hi {
			result = append(result, []int{lo, hi})
		}
		// The interval that ends first cannot meet anything later in the other list.
		if first[i][1] < second[j][1] {
			i++
		} else {
			j++
		}
	}
	return result
}
