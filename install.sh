#!/usr/bin/env bash
set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_URL="https://github.com/m1guelpf/auto-commit.git"
BINARY_NAME="auto-commit"

# Error handler
err() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

# Success message
success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Info message
info() {
    echo -e "${BLUE}→ $1${NC}"
}

# Warning message
warn() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Check if a command exists
check_command() {
    local cmd="$1"
    local install_msg="$2"
    
    if ! command -v "$cmd" &> /dev/null; then
        err "$cmd is not installed. $install_msg"
    fi
}

# Check prerequisites
check_prerequisites() {
    info "Checking prerequisites..."
    
    # Check for git
    check_command "git" "Please install git first."
    
    # Check for cargo/rust
    if ! command -v cargo &> /dev/null; then
        warn "Rust is not installed."
        info "Installing Rust..."
        
        # Install rustup
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        
        # Source cargo env
        source "$HOME/.cargo/env"
        
        # Verify installation
        if ! command -v cargo &> /dev/null; then
            err "Failed to install Rust. Please install manually from https://rustup.rs/"
        fi
        
        success "Rust installed successfully"
    else
        success "Rust is already installed"
    fi
    
    # Check for curl
    check_command "curl" "Please install curl first."
}

# Clone repository
clone_repository() {
    info "Cloning repository..."
    
    # Create temporary directory
    local temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT
    
    # Clone repo
    if git clone "$REPO_URL" "$temp_dir/$BINARY_NAME" &> /dev/null; then
        success "Repository cloned"
        echo "$temp_dir/$BINARY_NAME"
    else
        err "Failed to clone repository from $REPO_URL"
    fi
}

# Build project
build_project() {
    local project_dir="$1"
    
    info "Building project..."
    
    cd "$project_dir"
    
    # Build in release mode
    if cargo build --release &> /dev/null; then
        success "Project built successfully"
        
        # Check if binary exists
        if [ -f "target/release/$BINARY_NAME" ]; then
            echo "target/release/$BINARY_NAME"
        else
            err "Binary not found after build"
        fi
    else
        err "Failed to build project. Check if you have all required dependencies."
    fi
}

# Install binary
install_binary() {
    local binary_path="$1"
    local bin_dir="${BIN_DIR:-$HOME/.bin}"
    
    info "Installing binary to $bin_dir..."
    
    # Create bin directory
    mkdir -p "$bin_dir"
    
    # Copy binary
    if cp "$binary_path" "$bin_dir/$BINARY_NAME"; then
        chmod +x "$bin_dir/$BINARY_NAME"
        success "Binary installed"
    else
        err "Failed to install binary"
    fi
    
    # Add to PATH if needed
    add_to_path "$bin_dir"
    
    # Verify installation
    if "$bin_dir/$BINARY_NAME" --version &> /dev/null; then
        local version=$("$bin_dir/$BINARY_NAME" --version 2>&1 || echo "unknown")
        success "Installation complete: $version"
    else
        warn "Binary installed but verification failed"
    fi
}

# Add directory to PATH
add_to_path() {
    local bin_dir="$1"
    local shell_profile=""
    
    # Detect shell profile
    case $SHELL in
        */zsh)
            shell_profile="$HOME/.zshrc"
            ;;
        */bash)
            shell_profile="$HOME/.bashrc"
            ;;
        */fish)
            shell_profile="$HOME/.config/fish/config.fish"
            ;;
        *)
            warn "Unknown shell: $SHELL"
            info "Please add $bin_dir to your PATH manually"
            return
            ;;
    esac
    
    # Check if already in PATH
    if [[ ":$PATH:" != *":$bin_dir:"* ]]; then
        info "Adding $bin_dir to PATH in $shell_profile"
        
        # Add to profile
        echo "" >> "$shell_profile"
        echo "# Added by auto-commit installer" >> "$shell_profile"
        echo "export PATH=\"\$PATH:$bin_dir\"" >> "$shell_profile"
        
        success "PATH updated. Please run: source $shell_profile"
    else
        success "$bin_dir is already in PATH"
    fi
}

# Main installation process
main() {
    echo -e "${BLUE}Auto-Commit Installer${NC}"
    echo "====================="
    echo
    
    # Check prerequisites
    check_prerequisites
    
    # Clone repository
    local project_dir=$(clone_repository)
    
    # Build project
    local binary_path=$(build_project "$project_dir")
    
    # Install binary
    install_binary "$binary_path"
    
    echo
    echo -e "${GREEN}Installation successful!${NC}"
    echo
    echo "To get started:"
    echo "  1. Set your DeepSeek API key:"
    echo "     export DEEPSEEK_API_KEY='your-api-key'"
    echo
    echo "  2. Stage some changes and run:"
    echo "     git add ."
    echo "     auto-commit"
    echo
}

# Run main function (skip if sourced for testing)
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    main "$@"
fi