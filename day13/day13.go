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

// modulo implementation that can handle negative numbers
func mod(a, b int) int {
	return (a%b + b) % b
}

// calculates a^b under (mod m)
func modPower(a, b, m int) int {
	if b == 0 {
		return 1
	}

	p := modPower(a, b/2, m)
	p = (p * p) % m

	if b%2 == 0 {
		return p
	} else {
		return ((a * p) % m)
	}
}

func main() {

	// handle input
	io.Scan()
	timestamp, _ := strconv.Atoi(io.Text())
	io.Scan()
	strs := strings.Split(io.Text(), ",")

	var ids []int     // bus ids
	var offsets []int // bus offsets
	N := 1
	for i := range strs {
		if strs[i] != "x" {
			id, _ := strconv.Atoi(strs[i])
			ids = append(ids, id)
			offsets = append(offsets, mod(i, id))
			N *= id
		}
	}

	// PART 1
	// inital values to minimize
	downTime := ids[0] - (timestamp % ids[0])
	res := ids[0] * downTime

	for _, id := range ids {
		dt := id - (timestamp % id)
		if dt < downTime {
			downTime = dt
			res = id * dt
		}
	}
	fmt.Println("<==Part 1==>")
	fmt.Println(res)

	// PART 2
	// We want to find an timestamp x such that x + ofsets[i] === 0 (mod ids[i]) for 0 <= i < len(ids)

	// we get this system of congruences

	// x ===  -offset[0] (mod ids[0])
	// x ===  -offset[1] (mod ids[1])
	// .
	// .
	// .
	// x ===  -offset[k] (mod ids[k])

	// by the chinese remainder theorem the soulution is given by:
	// x = Sum(i=0 to len(ids))(-offset[i] * MI * NI) where :
	// .- NI = N/ids[i], where N = product(ids)
	// .- all elements in ids are coprime (in our case they are all prime numbers!)
	// .- MI is a number such that MI * NI === 1 (mod n_i)

	// The tricky part is to determine MI (modular inverse of NI)
	// by fermat little theorem we now that a^(p-1) === 1 (mod p)
	// if p is a prime number.
	// we can use this to find the modular inverse:
	// a^(p-2) * a === 1 (mod p) => M_i = N_i^(n_i - 2)

	ans := 0
	for i := 0; i < len(ids); i++ {
		ni := ids[i]
		NI := N / ni
		MI := modPower(NI, ni-2, ni)
		ans += -offsets[i] * NI * MI
	}

	fmt.Println("<==Part 2==>")
	fmt.Println(mod(ans, N))
}
