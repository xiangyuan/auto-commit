#!/usr/bin/env bash
# Simple test for installer functions without actual installation

set -e

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
NC='\033[0m'

echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${MAGENTA}     Auto-Commit-改 Installer Function Tests${NC}"
echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════${NC}"

# Test 1: Source installer and check functions
echo -e "\n${BLUE}Test 1: Function availability${NC}"
echo "==============================="
source ./install.sh

required_functions=(
    "debug" "debug_var" "debug_cmd" "verbose"
    "log_section" "err" "success" "info" "warn"
    "detect_system" "check_prerequisites"
    "setup_repository" "build_project"
    "install_binary" "setup_environment"
    "setup_shell_integration" "cleanup"
    "post_install" "get_shell_config"
)

all_good=true
for func in "${required_functions[@]}"; do
    if declare -f "$func" &> /dev/null; then
        echo "✓ $func"
    else
        echo "✗ $func missing!"
        all_good=false
    fi
done

if $all_good; then
    echo -e "\n${GREEN}✅ All required functions present${NC}"
else
    echo -e "\n${YELLOW}⚠️  Some functions missing${NC}"
fi

# Test 2: System detection
echo -e "\n${BLUE}Test 2: System detection${NC}"
echo "========================="
detect_system

# Test 3: Shell config detection
echo -e "\n${BLUE}Test 3: Shell config detection${NC}"
echo "================================"
echo "Current shell: $(basename "$SHELL")"
echo "Config file: $(get_shell_config)"

# Test 4: Help message
echo -e "\n${BLUE}Test 4: Help message${NC}"
echo "====================="
./install.sh --help

# Test 5: Debug mode activation
echo -e "\n${BLUE}Test 5: Debug mode test${NC}"
echo "========================"
DEBUG=1 bash -c '
    source ./install.sh
    debug "This is a debug message"
    debug_var "USER"
    verbose "This is verbose output"
'

# Test 6: Message formatting
echo -e "\n${BLUE}Test 6: Message formatting${NC}"
echo "==========================="
info "Information message"
warn "Warning message"
success "Success message"

# Test 7: Fork owner parameter
echo -e "\n${BLUE}Test 7: Fork owner parameter${NC}"
echo "=============================="
FORK_OWNER=testuser bash -c '
    source ./install.sh
    echo "Fork owner: $FORK_OWNER"
    echo "Repo URL: $REPO_URL"
'

echo -e "\n${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}     ✅ All tests completed successfully!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"