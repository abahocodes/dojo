package main

func calculateNoParens(s string) int {
	total, last, num := 0, 0, 0
	op := byte('+')
	n := len(s)
	for i := 0; i < n; i++ {
		ch := s[i]
		isDigit := ch >= '0' && ch <= '9'
		if isDigit {
			num = num*10 + int(ch-'0')
		}
		if (!isDigit && ch != ' ') || i == n-1 {
			switch op {
			case '+':
				total += last
				last = num
			case '-':
				total += last
				last = -num
			case '*':
				last *= num
			default:
				last /= num // Go truncates toward zero
			}
			op = ch
			num = 0
		}
	}
	return total + last
}
