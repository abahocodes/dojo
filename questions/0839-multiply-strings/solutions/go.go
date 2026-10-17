package main

import "strings"

func multiply(num1 string, num2 string) string {
	// Schoolbook multiplication: digit i of num1 times digit j of num2 lands
	// in positions i + j and i + j + 1 of the product (counting from the left).
	if num1 == "0" || num2 == "0" {
		return "0"
	}
	m, n := len(num1), len(num2)
	out := make([]int, m+n)
	for i := m - 1; i >= 0; i-- {
		a := int(num1[i] - '0')
		for j := n - 1; j >= 0; j-- {
			total := a*int(num2[j]-'0') + out[i+j+1]
			out[i+j+1] = total % 10
			out[i+j] += total / 10
		}
	}
	start := 0
	for start < len(out)-1 && out[start] == 0 {
		start++
	}
	var sb strings.Builder
	for _, d := range out[start:] {
		sb.WriteByte(byte('0' + d))
	}
	return sb.String()
}
