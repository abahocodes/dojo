package main

func findDuplicateSubtrees(root *TreeNode) []*TreeNode {
	ids := map[[3]int]int{}             // (left id, value, right id) -> subtree id
	count := map[int]int{}              // subtree id -> occurrences
	nodeID := map[*TreeNode]int{nil: 0} // node -> subtree id (0 means empty)
	var result []*TreeNode
	type frame struct {
		node *TreeNode
		done bool
	}
	stack := []frame{{root, false}}
	for len(stack) > 0 {
		f := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node := f.node
		if node == nil {
			continue
		}
		if !f.done {
			stack = append(stack, frame{node, true}, frame{node.Right, false}, frame{node.Left, false})
			continue
		}
		key := [3]int{nodeID[node.Left], node.Val, nodeID[node.Right]}
		sid, ok := ids[key]
		if !ok {
			sid = len(ids) + 1
			ids[key] = sid
		}
		nodeID[node] = sid
		count[sid]++
		if count[sid] == 2 {
			result = append(result, node)
		}
	}
	return result
}
