package main

func hasPathSum(root *TreeNode, targetSum int) bool {
	if root == nil {
		return false
	}
	type frame struct {
		node      *TreeNode
		remaining int
	}
	stack := []frame{{root, targetSum - root.Val}}
	for len(stack) > 0 {
		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node := top.node
		if node.Left == nil && node.Right == nil {
			if top.remaining == 0 {
				return true
			}
			continue
		}
		if node.Left != nil {
			stack = append(stack, frame{node.Left, top.remaining - node.Left.Val})
		}
		if node.Right != nil {
			stack = append(stack, frame{node.Right, top.remaining - node.Right.Val})
		}
	}
	return false
}
