package main

func doubleIt(head *ListNode) *ListNode {
	if head.Val >= 5 {
		head = &ListNode{Val: 0, Next: head}
	}
	for node := head; node != nil; node = node.Next {
		node.Val = (node.Val * 2) % 10
		if node.Next != nil && node.Next.Val >= 5 {
			node.Val++
		}
	}
	return head
}
