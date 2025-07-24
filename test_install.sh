#!/usr/bin/env bash
# Test script for the fork installer

set -e

echo "Testing auto-commit-改 installer..."
echo

# Test 1: Debug mode
echo "Test 1: Running installer in debug mode (dry run)"
echo "================================================"
DEBUG=1 VERBOSE=1 bash -c '
    # Override main to prevent actual installation
    main() {
        echo "Main function called - installation would start here"
    }
    
    # Source the installer
    source ./install.sh
    
    # Test individual functions
    echo "Testing detect_system..."
    detect_system
    
    echo -e "\nTesting debug functions..."
    debug "This is a debug message"
    debug_var "PATH"
    verbose "This is a verbose message"
    
    echo -e "\nTesting message functions..."
    info "Info message test"
    warn "Warning message test"
    success "Success message test"
'

echo -e "\n\nTest 2: Help output"
echo "==================="
./install.sh --help

echo -e "\n\nTest 3: Function availability check"
echo "==================================="
source ./install.sh
functions=(
    "debug" "debug_var" "debug_cmd" "verbose"
    "log_section" "err" "success" "info" "warn"
    "detect_system" "check_prerequisites"
    "setup_repository" "build_project"
    "install_binary" "setup_environment"
    "setup_shell_integration" "cleanup"
    "post_install" "get_shell_config"
)

echo "Checking if all functions are defined..."
missing=0
for func in "${functions[@]}"; do
    if declare -f "$func" &> /dev/null; then
        echo "✓ $func"
    else
        echo "✗ $func is missing!"
        ((missing++))
    fi
done

if [ $missing -eq 0 ]; then
    echo -e "\n✅ All functions are properly defined!"
else
    echo -e "\n❌ $missing functions are missing!"
fi

echo -e "\n\nTest 4: System detection output"
echo "==============================="
detect_system

echo -e "\n\n✅ All tests completed!"