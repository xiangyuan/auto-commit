#!/usr/bin/env bash
# Auto-Commit-改 (Fork) Installer with EXTREME DEBUGGING
# This installer is specifically for the forked version with DeepSeek integration

set -e

# ===== DEBUG SETTINGS =====
DEBUG_MODE="${DEBUG:-0}"
DEBUG_LOG="install_debug_$(date +%Y%m%d_%H%M%S).log"
VERBOSE="${VERBOSE:-0}"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m' # No Color

# ===== DEBUG FUNCTIONS =====
debug() {
    if [[ "$DEBUG_MODE" == "1" ]]; then
        echo -e "${GRAY}[DEBUG $(date +%H:%M:%S)] $1${NC}" | tee -a "$DEBUG_LOG"
    fi
}

debug_var() {
    local var_name="$1"
    local var_value="${!var_name}"
    debug "$var_name='$var_value'"
}

debug_cmd() {
    local cmd="$1"
    debug "Running command: $cmd"
    if [[ "$DEBUG_MODE" == "1" ]]; then
        eval "$cmd" 2>&1 | tee -a "$DEBUG_LOG"
    else
        eval "$cmd"
    fi
}

verbose() {
    if [[ "$VERBOSE" == "1" ]] || [[ "$DEBUG_MODE" == "1" ]]; then
        echo -e "${CYAN}[VERBOSE] $1${NC}"
    fi
}

# ===== LOGGING FUNCTIONS =====
log_section() {
    echo -e "\n${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${MAGENTA}▶ $1${NC}"
    echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    debug "Starting section: $1"
}

err() {
    echo -e "${RED}❌ Error: $1${NC}" >&2
    debug "ERROR: $1"
    debug "Stack trace:"
    if [[ "$DEBUG_MODE" == "1" ]]; then
        local frame=0
        while caller $frame; do
            ((frame++))
        done | tee -a "$DEBUG_LOG"
    fi
    exit 1
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
    debug "SUCCESS: $1"
}

info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
    debug "INFO: $1"
}

warn() {
    echo -e "${YELLOW}⚠️  $1${NC}"
    debug "WARNING: $1"
}

# ===== CONFIGURATION =====
# Fork-specific settings
FORK_OWNER="${FORK_OWNER:-YOUR_GITHUB_USERNAME}"  # Replace with your username
FORK_REPO="${FORK_REPO:-auto-commit}"
REPO_URL="https://github.com/${FORK_OWNER}/${FORK_REPO}.git"
BINARY_NAME="auto-commit"
PROJECT_NAME="auto-commit-改"

# Installation directories
BIN_DIR="${BIN_DIR:-$HOME/.bin}"
CONFIG_DIR="${CONFIG_DIR:-$HOME/.config/auto-commit}"
TEMP_DIR=""

# ===== SYSTEM DETECTION =====
detect_system() {
    log_section "System Detection"
    
    # OS Detection
    OS="$(uname -s)"
    debug_var "OS"
    
    case "$OS" in
        Linux*)     OS_TYPE="Linux";;
        Darwin*)    OS_TYPE="macOS";;
        CYGWIN*)    OS_TYPE="Cygwin";;
        MINGW*)     OS_TYPE="MinGW";;
        *)          OS_TYPE="Unknown";;
    esac
    info "Operating System: $OS_TYPE"
    debug_var "OS_TYPE"
    
    # Architecture Detection
    ARCH="$(uname -m)"
    debug_var "ARCH"
    info "Architecture: $ARCH"
    
    # Shell Detection
    SHELL_TYPE="$(basename "$SHELL")"
    debug_var "SHELL_TYPE"
    info "Shell: $SHELL_TYPE"
    
    # Package Manager Detection
    if command -v apt-get &> /dev/null; then
        PKG_MANAGER="apt"
    elif command -v yum &> /dev/null; then
        PKG_MANAGER="yum"  
    elif command -v brew &> /dev/null; then
        PKG_MANAGER="brew"
    elif command -v pacman &> /dev/null; then
        PKG_MANAGER="pacman"
    else
        PKG_MANAGER="unknown"
    fi
    debug_var "PKG_MANAGER"
    info "Package Manager: $PKG_MANAGER"
}

# ===== PREREQUISITE CHECKS =====
check_prerequisites() {
    log_section "Prerequisites Check"
    
    local missing_deps=()
    
    # Check Git
    verbose "Checking for git..."
    if ! command -v git &> /dev/null; then
        missing_deps+=("git")
        warn "Git is not installed"
    else
        success "Git $(git --version | awk '{print $3}')"
        debug "Git path: $(which git)"
    fi
    
    # Check Curl
    verbose "Checking for curl..."
    if ! command -v curl &> /dev/null; then
        missing_deps+=("curl")
        warn "Curl is not installed"
    else
        success "Curl $(curl --version | head -1 | awk '{print $2}')"
        debug "Curl path: $(which curl)"
    fi
    
    # Check Rust/Cargo
    verbose "Checking for Rust/Cargo..."
    if ! command -v cargo &> /dev/null; then
        warn "Rust is not installed"
        INSTALL_RUST=1
    else
        success "Cargo $(cargo --version | awk '{print $2}')"
        debug "Cargo path: $(which cargo)"
        debug "Rust toolchain: $(rustc --version)"
        INSTALL_RUST=0
    fi
    
    # Install missing dependencies
    if [ ${#missing_deps[@]} -gt 0 ]; then
        warn "Missing dependencies: ${missing_deps[*]}"
        
        if [[ "$PKG_MANAGER" != "unknown" ]]; then
            info "Installing missing dependencies..."
            
            case "$PKG_MANAGER" in
                apt)
                    debug_cmd "sudo apt-get update"
                    debug_cmd "sudo apt-get install -y ${missing_deps[*]}"
                    ;;
                brew)
                    debug_cmd "brew install ${missing_deps[*]}"
                    ;;
                pacman)
                    debug_cmd "sudo pacman -S --noconfirm ${missing_deps[*]}"
                    ;;
                yum)
                    debug_cmd "sudo yum install -y ${missing_deps[*]}"
                    ;;
            esac
        else
            err "Please install missing dependencies manually: ${missing_deps[*]}"
        fi
    fi
    
    # Install Rust if needed
    if [[ "$INSTALL_RUST" == "1" ]]; then
        info "Installing Rust..."
        debug "Downloading rustup installer..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh
        debug "Running rustup installer..."
        sh /tmp/rustup.sh -y
        
        # Source cargo env
        source "$HOME/.cargo/env"
        
        if ! command -v cargo &> /dev/null; then
            err "Failed to install Rust"
        fi
        success "Rust installed successfully"
    fi
}

# ===== REPOSITORY SETUP =====
setup_repository() {
    log_section "Repository Setup"
    
    # Create temp directory
    TEMP_DIR=$(mktemp -d)
    debug_var "TEMP_DIR"
    
    # Set trap for cleanup
    trap cleanup EXIT
    
    info "Cloning repository from $REPO_URL"
    debug "Clone destination: $TEMP_DIR/$BINARY_NAME"
    
    if ! git clone "$REPO_URL" "$TEMP_DIR/$BINARY_NAME" 2>&1 | while read -r line; do
        verbose "$line"
    done; then
        # Fallback to original repo if fork doesn't exist
        warn "Failed to clone fork, trying original repository..."
        REPO_URL="https://github.com/m1guelpf/auto-commit.git"
        debug_var "REPO_URL"
        
        if ! git clone "$REPO_URL" "$TEMP_DIR/$BINARY_NAME"; then
            err "Failed to clone repository"
        fi
    fi
    
    success "Repository cloned"
    
    cd "$TEMP_DIR/$BINARY_NAME"
    debug "Current directory: $(pwd)"
    debug "Repository contents:"
    if [[ "$DEBUG_MODE" == "1" ]]; then
        ls -la | tee -a "$DEBUG_LOG"
    fi
}

# ===== BUILD PROJECT =====
build_project() {
    log_section "Building Project"
    
    info "Running cargo build --release"
    verbose "This may take a few minutes..."
    
    # Set RUST_BACKTRACE for debugging
    export RUST_BACKTRACE=1
    debug_var "RUST_BACKTRACE"
    
    # Build with verbose output if debug mode
    if [[ "$DEBUG_MODE" == "1" ]]; then
        cargo build --release --verbose 2>&1 | tee -a "$DEBUG_LOG"
    else
        cargo build --release
    fi
    
    if [ ! -f "target/release/$BINARY_NAME" ]; then
        err "Build failed: binary not found"
    fi
    
    success "Build completed successfully"
    debug "Binary location: $(pwd)/target/release/$BINARY_NAME"
    debug "Binary size: $(du -h target/release/$BINARY_NAME | cut -f1)"
}

# ===== INSTALL BINARY =====
install_binary() {
    log_section "Installing Binary"
    
    # Create directories
    mkdir -p "$BIN_DIR"
    mkdir -p "$CONFIG_DIR"
    
    debug_var "BIN_DIR"
    debug_var "CONFIG_DIR"
    
    # Copy binary
    info "Installing binary to $BIN_DIR"
    cp "target/release/$BINARY_NAME" "$BIN_DIR/"
    chmod +x "$BIN_DIR/$BINARY_NAME"
    
    success "Binary installed"
    
    # Verify installation
    if "$BIN_DIR/$BINARY_NAME" --version &> /dev/null; then
        local version=$("$BIN_DIR/$BINARY_NAME" --version)
        success "Version: $version"
    else
        warn "Binary installed but version check failed"
    fi
}

# ===== ENVIRONMENT SETUP =====
setup_environment() {
    log_section "Environment Setup"
    
    # Create .env file template
    info "Creating .env template"
    cat > "$CONFIG_DIR/.env.example" << 'EOF'
# DeepSeek API Configuration
export DEEPSEEK_API_KEY='sk-your-deepseek-api-key-here'

# Optional: Custom settings
# export AUTO_COMMIT_FORMAT='{prefix}: {emoji} {title}'
# export AUTO_COMMIT_TIMEOUT=30
EOF
    
    debug "Created $CONFIG_DIR/.env.example"
    
    # Check for existing .env
    if [ -f "$HOME/.env" ] && grep -q "DEEPSEEK_API_KEY" "$HOME/.env"; then
        info "Found existing .env with DEEPSEEK_API_KEY"
    elif [ -f "src/.env" ] && grep -q "DEEPSEEK_API_KEY" "src/.env"; then
        info "Found src/.env with DEEPSEEK_API_KEY" 
        cp "src/.env" "$CONFIG_DIR/.env"
        success "Copied existing configuration"
    else
        warn "No DEEPSEEK_API_KEY found in environment"
        info "Please set up your API key in $CONFIG_DIR/.env"
    fi
    
    # Setup shell integration
    setup_shell_integration
}

# ===== SHELL INTEGRATION =====
setup_shell_integration() {
    log_section "Shell Integration"
    
    local shell_config=""
    local shell_type="$(basename "$SHELL")"
    
    case "$shell_type" in
        bash)
            shell_config="$HOME/.bashrc"
            ;;
        zsh)
            shell_config="$HOME/.zshrc"
            ;;
        fish)
            shell_config="$HOME/.config/fish/config.fish"
            ;;
        *)
            warn "Unknown shell: $shell_type"
            return
            ;;
    esac
    
    debug_var "shell_config"
    
    # Add to PATH
    if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
        info "Adding $BIN_DIR to PATH"
        echo "" >> "$shell_config"
        echo "# Added by $PROJECT_NAME installer" >> "$shell_config"
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$shell_config"
        success "PATH updated"
    else
        info "$BIN_DIR already in PATH"
    fi
    
    # Add auto-completion (if available)
    if [ -f "completions/$BINARY_NAME.bash" ] && [ "$shell_type" = "bash" ]; then
        cp "completions/$BINARY_NAME.bash" "$CONFIG_DIR/"
        echo "source $CONFIG_DIR/$BINARY_NAME.bash" >> "$shell_config"
        success "Bash completion installed"
    fi
    
    # Add convenience alias
    echo "alias ac='$BINARY_NAME'" >> "$shell_config"
    success "Added 'ac' alias for $BINARY_NAME"
}

# ===== CLEANUP =====
cleanup() {
    if [ -n "$TEMP_DIR" ] && [ -d "$TEMP_DIR" ]; then
        debug "Cleaning up $TEMP_DIR"
        rm -rf "$TEMP_DIR"
    fi
}

# ===== POST INSTALL =====
post_install() {
    log_section "Post Installation"
    
    echo
    echo -e "${GREEN}🎉 Installation Complete!${NC}"
    echo
    echo -e "${CYAN}Next Steps:${NC}"
    echo "1. Set your DeepSeek API key:"
    echo "   export DEEPSEEK_API_KEY='your-key-here'"
    echo "   # Or add to $CONFIG_DIR/.env"
    echo
    echo "2. Reload your shell configuration:"
    echo "   source $(get_shell_config)"
    echo
    echo "3. Test the installation:"
    echo "   $BINARY_NAME --help"
    echo
    echo "4. Use the tool:"
    echo "   git add ."
    echo "   $BINARY_NAME"
    echo
    
    if [[ "$DEBUG_MODE" == "1" ]]; then
        echo -e "${GRAY}Debug log saved to: $DEBUG_LOG${NC}"
    fi
}

get_shell_config() {
    case "$(basename "$SHELL")" in
        bash) echo "$HOME/.bashrc";;
        zsh) echo "$HOME/.zshrc";;
        fish) echo "$HOME/.config/fish/config.fish";;
        *) echo "$HOME/.profile";;
    esac
}

# ===== MAIN INSTALLATION =====
main() {
    echo -e "${MAGENTA}"
    echo "╔═══════════════════════════════════════════════════════╗"
    echo "║          Auto-Commit-改 (DeepSeek Fork)               ║"
    echo "║              Advanced Installer v2.0                   ║"
    echo "╚═══════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    if [[ "$DEBUG_MODE" == "1" ]]; then
        echo -e "${GRAY}Debug mode enabled. Log: $DEBUG_LOG${NC}"
    fi
    
    # Run installation steps
    detect_system
    check_prerequisites
    setup_repository
    build_project
    install_binary
    setup_environment
    post_install
}

# ===== SCRIPT ENTRY POINT =====
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --debug|-d)
                DEBUG_MODE=1
                shift
                ;;
            --verbose|-v)
                VERBOSE=1
                shift
                ;;
            --fork-owner)
                FORK_OWNER="$2"
                shift 2
                ;;
            --help|-h)
                echo "Usage: $0 [options]"
                echo "Options:"
                echo "  --debug, -d        Enable debug mode"
                echo "  --verbose, -v      Enable verbose output"
                echo "  --fork-owner USER  Specify fork owner (default: YOUR_GITHUB_USERNAME)"
                echo "  --help, -h         Show this help"
                exit 0
                ;;
            *)
                warn "Unknown option: $1"
                shift
                ;;
        esac
    done
    
    main "$@"
fi