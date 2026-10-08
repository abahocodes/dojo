package main

import "strconv"

func binaryTreePaths(root *TreeNode) []string {
	type frame struct {
		node *TreeNode
		path string
	}
	paths := []string{}
	stack := []frame{{root, strconv.Itoa(root.Val)}}
	for len(stack) > 0 {
		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node := top.node
		if node.Left == nil && node.Right == nil {
			paths = append(paths, top.path)
			continue
		}
		if node.Right != nil {
			stack = append(stack, frame{node.Right, top.path + "->" + strconv.Itoa(node.Right.Val)})
		}
		if node.Left != nil {
			stack = append(stack, frame{node.Left, top.path + "->" + strconv.Itoa(node.Left.Val)})
		}
	}
	return paths
}
