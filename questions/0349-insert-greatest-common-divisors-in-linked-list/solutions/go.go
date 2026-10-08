package main

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func insertGcds(head *ListNode) *ListNode {
	cur := head
	for cur.Next != nil {
		nxt := cur.Next
		cur.Next = &ListNode{Val: gcd(cur.Val, nxt.Val), Next: nxt}
		cur = nxt
	}
	return head
}
