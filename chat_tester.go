package main

import (
	"bufio"
	"fmt"
	"io"
	"os/exec"
	"time"
)

func main() {
	fmt.Println("🧪 OCLI Conversational Test")
	fmt.Println("============================\n")

	cmd := exec.Command("/home/drew/projects/ollama-ocli/target/release/ocli")
	
	stdin, _ := cmd.StdinPipe()
	stdout, _ := cmd.StdoutPipe()
	stderr, _ := cmd.StderrPipe()
	
	cmd.Start()
	
	// Read output
	go func() {
		scanner := bufio.NewScanner(stdout)
		for scanner.Scan() {
			fmt.Println("📤 " + scanner.Text())
		}
	}()
	
	go func() {
		scanner := bufio.NewScanner(stderr)
		for scanner.Scan() {
			fmt.Println("⚠️  " + scanner.Text())
		}
	}()
	
	time.Sleep(2 * time.Second)
	
	// Test conversation
	tests := []struct {
		input   string
		wait    int
		desc    string
	}{
		{"What is 2+2?", 8, "Simple math question"},
		{"/write-direct /tmp/test.txt Hello", 2, "Slash command"},
		{"exit", 1, "Exit"},
	}
	
	for i, test := range tests {
		fmt.Printf("\n🔵 Test %d: %s\n", i+1, test.desc)
		fmt.Printf("📥 Input: %s\n", test.input)
		
		io.WriteString(stdin, test.input+"\n")
		time.Sleep(time.Duration(test.wait) * time.Second)
	}
	
	stdin.Close()
	cmd.Wait()
	
	fmt.Println("\n✅ Complete!")
}
