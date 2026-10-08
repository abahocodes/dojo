package main

func majorityElement(nums []int) []int {
	c1, c2, n1, n2 := 0, 1, 0, 0
	for _, x := range nums {
		switch {
		case x == c1:
			n1++
		case x == c2:
			n2++
		case n1 == 0:
			c1, n1 = x, 1
		case n2 == 0:
			c2, n2 = x, 1
		default:
			n1--
			n2--
		}
	}
	f1, f2 := 0, 0
	for _, x := range nums {
		if x == c1 {
			f1++
		} else if x == c2 {
			f2++
		}
	}
	limit := len(nums) / 3
	result := []int{}
	if f1 > limit {
		result = append(result, c1)
	}
	if f2 > limit {
		result = append(result, c2)
	}
	return result
}
