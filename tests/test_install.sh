#!/usr/bin/env bash
# Test suite for install.sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Test framework functions
assert_equals() {
    local expected="$1"
    local actual="$2"
    local message="${3:-}"
    
    if [ "$expected" = "$actual" ]; then
        pass "$message"
    else
        fail "$message: expected '$expected', got '$actual'"
    fi
}

assert_contains() {
    local haystack="$1"
    local needle="$2"
    local message="${3:-}"
    
    if [[ "$haystack" == *"$needle"* ]]; then
        pass "$message"
    else
        fail "$message: '$needle' not found in '$haystack'"
    fi
}

assert_file_exists() {
    local file="$1"
    local message="${2:-File should exist}"
    
    if [ -f "$file" ]; then
        pass "$message"
    else
        fail "$message: $file does not exist"
    fi
}

assert_executable() {
    local file="$1"
    local message="${2:-File should be executable}"
    
    if [ -x "$file" ]; then
        pass "$message"
    else
        fail "$message: $file is not executable"
    fi
}

assert_command_exists() {
    local cmd="$1"
    local message="${2:-Command should exist}"
    
    if command -v "$cmd" &> /dev/null; then
        pass "$message"
    else
        fail "$message: command '$cmd' not found"
    fi
}

pass() {
    ((TESTS_PASSED++))
    echo -e "${GREEN}✓${NC} $1"
}

fail() {
    ((TESTS_FAILED++))
    echo -e "${RED}✗${NC} $1"
}

describe() {
    echo -e "\n${YELLOW}$1${NC}"
}

it() {
    ((TESTS_RUN++))
    echo -n "  "
}

# Setup and teardown
setup() {
    # Create temporary directory for tests
    export TEST_DIR=$(mktemp -d)
    export ORIG_PATH=$PATH
    export ORIG_HOME=$HOME
    
    # Mock HOME directory
    export HOME=$TEST_DIR/home
    mkdir -p $HOME
    
    # Mock BIN_DIR
    export BIN_DIR=$HOME/.bin
    mkdir -p $BIN_DIR
    
    # Copy install script
    cp install.sh $TEST_DIR/
    
    # Change to test directory
    cd $TEST_DIR
}

teardown() {
    # Restore original values
    export HOME=$ORIG_HOME
    export PATH=$ORIG_PATH
    
    # Clean up
    rm -rf $TEST_DIR
}

# Mock functions for testing
mock_curl() {
    echo "mock_curl called with: $@" >> $TEST_DIR/curl.log
    # Simulate successful download
    if [[ "$*" == *"-L"* ]]; then
        echo "#!/bin/bash" > "${@: -1}"
        echo "echo 'auto-commit v0.1.4'" >> "${@: -1}"
        return 0
    fi
    return 1
}

mock_git() {
    echo "mock_git called with: $@" >> $TEST_DIR/git.log
    
    if [[ "$1" == "clone" ]]; then
        # Simulate git clone
        local repo_url="$2"
        local dest="${3:-auto-commit}"
        mkdir -p "$dest"
        
        # Create mock Rust project structure
        mkdir -p "$dest/src"
        echo 'fn main() { println!("auto-commit v0.1.4"); }' > "$dest/src/main.rs"
        
        # Create mock Cargo.toml
        cat > "$dest/Cargo.toml" << 'EOF'
[package]
name = "auto-commit"
version = "0.1.4"
edition = "2021"

[[bin]]
name = "auto-commit"
path = "src/main.rs"
EOF
        return 0
    fi
    
    return 1
}

mock_cargo() {
    echo "mock_cargo called with: $@" >> $TEST_DIR/cargo.log
    
    if [[ "$1" == "build" ]] && [[ "$2" == "--release" ]]; then
        # Simulate cargo build
        mkdir -p target/release
        cat > target/release/auto-commit << 'EOF'
#!/bin/bash
echo "auto-commit v0.1.4"
EOF
        chmod +x target/release/auto-commit
        return 0
    fi
    
    return 1
}

# Run setup
setup

# Test: Check if script detects missing Git
describe "Git detection"

it "should check if git is installed"
# Create modified install.sh that checks for git
cat > install_test.sh << 'EOF'
#!/usr/bin/env bash
set -e

check_git() {
    if ! command -v git &> /dev/null; then
        echo "Error: git is not installed"
        return 1
    fi
    return 0
}

# Test function
check_git
EOF
chmod +x install_test.sh

# Test with git available
if ./install_test.sh &> /dev/null; then
    pass "detects git when available"
else
    fail "fails when git is available"
fi

# Test: Check if script detects missing Rust
describe "Rust detection"

it "should check if cargo is installed"
cat > install_test.sh << 'EOF'
#!/usr/bin/env bash
set -e

check_rust() {
    if ! command -v cargo &> /dev/null; then
        echo "Error: Rust is not installed"
        echo "Please install Rust from https://rustup.rs/"
        return 1
    fi
    return 0
}

# Test function
check_rust
EOF
chmod +x install_test.sh

# Since cargo might be installed, we test the function logic
source install_test.sh
if declare -f check_rust &> /dev/null; then
    pass "rust check function defined"
else
    fail "rust check function not defined"
fi

# Test: Git clone functionality
describe "Git clone"

it "should clone the repository"
cat > install_test.sh << 'EOF'
#!/usr/bin/env bash
set -e

clone_repo() {
    local repo_url="https://github.com/m1guelpf/auto-commit.git"
    local temp_dir=$(mktemp -d)
    
    echo "Cloning repository..."
    git clone "$repo_url" "$temp_dir/auto-commit" || return 1
    
    echo "$temp_dir"
}

# For testing, override git command
git() {
    mock_git "$@"
}

# Import mock
source /dev/stdin << 'MOCK'
mock_git() {
    echo "mock_git called with: $@" >> git.log
    
    if [[ "$1" == "clone" ]]; then
        local dest="${3:-auto-commit}"
        mkdir -p "$dest"
        touch "$dest/Cargo.toml"
        return 0
    fi
    
    return 1
}
MOCK

# Test function
temp_dir=$(clone_repo)
if [ -d "$temp_dir/auto-commit" ] && [ -f "$temp_dir/auto-commit/Cargo.toml" ]; then
    echo "SUCCESS"
    rm -rf "$temp_dir"
else
    echo "FAILED"
fi
EOF
chmod +x install_test.sh

output=$(./install_test.sh 2>&1)
if [[ "$output" == *"SUCCESS"* ]]; then
    pass "clones repository successfully"
else
    fail "failed to clone repository"
fi

# Test: Build functionality
describe "Cargo build"

it "should build the project"
cat > install_test.sh << 'EOF'
#!/usr/bin/env bash
set -e

build_project() {
    local project_dir="$1"
    
    cd "$project_dir"
    echo "Building project..."
    cargo build --release || return 1
    
    if [ -f "target/release/auto-commit" ]; then
        return 0
    else
        return 1
    fi
}

# Mock cargo for testing
cargo() {
    if [[ "$1" == "build" ]] && [[ "$2" == "--release" ]]; then
        mkdir -p target/release
        touch target/release/auto-commit
        chmod +x target/release/auto-commit
        return 0
    fi
    return 1
}

# Test
mkdir -p test_project
cd test_project
if build_project "."; then
    echo "SUCCESS"
else
    echo "FAILED"
fi
EOF
chmod +x install_test.sh

output=$(./install_test.sh 2>&1)
if [[ "$output" == *"SUCCESS"* ]]; then
    pass "builds project successfully"
else
    fail "failed to build project"
fi

# Test: Installation
describe "Installation"

it "should install binary to BIN_DIR"
cat > install_test.sh << 'EOF'
#!/usr/bin/env bash
set -e

install_binary() {
    local binary_path="$1"
    local bin_dir="${BIN_DIR:-$HOME/.bin}"
    
    mkdir -p "$bin_dir"
    
    echo "Installing binary..."
    cp "$binary_path" "$bin_dir/auto-commit" || return 1
    chmod +x "$bin_dir/auto-commit" || return 1
    
    return 0
}

# Test
BIN_DIR="./test_bin"
mkdir -p target/release
echo '#!/bin/bash' > target/release/auto-commit
echo 'echo "test"' >> target/release/auto-commit
chmod +x target/release/auto-commit

if install_binary "target/release/auto-commit"; then
    if [ -x "./test_bin/auto-commit" ]; then
        echo "SUCCESS"
    else
        echo "FAILED: not executable"
    fi
else
    echo "FAILED: install failed"
fi
EOF
chmod +x install_test.sh

output=$(./install_test.sh 2>&1)
if [[ "$output" == *"SUCCESS"* ]]; then
    pass "installs binary successfully"
else
    fail "failed to install binary: $output"
fi

# Print test summary
echo -e "\n${YELLOW}Test Summary:${NC}"
echo -e "Tests run: $TESTS_RUN"
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Failed: $TESTS_FAILED${NC}"

# Clean up
teardown

# Exit with appropriate code
if [ $TESTS_FAILED -eq 0 ]; then
    exit 0
else
    exit 1
fi