package main

import (
	"sort"
	"strconv"
	"strings"
)

func countOfAtoms(formula string) string {
	n := len(formula)
	i := 0
	readNumber := func() int {
		start := i
		for i < n && formula[i] >= '0' && formula[i] <= '9' {
			i++
		}
		if i == start {
			return 1
		}
		v, _ := strconv.Atoi(formula[start:i])
		return v
	}
	stack := []map[string]int{{}} // one count map per open group
	for i < n {
		ch := formula[i]
		if ch == '(' {
			stack = append(stack, map[string]int{})
			i++
		} else if ch == ')' {
			i++
			mult := readNumber()
			group := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			top := stack[len(stack)-1]
			for name, cnt := range group {
				top[name] += cnt * mult
			}
		} else {
			start := i
			i++
			for i < n && formula[i] >= 'a' && formula[i] <= 'z' {
				i++
			}
			name := formula[start:i]
			stack[len(stack)-1][name] += readNumber()
		}
	}
	counts := stack[0]
	names := make([]string, 0, len(counts))
	for name := range counts {
		names = append(names, name)
	}
	sort.Strings(names)
	var sb strings.Builder
	for _, name := range names {
		sb.WriteString(name)
		if counts[name] > 1 {
			sb.WriteString(strconv.Itoa(counts[name]))
		}
	}
	return sb.String()
}
