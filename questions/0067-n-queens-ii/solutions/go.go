package main

func totalNQueens(n int) int {
	full := (1 << n) - 1

	// cols, diag, anti: bitmasks of the columns attacked in the current row
	var place func(cols, diag, anti int) int
	place = func(cols, diag, anti int) int {
		if cols == full {
			return 1
		}
		count := 0
		free := full &^ (cols | diag | anti)
		for free != 0 {
			bit := free & -free // lowest free column
			free ^= bit
			count += place(cols|bit, ((diag|bit)<<1)&full, (anti|bit)>>1)
		}
		return count
	}

	return place(0, 0, 0)
}
