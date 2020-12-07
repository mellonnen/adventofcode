package main

import (
	"bufio"
	"fmt"
	"os"
)

var file, _ = os.Open("./input")
var io = bufio.NewScanner(file)
var sum = 0
var allSum = 0

func main() {
	set := make(map[rune]bool)
	allMap := make(map[rune]int)
	groupCount := 0
	for io.Scan() {
		line := io.Text()

		if len(line) == 0 {
			sum += len(set)
			set = make(map[rune]bool)
			for _, count := range allMap {
				if count == groupCount {
					allSum++
				}
			}
      groupCount = 0
      allMap = make(map[rune]int)

		} else {
			groupCount++
			for _, r := range line {
				set[r] = true
				allMap[r]++
			}
		}
	}
	sum += len(set)
	for _, count := range allMap {
		if count == groupCount {
			allSum++
		}
	}
	fmt.Println(sum)
	fmt.Println(allSum)
}
