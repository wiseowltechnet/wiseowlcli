#!/usr/bin/env bats

@test "version flag works" {
  run ./target/release/ocli --version
  [ "$status" -eq 0 ]
  [[ "$output" =~ "OCLI v0.3.0" ]]
}

@test "version short flag works" {
  run ./target/release/ocli -V
  [ "$status" -eq 0 ]
  [[ "$output" =~ "OCLI v0.3.0" ]]
}

@test "help command works" {
  run bash -c "echo -e /helpnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
  [[ "$output" =~ "OCLI COMMANDS" ]]
}

@test "version command works" {
  run bash -c "echo -e /versionnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
  [[ "$output" =~ "OCLI v0.3.0" ]]
}

@test "stats command works" {
  run bash -c "echo -e /statsnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
  [[ "$output" =~ "Session Statistics" ]]
}

@test "config set and get works" {
  run bash -c "echo -e /config set test_key test_valuen/config get test_keynexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
  [[ "$output" =~ "test_key" ]]
}

@test "config list works" {
  run bash -c "echo -e /config listnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
  [[ "$output" =~ "Configuration" ]]
}

@test "alias set and list works" {
  run bash -c "echo -e /alias set h /helpn/alias listnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
  [[ "$output" =~ "Aliases" ]]
}

@test "history command works" {
  run bash -c "echo -e /historynexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
}

@test "perf command works" {
  run bash -c "echo -e /perfnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
  [[ "$output" =~ "Performance Metrics" ]]
}

@test "mcp list works" {
  run bash -c "echo -e /mcp listnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
}

@test "context commands work" {
  run bash -c "echo -e /contextnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
}

@test "models command works" {
  run bash -c "echo -e /modelsnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
}

@test "clear command works" {
  run bash -c "echo -e /clearnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
}

@test "save command works" {
  run bash -c "echo -e /save test_sessionnexit | ./target/release/ocli"
  [ "$status" -eq 0 ]
}
