package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
)

// IO
var file, _ = os.Open("./input")
var io = bufio.NewScanner(file)

// GLOBAL VARS
var data [][]rune
var pp = 0                                    // parsing pointer
var precedence = map[rune]int{'*': 1, '+': 1} // precedence for part 1

// FUNCTIONS
func getInput() {
	for io.Scan() {
		expr := []rune(strings.ReplaceAll(io.Text(), " ", "")) // "tokenize" each expression
		data = append(data, expr)
	}
}

// Parsing by precedence climbing
// https://en.wikipedia.org/wiki/Operator-precedence_parser#Precedence_climbing_method

// parses numbers and parentheses expressions
func parsePrimary(expr []rune) int {
	var res int
	token := expr[pp]

	if token == '(' {
		pp++ //consume (
		res = parseExpression(expr)
		pp++ // consume )
	}

	if unicode.IsDigit(token) {
		res = int(token - '0')
		pp++ // consume number
	}

	return res
}
func applyOp(op rune, lhs, rhs int) int {
	var res int

	if op == '*' {
		res = lhs * rhs
	}

	if op == '+' {
		res = lhs + rhs
	}

	return res
}

// Parses a "new" expresion -> precedence of previous operator has no effect (parentheses expressions)
func parseExpression(expr []rune) int {
	return parseExpression_(expr, parsePrimary(expr), 0)
}

// Auxillary function for parsing when precedence of previous operation matters
func parseExpression_(expr []rune, lhs, minPrecedence int) int {
	if pp >= len(expr) {
		return lhs
	}

	lookahead := expr[pp]                        // peek token
	for precedence[lookahead] >= minPrecedence { // loop while as long as we do not encounter an operation that has precedance over the min
		op := lookahead
		pp++ // next token

		if pp >= len(expr) {
			return lhs
		}

		rhs := parsePrimary(expr)

		if pp < len(expr) {
			lookahead = expr[pp] // peek token
			for precedence[lookahead] > precedence[op] {
				rhs = parseExpression_(expr, rhs, precedence[lookahead])
				if pp >= len(expr) {
					break
				} // we have reached end of ecpression
				lookahead = expr[pp]
			}
		}
		lhs = applyOp(op, lhs, rhs) // execute op
		if lookahead == ')' {
			return lhs
		} // we are done processing parentheses expression
	}

	return lhs
}

func p1() int {
	res := 0
	for _, expr := range data {
		res += parseExpression(expr)
		pp = 0 // reset pointer
	}
	return res
}

func p2() int {
	precedence['+'] = 2 // change precedence of '+'
	pp = 0
	return p1()
}

func main() {
	getInput()
	fmt.Println("<==Part 1==>")
	fmt.Println(p1())
	fmt.Println("<==Part 2==>")
	fmt.Println(p2())
}
