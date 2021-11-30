package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// IO
var file, _ = os.Open("./input")
var io = bufio.NewScanner(file)
var rules []func(int) bool
var invalid = make([][]bool, 20)
var mine []int
var validTickets [][]int

// Solves part 1 and does some prep for part 2
func p1() int {
	errorRate := 0
	phase := 1

	for io.Scan() {
		line := io.Text()

		if line == "" { // on whitespacke we switch phases
			phase++
			continue
		}

		if phase == 1 {
			// Parse input line
			split := strings.Split(line, ": ")
			cs := strings.Split(split[1], " or ")
			lc := strings.Split(cs[0], "-")
			rc := strings.Split(cs[1], "-")

			rc1, _ := strconv.Atoi(rc[0])
			rc2, _ := strconv.Atoi(rc[1])

			lc1, _ := strconv.Atoi(lc[0])
			lc2, _ := strconv.Atoi(lc[1])

			// create function
			rule := func(x int) bool {
				return (lc1 <= x && x <= lc2) || (rc1 <= x && x <= rc2)
			}
			// add the rule(function) to the slice of rules for later
			rules = append(rules, rule)
		} else if phase == 2 {
			if line == "your ticket:" { // skip this line
				continue
			}
			// parse line and save for part 2
			split := strings.Split(line, ",")

			for _, f := range split {
				field, _ := strconv.Atoi(f)
				mine = append(mine, field)
			}
		} else {
			if line == "other tickets:" {
				continue
			}
			var ticket []int
			fields := strings.Split(line, ",")
			validTicket := true

			// check fields angaingst the rules from phase 1
			for _, f := range fields {
				field, _ := strconv.Atoi(f)
				validField := false
				ticket = append(ticket, field)

				for _, rule := range rules {
					if rule(field) {
						validField = true
						break
					}
				}
				if !validField {
					errorRate += field // result for part 1
					validTicket = false
				}
			}
			if validTicket { // prep for part 2
				validTickets = append(validTickets, ticket) // save ticket
				// check all the fields against all the rules
				for i, f := range fields {
					if invalid[i] == nil {
						invalid[i] = make([]bool, 20)
					}
					field, _ := strconv.Atoi(f)
					for j, rule := range rules {
						if !rule(field) {
							// if rule is not aplicablie for this field we now that the mapping for that field -> rule is invalid
							invalid[i][j] = true
						}
					}
				}
			}
		}
	}
	return errorRate
}

// Part 2
func p2() int {
	soulution := make([]int, 20)
	used := make([]bool, 20)
	found := 0
	for {
		for i := 0; i < 20; i++ {
			var js []int
			for j := 0; j < 20; j++ {
				if !invalid[i][j] && !used[j] {
					js = append(js, j)
				}
			}
			if len(js) == 1 { // find the fields that only has one possibility
				soulution[i] = js[0]
				used[js[0]] = true
				found++
			}
		}
		if found == 20 {
			break
		}
	}
	res := 1
	for i, j := range soulution {
		if j < 6 {
			res *= mine[i]
		}
	}
	return res
}

func main() {
	fmt.Println(p1())
	fmt.Println(p2())
}
