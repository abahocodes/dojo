package main

func minSubarrayRemove(nums []int, p int) int {
	need := 0
	for _, x := range nums {
		need = (need + x) % p
	}
	if need == 0 {
		return 0
	}
	latest := map[int]int{0: -1}
	cur := 0
	best := len(nums)
	for j, x := range nums {
		cur = (cur + x) % p
		want := (cur - need + p) % p
		if at, ok := latest[want]; ok && j-at < best {
			best = j - at
		}
		latest[cur] = j
	}
	if best < len(nums) {
		return best
	}
	return -1
}
