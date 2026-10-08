package main

func removeElement(nums []int, val int) []int {
	out := make([]int, 0, len(nums))
	out = append(out, nums...)
	write := 0
	for _, x := range nums {
		if x != val {
			out[write] = x
			write++
		}
	}
	return out[:write]
}
