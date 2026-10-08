package main

import "sort"

func accountsMerge(accounts [][]string) [][]string {
	parent := make([]int, len(accounts))
	for i := range parent {
		parent[i] = i
	}
	find := func(x int) int {
		for parent[x] != x {
			parent[x] = parent[parent[x]]
			x = parent[x]
		}
		return x
	}

	owner := map[string]int{} // email -> index of the first account that listed it
	for i, account := range accounts {
		for _, email := range account[1:] {
			if j, ok := owner[email]; ok {
				ri, rj := find(i), find(j)
				if ri != rj {
					parent[ri] = rj
				}
			} else {
				owner[email] = i
			}
		}
	}

	groups := map[int][]string{}
	for email, i := range owner {
		root := find(i)
		groups[root] = append(groups[root], email)
	}

	merged := make([][]string, 0, len(groups))
	for root, emails := range groups {
		sort.Strings(emails)
		merged = append(merged, append([]string{accounts[root][0]}, emails...))
	}
	sort.Slice(merged, func(a, b int) bool {
		if merged[a][0] != merged[b][0] {
			return merged[a][0] < merged[b][0]
		}
		return merged[a][1] < merged[b][1]
	})
	return merged
}
