package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

var file, _ = os.Open("./input")
var maxID = 0
var io = bufio.NewScanner(file)
var seats []int

func main() {

	for io.Scan() {
		maxRow := 127
		var midRow int
		minRow := 0

		maxCol := 7
		var midCol int
		minCol := 0

		line := io.Text()

		for _, c := range line {
			midRow = (maxRow + minRow) / 2
			midCol = (maxCol + minCol) / 2
			switch c {
			case 'F':
				maxRow = midRow
			case 'B':
				midRow++
				minRow = midRow
			case 'L':
				maxCol = midCol
			case 'R':
				midCol++
				minCol = midCol
			}
		}
		id := midRow*8 + midCol
		seats = append(seats, id)

		if id > maxID {
			maxID = id
		}
	}
	sort.Slice(seats, func(i, j int) bool { return seats[i] < seats[j] })
	for i := 0; i < len(seats)-1; i++ {
		if seats[i+1]-seats[i] == 2 {
			fmt.Println(seats[i] + 1)
		}
	}
	fmt.Println(maxID)
}
