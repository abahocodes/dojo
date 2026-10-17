// 1004. Copy List with Random Pointer (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
	return nil
}

// dojo calls this to run the tests; you don't need to change it.
func runCopyRandomList(nodes [][]int) [][]int {
	n := len(nodes)
	originals := make([]*Node, n)
	for i := range nodes {
		originals[i] = &Node{Val: nodes[i][0]}
	}
	nextOf := func(i int) *Node {
		if i+1 < n {
			return originals[i+1]
		}
		return nil
	}
	randomOf := func(r int) *Node {
		if r >= 0 {
			return originals[r]
		}
		return nil
	}
	for i := range nodes {
		originals[i].Next = nextOf(i)
		originals[i].Random = randomOf(nodes[i][1])
	}
	var head *Node
	if n > 0 {
		head = originals[0]
	}
	copy := copyRandomList(head)

	// The original list must be left as it was.
	for i, node := range originals {
		if node.Val != nodes[i][0] || node.Next != nextOf(i) || node.Random != randomOf(nodes[i][1]) {
			return [][]int{{-1}}
		}
	}
	// Walk the copy: every node must be new, and the list must end.
	originalSet := make(map[*Node]bool, n)
	for _, node := range originals {
		originalSet[node] = true
	}
	index := map[*Node]int{}
	var order []*Node
	for node := copy; node != nil; node = node.Next {
		if _, seen := index[node]; seen || originalSet[node] {
			return [][]int{{-1}}
		}
		index[node] = len(order)
		order = append(order, node)
	}
	result := [][]int{}
	for _, node := range order {
		if node.Random == nil {
			result = append(result, []int{node.Val, -1})
		} else if r, ok := index[node.Random]; ok {
			result = append(result, []int{node.Val, r})
		} else {
			return [][]int{{-1}}
		}
	}
	return result
}
