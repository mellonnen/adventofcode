package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strings"
)

func main() {
	file, err := os.Open("./input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var b strings.Builder
	rex1 := regexp.MustCompile(`((byr|iyr|eyr|hgt|hcl|ecl|pid):.*){7}`)
	rex2 := regexp.MustCompile("(((byr:(19[2-9][0-9]|200[0-2]))" +
		"|(iyr:(20(1[0-9]|20)))" +
		"|(eyr:(20(2[0-9]|30)))" +
		"|(hgt:((1([5-8][0-9]|9[0-3])cm)|(([5-6][0-9]|7[0-6])in)))" +
		"|(hcl:(#[0-9a-f]{6}))" +
		"|(ecl:(amb|blu|brn|gry|grn|hzl|oth))" +
		"|(pid:([0-9]{9}))" +
		").*){7}")
	valid1 := 0
	valid2 := 0

	for scanner.Scan() {

		line := scanner.Text()

		if len(line) == 0 {
			if rex1.MatchString(b.String()) {
				valid1++
			}
			if rex2.MatchString(b.String()) {
				valid2++
			}
			b.Reset()
		} else {
			b.WriteString(" ")
			b.WriteString(line)
		}
	}

	if rex1.MatchString(b.String()) {
		valid1++
	}
	if rex2.MatchString(b.String()) {
			valid2++
		}
	fmt.Println(valid1)
	fmt.Println(valid2)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
