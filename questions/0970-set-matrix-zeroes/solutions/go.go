package main

func setZeroes(matrix [][]int) [][]int {
	rows, cols := len(matrix), len(matrix[0])
	// Row 0 and column 0 double as markers; remember separately whether
	// they themselves need clearing.
	firstRowZero, firstColZero := false, false
	for c := 0; c < cols; c++ {
		if matrix[0][c] == 0 {
			firstRowZero = true
		}
	}
	for r := 0; r < rows; r++ {
		if matrix[r][0] == 0 {
			firstColZero = true
		}
	}

	for r := 1; r < rows; r++ {
		for c := 1; c < cols; c++ {
			if matrix[r][c] == 0 {
				matrix[r][0] = 0
				matrix[0][c] = 0
			}
		}
	}

	for r := 1; r < rows; r++ {
		for c := 1; c < cols; c++ {
			if matrix[r][0] == 0 || matrix[0][c] == 0 {
				matrix[r][c] = 0
			}
		}
	}

	if firstRowZero {
		for c := 0; c < cols; c++ {
			matrix[0][c] = 0
		}
	}
	if firstColZero {
		for r := 0; r < rows; r++ {
			matrix[r][0] = 0
		}
	}
	return matrix
}
