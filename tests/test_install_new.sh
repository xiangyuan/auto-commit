#!/usr/bin/env bash
# Integration tests for new install.sh

set -e

# Test that the script has all required functions
echo "Testing function definitions..."

# Source the script
source ./install_new.sh

# Check if functions are defined
functions=(
    "err"
    "success"
    "info"
    "warn"
    "check_command"
    "check_prerequisites"
    "clone_repository"
    "build_project"
    "install_binary"
    "add_to_path"
)

missing_functions=0
for func in "${functions[@]}"; do
    if ! declare -f "$func" &> /dev/null; then
        echo "✗ Function $func is not defined"
        ((missing_functions++))
    else
        echo "✓ Function $func is defined"
    fi
done

if [ $missing_functions -eq 0 ]; then
    echo "✓ All required functions are defined"
else
    echo "✗ $missing_functions functions are missing"
    exit 1
fi

# Test error messages
echo -e "\nTesting error handling..."

# Test check_command function
echo -n "Testing check_command with missing command: "
if output=$(check_command "nonexistentcommand" "Test message" 2>&1); then
    echo "✗ Should have failed"
else
    if [[ "$output" == *"Error:"* ]] && [[ "$output" == *"Test message"* ]]; then
        echo "✓ Correct error message"
    else
        echo "✗ Incorrect error message: $output"
    fi
fi

# Test with existing command
echo -n "Testing check_command with existing command: "
if check_command "bash" "Should not see this" 2>&1; then
    echo "✓ Command found"
else
    echo "✗ Should have succeeded"
fi

echo -e "\n✓ All tests passed!"