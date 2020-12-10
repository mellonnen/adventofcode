package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

// IO
var file,_  = os.Open("./input")
var io = bufio.NewScanner(file)


var adapters []int 
var memo = make(map[int]int) // for memoizing the recursion

// memoized recursion (PART 2)
func arrangements(i int) int {
  if i == len(adapters) - 1 { // base case
    return 1
  }
  if seen,ok := memo[i];ok {  // check if memoized 
    return seen
  }
  ways := 0
  for j := i + 1; j < len(adapters); j++ {
    if adapters[j] - adapters[i] <= 3 { // if the next is in range -> recurse
      ways += arrangements(j)
    } else { // we can break after the first comparision that fails since adapters is sorted
      break
    }    
  }
  memo[i] = ways
  return ways
}


func main() {

  // SETUP
  builtInAdapter := 0 
  for io.Scan() {
    adapter, _ := strconv.Atoi(io.Text())
    adapters = append(adapters,adapter)
    // find the maxiumum adapter value
    if adapter > builtInAdapter {
      builtInAdapter = adapter
    } 
  }
  
  // add 3 "extra" joltage
  builtInAdapter += 3
  // add the to the other adapters
  adapters = append(adapters,builtInAdapter) 
  // add zero (outlet)
  adapters = append(adapters,0) 

  // sort slice
  sort.Ints(adapters)

  // PART 1
  ones,threes := 0,0

  for i := 0; i < len(adapters) - 1; i++ {
    diff := adapters[i+1] - adapters[i] 
    // count diffs of three and one
    switch diff {
    case 1:
      ones ++
    case 3:
      threes ++
    }

  }

  fmt.Println("Part 1 : " + strconv.Itoa(ones*threes))
  
  // PART 2
  
  fmt.Println("Part 2 : " + strconv.Itoa(arrangements(0)))
}

