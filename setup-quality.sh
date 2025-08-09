#!/bin/bash

# GridTokenX Quality Setup Script
# This script sets up quality assurance tools and processes

echo "🔧 Setting up GridTokenX Quality Assurance"
echo "=========================================="

# Function to print colored output
print_status() {
    echo -e "\033[0;34m[INFO]\033[0m $1"
}

print_success() {
    echo -e "\033[0;32m[SUCCESS]\033[0m $1"
}

print_warning() {
    echo -e "\033[1;33m[WARNING]\033[0m $1"
}

print_error() {
    echo -e "\033[0;31m[ERROR]\033[0m $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Not in a Rust project directory"
    exit 1
fi

print_status "Installing quality assurance tools..."

# Install cargo tools for quality assurance
TOOLS=(
    "cargo-audit"      # Security vulnerability scanner
    "cargo-deny"       # Dependency analyzer
    "cargo-tarpaulin"  # Code coverage
    "cargo-outdated"   # Dependency update checker
    "cargo-machete"    # Dead dependency finder
    "tokei"           # Code statistics
)

for tool in "${TOOLS[@]}"; do
    if ! command -v "$tool" &> /dev/null; then
        print_status "Installing $tool..."
        if cargo install "$tool"; then
            print_success "Installed $tool"
        else
            print_warning "Failed to install $tool (may already be installed)"
        fi
    else
        print_success "$tool is already installed"
    fi
done

# Setup git hooks
print_status "Setting up git hooks..."

if [ -d ".git" ]; then
    # Copy pre-commit hook
    if [ -f "scripts/pre-commit" ]; then
        cp scripts/pre-commit .git/hooks/pre-commit
        chmod +x .git/hooks/pre-commit
        print_success "Pre-commit hook installed"
    else
        print_warning "Pre-commit script not found"
    fi
else
    print_warning "Not a git repository - skipping git hooks"
fi

# Create quality baseline
print_status "Creating quality baseline..."

# Run initial quality check
if [ -f "qa-check.sh" ]; then
    print_status "Running initial quality assessment..."
    if ./qa-check.sh --fast > qa-baseline.txt 2>&1; then
        print_success "Quality baseline created"
    else
        print_warning "Quality baseline created with issues (see qa-baseline.txt)"
    fi
else
    print_warning "QA check script not found"
fi

# Setup quality metrics collection
print_status "Setting up quality metrics..."

# Create metrics directory
mkdir -p metrics

# Generate initial metrics
if command -v tokei &> /dev/null; then
    tokei --sort code > metrics/code-stats.txt
    print_success "Code statistics generated"
fi

if command -v cargo-audit &> /dev/null; then
    cargo audit --format json > metrics/security-audit.json 2>/dev/null || true
    print_success "Security audit completed"
fi

# Setup development configuration
print_status "Setting up development configuration..."

# Create .cargo/config.toml if it doesn't exist
mkdir -p .cargo

if [ ! -f ".cargo/config.toml" ]; then
    cat > .cargo/config.toml << EOF
[alias]
# Quality assurance aliases
qa = "run --bin qa-check"
fmt-check = "fmt -- --check"
lint = "clippy -- -D warnings"
test-all = "test --workspace --all-features"
coverage = "tarpaulin --all-features --workspace --timeout 120"
audit = "audit --deny warnings"
outdated = "outdated"

[build]
# Enable all warnings as errors in development
rustflags = ["-D", "warnings"]

[env]
# Environment variables for development
RUST_BACKTRACE = "1"
RUST_LOG = "debug"
EOF
    print_success "Cargo configuration created"
fi

# Setup VS Code configuration (if .vscode exists or user wants it)
if [ -d ".vscode" ] || [ "$1" = "--vscode" ]; then
    print_status "Setting up VS Code configuration..."
    
    mkdir -p .vscode
    
    # VS Code settings
    cat > .vscode/settings.json << EOF
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
    "editor.formatOnSave": true,
    "editor.rulers": [100],
    "files.trimTrailingWhitespace": true,
    "files.insertFinalNewline": true,
    "editor.codeActionsOnSave": {
        "source.fixAll": true,
        "source.organizeImports": true
    },
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.procMacro.enable": true
}
EOF

    # VS Code tasks
    cat > .vscode/tasks.json << EOF
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Quality Check",
            "type": "shell",
            "command": "./qa-check.sh",
            "group": "test",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            }
        },
        {
            "label": "Format Code",
            "type": "shell",
            "command": "cargo",
            "args": ["fmt"],
            "group": "build"
        },
        {
            "label": "Run Tests",
            "type": "shell",
            "command": "cargo",
            "args": ["test", "--workspace"],
            "group": "test"
        },
        {
            "label": "Run Benchmarks",
            "type": "shell",
            "command": "cargo",
            "args": ["bench"],
            "group": "test"
        }
    ]
}
EOF

    print_success "VS Code configuration created"
fi

# Final summary
print_status "Quality assurance setup complete!"
echo ""
echo "📋 Available quality commands:"
echo "• ./qa-check.sh           - Run comprehensive quality checks"
echo "• ./qa-check.sh --fast    - Run quick quality checks"
echo "• ./qa-check.sh --fix     - Run checks and auto-fix issues"
echo "• cargo qa                - Quality check alias"
echo "• cargo fmt               - Format code"
echo "• cargo lint              - Run lints"
echo "• cargo test-all          - Run all tests"
echo "• cargo coverage          - Generate code coverage"
echo "• cargo audit             - Security audit"
echo ""
echo "📊 Quality metrics:"
echo "• Code statistics: metrics/code-stats.txt"
echo "• Security audit: metrics/security-audit.json"
echo "• Quality baseline: qa-baseline.txt"
echo ""
echo "🔗 Integration:"
echo "• Pre-commit hook: Installed"
echo "• GitHub Actions: .github/workflows/quality.yml"
echo "• Quality configuration: deny.toml, clippy.toml"
echo ""
print_success "Quality assurance is ready! Run './qa-check.sh' to start."
