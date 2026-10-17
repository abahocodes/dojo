package main

import "sort"

func findItinerary(tickets [][]string) []string {
	// Destinations sorted descending, so taking from the end hands out the smallest first.
	graph := map[string][]string{}
	for _, t := range tickets {
		graph[t[0]] = append(graph[t[0]], t[1])
	}
	for _, out := range graph {
		sort.Sort(sort.Reverse(sort.StringSlice(out)))
	}

	// Hierholzer: walk greedily; an airport with no tickets left is final,
	// so it goes onto the route (built back to front).
	route := []string{}
	stack := []string{"JFK"}
	for len(stack) > 0 {
		top := stack[len(stack)-1]
		if out := graph[top]; len(out) > 0 {
			graph[top] = out[:len(out)-1]
			stack = append(stack, out[len(out)-1])
		} else {
			route = append(route, top)
			stack = stack[:len(stack)-1]
		}
	}
	for i, j := 0, len(route)-1; i < j; i, j = i+1, j-1 {
		route[i], route[j] = route[j], route[i]
	}
	return route
}
