package main

func getDecimalValue(head *ListNode) int {
	value := 0
	for head != nil {
		value = value<<1 | head.Val
		head = head.Next
	}
	return value
}
