package main

func findOriginalArray(changed []int) []int {
	if len(changed)%2 != 0 {
		return []int{}
	}
	top := 0
	for _, v := range changed {
		top = max(top, v)
	}
	count := make([]int, top+1)
	for _, v := range changed {
		count[v]++
	}
	if count[0]%2 != 0 {
		return []int{}
	}
	original := make([]int, count[0]/2, len(changed)/2)
	for x := 1; x <= top; x++ {
		c := count[x]
		if c == 0 {
			continue
		}
		if 2*x > top || count[2*x] < c {
			return []int{}
		}
		count[2*x] -= c
		for k := 0; k < c; k++ {
			original = append(original, x)
		}
	}
	return original
}
