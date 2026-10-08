package main

func removeNodes(head *ListNode) *ListNode {
	stack := []*ListNode{}
	for node := head; node != nil; node = node.Next {
		for len(stack) > 0 && stack[len(stack)-1].Val < node.Val {
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, node)
	}
	for i := 0; i+1 < len(stack); i++ {
		stack[i].Next = stack[i+1]
	}
	stack[len(stack)-1].Next = nil
	return stack[0]
}
