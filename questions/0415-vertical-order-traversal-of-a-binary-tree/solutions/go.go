package main

import "sort"

func verticalTraversal(root *TreeNode) [][]int {
	type entry struct{ col, row, val int }
	type item struct {
		node     *TreeNode
		row, col int
	}
	var entries []entry
	stack := []item{{root, 0, 0}}
	for len(stack) > 0 {
		it := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		entries = append(entries, entry{it.col, it.row, it.node.Val})
		if it.node.Left != nil {
			stack = append(stack, item{it.node.Left, it.row + 1, it.col - 1})
		}
		if it.node.Right != nil {
			stack = append(stack, item{it.node.Right, it.row + 1, it.col + 1})
		}
	}
	sort.Slice(entries, func(i, j int) bool {
		a, b := entries[i], entries[j]
		if a.col != b.col {
			return a.col < b.col
		}
		if a.row != b.row {
			return a.row < b.row
		}
		return a.val < b.val
	})
	var result [][]int
	for i, e := range entries {
		if i == 0 || e.col != entries[i-1].col {
			result = append(result, []int{})
		}
		result[len(result)-1] = append(result[len(result)-1], e.val)
	}
	return result
}
