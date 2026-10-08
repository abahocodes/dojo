package main

func evaluateTree(root *TreeNode) bool {
	type item struct {
		node         *TreeNode
		childrenDone bool
	}
	value := map[*TreeNode]bool{}
	stack := []item{{root, false}}
	for len(stack) > 0 {
		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node := top.node
		if node.Left == nil {
			value[node] = node.Val == 1
		} else if top.childrenDone {
			left, right := value[node.Left], value[node.Right]
			if node.Val == 2 {
				value[node] = left || right
			} else {
				value[node] = left && right
			}
		} else {
			stack = append(stack, item{node, true}, item{node.Left, false}, item{node.Right, false})
		}
	}
	return value[root]
}
