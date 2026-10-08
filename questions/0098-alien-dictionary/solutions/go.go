package main

import "strings"

func alienOrder(words []string) string {
	var present [26]bool
	for _, w := range words {
		for i := 0; i < len(w); i++ {
			present[w[i]-'a'] = true
		}
	}
	var after [26][26]bool
	var indegree [26]int
	for k := 0; k+1 < len(words); k++ {
		first, second := words[k], words[k+1]
		n := min(len(first), len(second))
		differs := false
		for i := 0; i < n; i++ {
			a, b := first[i]-'a', second[i]-'a'
			if a != b {
				if !after[a][b] {
					after[a][b] = true
					indegree[b]++
				}
				differs = true
				break
			}
		}
		if !differs && len(first) > len(second) {
			return "" // a longer word sits before its own prefix
		}
	}
	// Only 26 letters, so picking the smallest ready letter by scanning stands in for a heap.
	var ready [26]bool
	total := 0
	for ch := 0; ch < 26; ch++ {
		if present[ch] {
			total++
			ready[ch] = indegree[ch] == 0
		}
	}
	var order strings.Builder
	for {
		ch := -1
		for i := 0; i < 26; i++ {
			if ready[i] {
				ch = i
				break
			}
		}
		if ch < 0 {
			break
		}
		ready[ch] = false
		order.WriteByte(byte('a' + ch))
		for nxt := 0; nxt < 26; nxt++ {
			if after[ch][nxt] {
				indegree[nxt]--
				if indegree[nxt] == 0 {
					ready[nxt] = true
				}
			}
		}
	}
	if order.Len() != total {
		return ""
	}
	return order.String()
}
