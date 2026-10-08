package main

import "strconv"

func evalRpn(tokens []string) int {
	stack := []int{}
	for _, t := range tokens {
		switch t {
		case "+", "-", "*", "/":
			b := stack[len(stack)-1]
			a := stack[len(stack)-2]
			stack = stack[:len(stack)-2]
			switch t {
			case "+":
				stack = append(stack, a+b)
			case "-":
				stack = append(stack, a-b)
			case "*":
				stack = append(stack, a*b)
			default:
				stack = append(stack, a/b)
			}
		default:
			v, _ := strconv.Atoi(t)
			stack = append(stack, v)
		}
	}
	return stack[0]
}
