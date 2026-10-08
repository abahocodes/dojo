package main

import (
	"strconv"
	"strings"
)

func tree2str(root *TreeNode) string {
	// each entry is either a node to print or literal text (node == nil)
	type item struct {
		node *TreeNode
		text string
	}
	var out strings.Builder
	stack := []item{{node: root}}
	for len(stack) > 0 {
		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if top.node == nil {
			out.WriteString(top.text)
			continue
		}
		node := top.node
		out.WriteString(strconv.Itoa(node.Val))
		if node.Right != nil {
			stack = append(stack, item{text: ")"}, item{node: node.Right}, item{text: "("})
		}
		if node.Left != nil {
			stack = append(stack, item{text: ")"}, item{node: node.Left}, item{text: "("})
		} else if node.Right != nil {
			stack = append(stack, item{text: "()"})
		}
	}
	return out.String()
}
