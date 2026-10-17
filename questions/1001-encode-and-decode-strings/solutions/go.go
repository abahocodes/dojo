package main

import (
	"strconv"
	"strings"
)

func encode(strs []string) string {
	// Each string becomes "<length>#<string>"; the length says where it ends.
	var sb strings.Builder
	for _, s := range strs {
		sb.WriteString(strconv.Itoa(len(s)))
		sb.WriteByte('#')
		sb.WriteString(s)
	}
	return sb.String()
}

func decode(s string) []string {
	result := []string{}
	i := 0
	for i < len(s) {
		j := i + strings.IndexByte(s[i:], '#')
		length, _ := strconv.Atoi(s[i:j])
		result = append(result, s[j+1:j+1+length])
		i = j + 1 + length
	}
	return result
}

// dojo calls this to run the tests; you don't need to change it.
func encodeAndDecode(strs []string) []string {
	encoded := encode(append([]string{}, strs...))
	other := make([]string, 0, len(strs)+1)
	for i := len(strs) - 1; i >= 0; i-- {
		other = append(other, strs[i])
	}
	encode(append(other, "dojo"))
	return decode(encoded)
}
