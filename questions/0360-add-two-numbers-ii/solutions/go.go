package main

func addTwoNumbersIi(l1 *ListNode, l2 *ListNode) *ListNode {
	var a, b []int
	for p := l1; p != nil; p = p.Next {
		a = append(a, p.Val)
	}
	for p := l2; p != nil; p = p.Next {
		b = append(b, p.Val)
	}
	var head *ListNode
	carry := 0
	for len(a) > 0 || len(b) > 0 || carry > 0 {
		s := carry
		if len(a) > 0 {
			s += a[len(a)-1]
			a = a[:len(a)-1]
		}
		if len(b) > 0 {
			s += b[len(b)-1]
			b = b[:len(b)-1]
		}
		head = &ListNode{Val: s % 10, Next: head}
		carry = s / 10
	}
	return head
}
