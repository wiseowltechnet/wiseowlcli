#!/bin/bash
echo "🧹 OCLI Cleanup"
echo "==============="

# Keep backup
echo "✅ Keeping: src/main.rs.backup (safety backup)"

# Remove temp build files
echo "🗑️  Removing temp .rs files from /tmp..."
rm -f /tmp/*.rs /tmp/*_cmd.rs /tmp/main*.rs 2>/dev/null

# Organize test files
echo "📁 Test files already in tests/ directory"

# Remove redundant test scripts (keep the good ones)
echo "🗑️  Removing redundant test scripts..."
rm -f test_ai.exp test_exit.exp test.exp test_manual.sh test_file_ops.sh \
      test_chat_simple.sh test_direct.sh test_prompt.sh test_tools.sh \
      test_version.exp test_stats.exp test_self_improve.exp test_read_self.exp \
      test_monitor.exp test_lcars.exp test_git.exp test_autonomous.exp \
      test_ai_response.exp test_file.py test.py test_tool_parse.rs \
      test_export.md 2>/dev/null

# Keep useful ones
echo "✅ Keeping useful test files:"
echo "   - chat_tester.py (pexpect tester)"
echo "   - test_chat.sh (conversation test)"
echo "   - CONVERSATION_TEST.md (test results)"
echo "   - tests/ directory (organized tests)"

echo ""
echo "✅ Cleanup complete!"
echo ""
echo "📊 Remaining files:"
ls -lh *.sh *.py *.md 2>/dev/null | grep -E "(test|chat)" | wc -l
echo "   test/utility files"
