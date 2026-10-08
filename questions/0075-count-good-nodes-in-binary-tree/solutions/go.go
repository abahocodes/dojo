package main

func goodNodes(root *TreeNode) int {
	if root == nil {
		return 0
	}
	type frame struct {
		node *TreeNode
		best int // largest value strictly above it on the path
	}
	count := 0
	stack := []frame{{root, root.Val}}
	for len(stack) > 0 {
		f := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node, best := f.node, f.best
		if node.Val >= best {
			count++
			best = node.Val
		}
		if node.Left != nil {
			stack = append(stack, frame{node.Left, best})
		}
		if node.Right != nil {
			stack = append(stack, frame{node.Right, best})
		}
	}
	return count
}
