package main

func flatten(root *TreeNode) *TreeNode {
	for node := root; node != nil; node = node.Right {
		if node.Left != nil {
			// Splice the left subtree between node and its right subtree.
			tail := node.Left
			for tail.Right != nil {
				tail = tail.Right
			}
			tail.Right = node.Right
			node.Right = node.Left
			node.Left = nil
		}
	}
	return root
}
