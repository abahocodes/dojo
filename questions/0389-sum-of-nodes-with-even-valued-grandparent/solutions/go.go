package main

func sumEvenGrandparent(root *TreeNode) int {
	total := 0
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		for _, child := range []*TreeNode{node.Left, node.Right} {
			if child == nil {
				continue
			}
			if node.Val%2 == 0 {
				if child.Left != nil {
					total += child.Left.Val
				}
				if child.Right != nil {
					total += child.Right.Val
				}
			}
			stack = append(stack, child)
		}
	}
	return total
}
