// 1010. Clone Graph (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

import "sort"

type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	return nil
}

// dojo calls this to run the tests; you don't need to change it.
func runCloneGraph(adjList [][]int) [][]int {
	n := len(adjList)
	originals := make([]*Node, n)
	for i := range originals {
		originals[i] = &Node{Val: i + 1}
	}
	for i, nbrs := range adjList {
		for _, v := range nbrs {
			originals[i].Neighbors = append(originals[i].Neighbors, originals[v-1])
		}
	}
	var start *Node
	if n > 0 {
		start = originals[0]
	}
	clone := cloneGraph(start)

	// The original graph must be left as it was.
	for i, node := range originals {
		if node.Val != i+1 || len(node.Neighbors) != len(adjList[i]) {
			return [][]int{{-1}}
		}
		for j, v := range adjList[i] {
			if node.Neighbors[j] != originals[v-1] {
				return [][]int{{-1}}
			}
		}
	}
	if clone == nil {
		return [][]int{}
	}
	// Explore the clone: no original node, one node per value 1..n.
	originalSet := make(map[*Node]bool, n)
	for _, node := range originals {
		originalSet[node] = true
	}
	seen := map[*Node]bool{}
	byVal := make([]*Node, n+1)
	stack := []*Node{clone}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if node == nil || originalSet[node] {
			return [][]int{{-1}}
		}
		if seen[node] {
			continue
		}
		seen[node] = true
		if node.Val < 1 || node.Val > n || byVal[node.Val] != nil {
			return [][]int{{-1}}
		}
		byVal[node.Val] = node
		stack = append(stack, node.Neighbors...)
	}
	result := make([][]int, n)
	for v := 1; v <= n; v++ {
		result[v-1] = []int{}
		if byVal[v] == nil {
			continue
		}
		for _, nb := range byVal[v].Neighbors {
			result[v-1] = append(result[v-1], nb.Val)
		}
		sort.Ints(result[v-1])
	}
	return result
}
