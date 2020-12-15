package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// IO
var file,_ = os.Open("./input")
var io = bufio.NewScanner(file)


// bit masking functions
func set(b, flag uint64) uint64    { return b | flag }
func clear(b, flag uint64) uint64  { return b &^ flag }
func toggle(b, flag uint64) uint64 { return b ^ flag }

// recursive function for generating the adresses in Part 2
func genAdresses (adress uint64, floating []int) []uint64 {
  // base case
  if len(floating) == 0 {
    return []uint64{adress}
  } else {
    tempMask := uint64(1)  
    bit := floating[0] 
    floating = floating[1:] // pop of
    tempMask = tempMask << bit // shift 1 to correct position
    //make 2 recursive calls, one leaves value as it is, other toggles value
    return append(genAdresses(adress,floating), genAdresses(toggle(adress,tempMask),floating)...)
  }
}


func main() {
  
  // memory representation
  mem1 := make(map[uint64]uint64)
  mem2 := make(map[uint64]uint64)
  // masks
  var setMask uint64 
  var clearMask uint64 
  // positions of floating bits
  var floating []int

  for io.Scan() {
    line := strings.Split(io.Text(), " = ")
    
    if line[0] == "mask" {

      maskStr := line[1]
      // create set mask (used to set 1s)
      setMask, _ = strconv.ParseUint(strings.ReplaceAll(maskStr,"X","0"),2,64) 
      // create clear mask (used to set 0s)
      clearStr := strings.ReplaceAll(maskStr,"1","X")
      clearStr = strings.ReplaceAll(clearStr,"0","1")
      clearMask,_ = strconv.ParseUint(strings.ReplaceAll(clearStr,"X","0"),2,64)

      // track postions of floating bits
      floating = floating[:0]
      for i := 0 ; i < len(maskStr); i++ {
        if maskStr[len(maskStr) - 1 - i] == 'X' {
          floating = append(floating,i)
        }
      }
    } else {
      // get adress
      adressStr := line[0][4:]
      adressStr = adressStr[:len(adressStr)-1]
      adress,_ := strconv.ParseUint(adressStr,10,64)
      // get value
      val,_ := strconv.ParseUint(line[1],10,64)

      // Part 1 (mask value)
      maskedVal := set(val, setMask) // set 1s
      mem1[adress] = clear(maskedVal,clearMask) // set 0s and write to memory
      
      // Part 2 (mask adress)
      adress = set(adress,setMask) // set 1s
      // generate adresses for all floating bits combinations
      for _ ,adr := range genAdresses(adress,floating) {
        mem2[adr] = val // write to memory
      } 
      
    }
  }

  var res1 uint64 
  for _,val := range mem1 {
    res1 += val
  }

  var res2 uint64 
  for _,val := range mem2 {
    res2 += val
  }

  fmt.Println("<==Part 1==>")
  fmt.Println(res1)

  fmt.Println("<==Part 2==>")
  fmt.Println(res2)
}
