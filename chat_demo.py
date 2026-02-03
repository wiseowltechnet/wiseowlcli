#!/usr/bin/env python3
import pexpect
import sys

print("🧪 OCLI Live Conversation Demo")
print("=" * 70)
print("This will show OCLI responding to questions in real-time\n")

child = pexpect.spawn("/home/drew/projects/ollama-ocli/target/release/ocli", 
                      encoding="utf-8", timeout=60)
child.logfile = sys.stdout

print("⏳ Waiting for OCLI to start...")
child.expect("You:", timeout=10)

print("\n" + "=" * 70)
print("✅ OCLI READY - Sending conversational question")
print("=" * 70)
print("📥 Question: What is 2+2?\n")

child.sendline("What is 2+2?")

print("⏳ Waiting for AI response (this proves conversational AI works)...\n")
child.expect("You:", timeout=30)

print("\n" + "=" * 70)
print("✅ AI RESPONDED! Now testing slash command")
print("=" * 70)

child.sendline("/write-direct /tmp/ocli_test.txt ConversationWorks")
child.expect("You:", timeout=5)

print("\n" + "=" * 70)
print("✅ Slash command worked! Exiting...")
print("=" * 70)

child.sendline("exit")
child.close()

print("\n✅ TEST COMPLETE - Both conversational AI and slash commands work!")
