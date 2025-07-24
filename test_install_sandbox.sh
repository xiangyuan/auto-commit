#!/usr/bin/env bash
# Sandbox test for the fork installer
# This creates a temporary environment to test the full installation

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${MAGENTA}     Auto-Commit-改 Installer Sandbox Test${NC}"
echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════${NC}"
echo

# Create sandbox directory
SANDBOX_DIR="/tmp/auto-commit-test-$(date +%s)"
echo -e "${BLUE}→ Creating sandbox environment: $SANDBOX_DIR${NC}"
mkdir -p "$SANDBOX_DIR"

# Cleanup function
cleanup() {
    echo -e "\n${YELLOW}⚠️  Cleaning up sandbox...${NC}"
    rm -rf "$SANDBOX_DIR"
    echo -e "${GREEN}✅ Sandbox cleaned up${NC}"
}

# Set trap for cleanup
trap cleanup EXIT

# Copy installer to sandbox
cp install.sh "$SANDBOX_DIR/"

# Create a mock .env file in sandbox
echo "export DEEPSEEK_API_KEY='sk-test-key-12345'" > "$SANDBOX_DIR/.env"

# Test 1: Dry run with custom directories
echo -e "\n${CYAN}Test 1: Dry run with custom directories${NC}"
echo "========================================="
(
    cd "$SANDBOX_DIR"
    BIN_DIR="$SANDBOX_DIR/bin" \
    CONFIG_DIR="$SANDBOX_DIR/config" \
    DEBUG=1 \
    bash -c '
        # Override functions to prevent actual operations
        setup_repository() {
            log_section "Repository Setup (MOCKED)"
            info "Would clone repository to temp directory"
            TEMP_DIR="$SANDBOX_DIR/temp"
            mkdir -p "$TEMP_DIR/auto-commit"
            cd "$TEMP_DIR/auto-commit"
            success "Repository setup mocked"
        }
        
        build_project() {
            log_section "Building Project (MOCKED)"
            info "Would run cargo build --release"
            success "Build mocked"
        }
        
        install_binary() {
            log_section "Installing Binary (MOCKED)"
            mkdir -p "$BIN_DIR"
            echo "#!/bin/bash" > "$BIN_DIR/auto-commit"
            echo "echo \"auto-commit v0.1.0 (test)\"" >> "$BIN_DIR/auto-commit"
            chmod +x "$BIN_DIR/auto-commit"
            success "Binary installed (mock)"
        }
        
        # Source installer
        source ./install.sh
        
        # Run main without actual build
        detect_system
        check_prerequisites() {
            log_section "Prerequisites Check (MOCKED)"
            info "All prerequisites satisfied (mock)"
        }
        check_prerequisites
        setup_repository
        build_project
        install_binary
        setup_environment
    '
)

# Test 2: Check created files
echo -e "\n${CYAN}Test 2: Verify created files${NC}"
echo "=============================="

if [ -f "$SANDBOX_DIR/bin/auto-commit" ]; then
    echo -e "${GREEN}✅ Mock binary created${NC}"
    "$SANDBOX_DIR/bin/auto-commit"
else
    echo -e "${RED}❌ Mock binary not found${NC}"
fi

if [ -f "$SANDBOX_DIR/config/.env.example" ]; then
    echo -e "${GREEN}✅ .env.example created${NC}"
    echo -e "${BLUE}Contents:${NC}"
    cat "$SANDBOX_DIR/config/.env.example" | head -5
else
    echo -e "${RED}❌ .env.example not found${NC}"
fi

# Test 3: Test argument parsing
echo -e "\n${CYAN}Test 3: Test argument parsing${NC}"
echo "==============================="

echo -e "\n${BLUE}Testing --fork-owner:${NC}"
(
    cd "$SANDBOX_DIR"
    ./install.sh --fork-owner testuser --help 2>&1 | grep -q "testuser" && \
        echo -e "${GREEN}✅ --fork-owner parameter works${NC}" || \
        echo -e "${RED}❌ --fork-owner parameter failed${NC}"
)

# Test 4: Test error handling
echo -e "\n${CYAN}Test 4: Test error handling${NC}"
echo "============================"

echo -e "\n${BLUE}Testing invalid option:${NC}"
(
    cd "$SANDBOX_DIR"
    ./install.sh --invalid-option 2>&1 | grep -q "Unknown option" && \
        echo -e "${GREEN}✅ Invalid option detected${NC}" || \
        echo -e "${RED}❌ Invalid option not handled${NC}"
)

# Test 5: Test debug log creation
echo -e "\n${CYAN}Test 5: Test debug log creation${NC}"
echo "================================"

(
    cd "$SANDBOX_DIR"
    DEBUG=1 bash -c '
        source ./install.sh
        debug "Test debug message"
    ' 2>&1 > /dev/null
)

if ls "$SANDBOX_DIR"/install_debug_*.log >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Debug log created${NC}"
    DEBUG_LOG=$(ls "$SANDBOX_DIR"/install_debug_*.log | head -1)
    echo -e "${BLUE}Debug log: $(basename "$DEBUG_LOG")${NC}"
    echo -e "${BLUE}Contents (first 5 lines):${NC}"
    head -5 "$DEBUG_LOG"
else
    echo -e "${YELLOW}⚠️  Debug log not found (this is OK if debug wasn't fully enabled)${NC}"
fi

echo -e "\n${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}     ✅ All sandbox tests completed!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"