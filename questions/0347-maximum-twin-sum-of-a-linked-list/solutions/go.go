package main

func pairSum(head *ListNode) int {
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}
	var prev *ListNode
	for slow != nil {
		nxt := slow.Next
		slow.Next = prev
		prev = slow
		slow = nxt
	}
	best := 0
	a, b := head, prev
	for b != nil {
		if s := a.Val + b.Val; s > best {
			best = s
		}
		a = a.Next
		b = b.Next
	}
	return best
}
