package main

import (
	"strconv"
	"strings"
)

// encode writes the tree in pre-order with explicit null markers; the leading
// comma on every token keeps "2" from matching inside "12"
func encode(root *TreeNode) string {
	var sb strings.Builder
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if node == nil {
			sb.WriteString(",#")
		} else {
			sb.WriteByte(',')
			sb.WriteString(strconv.Itoa(node.Val))
			stack = append(stack, node.Right, node.Left)
		}
	}
	return sb.String()
}

func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	return strings.Contains(encode(root), encode(subRoot))
}
