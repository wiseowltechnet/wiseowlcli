#!/usr/bin/env python3
import pexpect
import sys
import time

def test_conversation():
    print("🧪 OCLI Conversational Test")
    print("=" * 60)
    print()
    
    # Start OCLI
    print("🚀 Starting OCLI...")
    child = pexpect.spawn("/home/drew/projects/ollama-ocli/target/release/ocli", 
                          encoding="utf-8", timeout=30)
    child.logfile = sys.stdout
    
    # Wait for startup
    time.sleep(3)
    
    # Test 1: Conversational question
    print("\n" + "=" * 60)
    print("🔵 TEST 1: Conversational AI")
    print("=" * 60)
    print("📥 Sending: What is 2+2?")
    child.sendline("What is 2+2?")
    time.sleep(8)
    
    # Test 2: Slash command
    print("\n" + "=" * 60)
    print("🔵 TEST 2: Slash Command")
    print("=" * 60)
    print("📥 Sending: /write-direct /tmp/test.txt Hello")
    child.sendline("/write-direct /tmp/test.txt Hello")
    time.sleep(2)
    
    # Exit
    print("\n" + "=" * 60)
    print("🔵 Exiting OCLI")
    print("=" * 60)
    child.sendline("exit")
    child.expect(pexpect.EOF, timeout=5)
    
    print("\n✅ Test complete!")

if __name__ == "__main__":
    test_conversation()
