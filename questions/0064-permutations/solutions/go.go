package main

func permute(nums []int) [][]int {
	var result [][]int
	current := make([]int, 0, len(nums))
	used := make([]bool, len(nums))

	var backtrack func()
	backtrack = func() {
		if len(current) == len(nums) {
			result = append(result, append([]int(nil), current...))
			return
		}
		for i, x := range nums {
			if used[i] {
				continue
			}
			used[i] = true
			current = append(current, x)
			backtrack()
			current = current[:len(current)-1]
			used[i] = false
		}
	}

	backtrack()
	return result
}
