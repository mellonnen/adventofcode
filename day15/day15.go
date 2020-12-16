package main

import "fmt"

// INPUT
// most recent row numbers of the spoken words
var spoken = map[int]int{0:1, 13:2, 1:3, 8:4, 6:5, 15:6}
// the sequence of numbers in the game
// we can add all the input values and a 0 since we know that pattern will always occur
var seq = []int{0, 13, 1, 8, 6, 15, 0}

func main() {
  // PART 1 & 2
  row := len(seq) + 1 // next row
  for len(seq) !=  30000000 {

    prev := seq[len(seq) - 1] 
    prevRow := row - 1
   
    if dr,ok := spoken[prev]; ok { // the word has been spoken
      seq = append(seq, prevRow - dr)
      spoken[prev] = prevRow // update row number
    } else { // new word
      spoken[prev] = prevRow 
      seq = append(seq,0)
    }
    row++
  }
  fmt.Println(seq[len(seq) - 1])
}
