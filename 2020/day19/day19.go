package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

// IO
var file, _ = os.Open("./input")
var io = bufio.NewScanner(file)

func getInput() []string {
	var data []string
	for io.Scan() {
		line := io.Text()
		data = append(data, line)
	}
	return data
}

// SOULUTION

// recursivley determine rules
// could be momized to improve performance, but was quick enough
func getRule(r string, rules map[string]string) string {
	// base case
	if rules[r] == "a" || rules[r] == "b" {
		return rules[r]
		// or case (recursive)
	} else if strings.Contains(rules[r], "|") {

		split := strings.Split(rules[r], " | ")
		rhs := strings.Split(split[0], " ")
		lhs := strings.Split(split[1], " ")

		rule := "("
		for _, x := range rhs {
			rule += getRule(x, rules)
		}
		rule += "|"
		for _, x := range lhs {
			rule += getRule(x, rules)
		}
		rule += ")"

		return rule
		// regular case
	} else {

		rule := ""
		split := strings.Split(rules[r], " ")
		for _, x := range split {
			rule += getRule(x, rules)
		}
		return rule
	}
}

// Part 1
func p1(data []string) int {

	start := 0 // start "input" section
	rules := make(map[string]string)
	for i, line := range data {
		if line == "" {
			start = i + 1
			break
		}
		split := strings.Split(strings.Replace(line, "\"", "", -1), ": ")
		rules[split[0]] = split[1]
	}

	str := "^" + getRule("0", rules) + "$" // we want full match hence "^" and "$"
	rule := regexp.MustCompile(str)
	res := 0
	for i := start; i < len(data); i++ { // loop over "input"
		if rule.MatchString(data[i]) {
			res++
		}
	}
	return res
}

// part 2
func p2(data []string) int {

	start := 0
	rules := make(map[string]string)
	for i, line := range data {
		if line == "" {
			start = i + 1
			break
		}
		split := strings.Split(strings.Replace(line, "\"", "", -1), ": ")
		rules[split[0]] = split[1]
	}
	r42 := getRule("42", rules)
	r31 := getRule("31", rules)

	// rule 8: 42 | 42 8 => regex for rule 8 is 42+
	r8 := r42 + "+"

	// rule 11: 42 31 | 42 11 31 => regex for rule 11 is 42{n}31{n}
	// we create an or chain where 1 <= n <= 4
	r11 := "(("
	for n := 1; n <= 4; n++ {
		r11 += "(" + r42 + ")" + "{" + strconv.Itoa(n) + "}" + "(" + r31 + ")" + "{" + strconv.Itoa(n) + "}" + ")|("
	}
	r11 = r11[:len(r11)-2] // remove trailing "|("
	r11 += ")"

	// rule 0: 8 11
	r0 := "^" + r8 + r11 + "$"
	rex0 := regexp.MustCompile(r0)

	res := 0
	for i := start; i < len(data); i++ {
		if rex0.MatchString(data[i]) {
			res++
		}
	}
	return res
}

func main() {
	data := getInput()
	fmt.Println("<==Part 1==>")
	fmt.Println(p1(data))
	fmt.Println("<==Part 2==>")
	fmt.Println(p2(data))
}
