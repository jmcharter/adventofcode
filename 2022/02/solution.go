package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// A, X = Rock: 1
// B, Y = Paper: 2
// C, Z = Scissors: 3

// Win: 6
// Draw: 3
// Lose: 0

var valWin = 6
var valDraw = 3
var valLose = 0

var baseScore = map[string]int{
	"X": 1,
	"Y": 2,
	"Z": 3,
}

var score = map[string]map[string]int{
	"X": {
		"A": valDraw,
		"B": valLose,
		"C": valWin,
	},
	"Y": {
		"A": valWin,
		"B": valDraw,
		"C": valLose,
	},
	"Z": {
		"A": valLose,
		"B": valWin,
		"C": valDraw,
	},
}

var resultMap = map[string]map[string]string{
	"A": {
		"X": "Z",
		"Y": "X",
		"Z": "Y",
	},
	"B": {
		"X": "X",
		"Y": "Y",
		"Z": "Z",
	},
	"C": {
		"X": "Y",
		"Y": "Z",
		"Z": "X",
	},
}

func main() {
	path, err := filepath.Abs("input")
	if err != nil {
		panic(err)
	}
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	// part1(file)

	// Part 2: X = L, Y = D, Z = W
	part2(file)

}

func part1(file *os.File) {
	scanner := bufio.NewScanner(file)
	var totalScore int
	for scanner.Scan() {
		text := scanner.Text()
		text = strings.TrimSpace(text)
		round := strings.Split(text, " ")
		opponent, player := round[0], round[1]
		result := score[player][opponent]
		totalScore += baseScore[player]
		totalScore += result
	}
	fmt.Println(totalScore)
}

func part2(file *os.File) {
	scanner := bufio.NewScanner(file)
	var totalScore int
	for scanner.Scan() {
		text := scanner.Text()
		text = strings.TrimSpace(text)
		round := strings.Split(text, " ")
		opponent, tactic := round[0], round[1]
		player := resultMap[opponent][tactic]
		result := score[player][opponent]
		totalScore += baseScore[player]
		totalScore += result
	}
	fmt.Println(totalScore)
}
