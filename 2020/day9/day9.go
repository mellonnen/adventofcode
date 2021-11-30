package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

// IO
var file, _ = os.Open("./input")
var io = bufio.NewScanner(file)

// creates set of the sum of all possible pairs
// O(n^2) = {n is always 25} = O(625)
func getRefrenceSet(preamble []int) map[int]bool {
	set := make(map[int]bool)

	for i := 0; i < len(preamble); i++ {
		for j := i + 1; j < len(preamble); j++ {
			res := preamble[i] + preamble[j]
			set[res] = true
		}
	}
	return set
}

func main() {
	i := 0
	target := 0
	var preamble []int
	var ciffer []int

	// PART 1
	// Process input
	// O(n)
	for io.Scan() {
		line := io.Text()
		number, _ := strconv.Atoi(line)
		ciffer = append(ciffer, number) // save the ciffer for Part 2

		// create the intial preamble
		if i < 25 {
			preamble = append(preamble, number)
			i++
			continue
		}

		refrence := getRefrenceSet(preamble)

		if !refrence[number] {
			target = number
		}

		preamble = preamble[1:]
		preamble = append(preamble, number)
	}

	fmt.Println("Part 1 : " + strconv.Itoa(target)) // O(n)

	// PART 2
	// O(n^2)
	var set []int
	for i := 0; i < len(ciffer); i++ {
		sum := 0
		set = set[:0] // clears the set
		set = append(set, ciffer[i])
		j := i + 1
		for sum < target {
			sum += ciffer[j]
			set = append(set, ciffer[j])
			j++
		}
		if len(set) > 1 && sum == target {
			break
		}
	}

	// O(n)
	max, min := set[0], set[0]
	for i := 1; i < len(set); i++ {
		if set[i] < min {
			min = set[i]
		}
		if set[i] > max {
			max = set[i]
		}
	}

	fmt.Println("Part 2 : " + strconv.Itoa(max+min)) // O(n^2)
}
