package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var count int
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		pairs := strings.Split(scanner.Text(), ",")
		elf1 := strings.Split(pairs[0], "-")
		elf2 := strings.Split(pairs[1], "-")
		elf1_l, _ := strconv.Atoi(elf1[0])
		elf1_h, _ := strconv.Atoi(elf1[1])
		elf2_l, _ := strconv.Atoi(elf2[0])
		elf2_h, _ := strconv.Atoi(elf2[1])
		if (elf1_l <= elf2_l && elf1_h >= elf2_h) || (elf2_l <= elf1_l && elf2_h >= elf1_h) {
			count += 1
		}
	}
	fmt.Println(count)
}
