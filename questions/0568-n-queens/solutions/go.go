package main

import "strings"

func solveNQueens(n int) [][]string {
	cols := make([]bool, n)
	diag := make([]bool, 2*n-1) // indexed by r - c + n - 1
	anti := make([]bool, 2*n-1) // indexed by r + c
	queenCol := make([]int, n)  // queenCol[r] = column of the queen in row r
	boards := [][]string{}

	var place func(r int)
	place = func(r int) {
		if r == n {
			board := make([]string, n)
			for i, c := range queenCol {
				board[i] = strings.Repeat(".", c) + "Q" + strings.Repeat(".", n-1-c)
			}
			boards = append(boards, board)
			return
		}
		for c := 0; c < n; c++ {
			if cols[c] || diag[r-c+n-1] || anti[r+c] {
				continue
			}
			queenCol[r] = c
			cols[c], diag[r-c+n-1], anti[r+c] = true, true, true
			place(r + 1)
			cols[c], diag[r-c+n-1], anti[r+c] = false, false, false
		}
	}

	place(0)
	return boards
}
