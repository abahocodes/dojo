package main

import (
	"strconv"
	"strings"
)

func exclusiveTime(n int, logs []string) []int {
	result := make([]int, n)
	stack := make([]int, 0, len(logs))
	prev := 0
	for _, entry := range logs {
		parts := strings.Split(entry, ":")
		id, _ := strconv.Atoi(parts[0])
		t, _ := strconv.Atoi(parts[2])
		if parts[1] == "start" {
			if len(stack) > 0 {
				result[stack[len(stack)-1]] += t - prev
			}
			stack = append(stack, id)
			prev = t
		} else {
			result[stack[len(stack)-1]] += t - prev + 1
			stack = stack[:len(stack)-1]
			prev = t + 1
		}
	}
	return result
}
