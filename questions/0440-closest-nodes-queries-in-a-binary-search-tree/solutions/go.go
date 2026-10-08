package main

import "sort"

func closestNodes(root *TreeNode, queries []int) [][]int {
	var values []int
	var stack []*TreeNode
	node := root
	for len(stack) > 0 || node != nil {
		for node != nil {
			stack = append(stack, node)
			node = node.Left
		}
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		values = append(values, node.Val)
		node = node.Right
	}
	answer := make([][]int, 0, len(queries))
	for _, q := range queries {
		i := sort.SearchInts(values, q)
		if i < len(values) && values[i] == q {
			answer = append(answer, []int{q, q})
			continue
		}
		floor, ceil := -1, -1
		if i > 0 {
			floor = values[i-1]
		}
		if i < len(values) {
			ceil = values[i]
		}
		answer = append(answer, []int{floor, ceil})
	}
	return answer
}
