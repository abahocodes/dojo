package main

func reverseFrom(node *ListNode) *ListNode {
	var prev *ListNode
	for node != nil {
		nxt := node.Next
		node.Next = prev
		prev = node
		node = nxt
	}
	return prev
}

func isPalindromeList(head *ListNode) bool {
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}
	tail := reverseFrom(slow)
	ok := true
	a, b := head, tail
	for b != nil {
		if a.Val != b.Val {
			ok = false
			break
		}
		a = a.Next
		b = b.Next
	}
	reverseFrom(tail)
	return ok
}
