package main

var phoneKeys = [10]string{"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"}

func letterCombinations(digits string) []string {
	result := []string{}
	if len(digits) == 0 {
		return result
	}
	current := make([]byte, 0, len(digits))

	var backtrack func(i int)
	backtrack = func(i int) {
		if i == len(digits) {
			result = append(result, string(current))
			return
		}
		for _, letter := range []byte(phoneKeys[digits[i]-'0']) {
			current = append(current, letter)
			backtrack(i + 1)
			current = current[:len(current)-1]
		}
	}

	backtrack(0)
	return result
}
