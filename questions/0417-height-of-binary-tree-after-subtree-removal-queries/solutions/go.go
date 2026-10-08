package main

func treeQueries(root *TreeNode, queries []int) []int {
	type item struct {
		node *TreeNode
		d    int
	}
	var order []item
	stack := []item{{root, 0}}
	for len(stack) > 0 {
		it := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		order = append(order, it)
		if it.node.Left != nil {
			stack = append(stack, item{it.node.Left, it.d + 1})
		}
		if it.node.Right != nil {
			stack = append(stack, item{it.node.Right, it.d + 1})
		}
	}
	n := len(order)
	depth := make([]int, n+1)
	height := make([]int, n+1)
	levels := 0
	for i := n - 1; i >= 0; i-- {
		node, d := order[i].node, order[i].d
		depth[node.Val] = d
		if d+1 > levels {
			levels = d + 1
		}
		h := 0
		if node.Left != nil {
			h = height[node.Left.Val] + 1
		}
		if node.Right != nil && height[node.Right.Val]+1 > h {
			h = height[node.Right.Val] + 1
		}
		height[node.Val] = h
	}
	best1 := make([]int, levels)
	best2 := make([]int, levels)
	owner := make([]int, levels)
	for i := range best1 {
		best1[i], best2[i] = -1, -1
	}
	for v := 1; v <= n; v++ {
		d := depth[v]
		reach := d + height[v]
		if reach > best1[d] {
			best2[d] = best1[d]
			best1[d] = reach
			owner[d] = v
		} else if reach > best2[d] {
			best2[d] = reach
		}
	}
	answer := make([]int, len(queries))
	for i, q := range queries {
		d := depth[q]
		switch {
		case owner[d] != q:
			answer[i] = best1[d]
		case best2[d] >= 0:
			answer[i] = best2[d]
		default:
			answer[i] = d - 1
		}
	}
	return answer
}
