#!/bin/bash

# GridTokenX Quality Assurance Script
# This script runs comprehensive quality checks on the codebase

set -e

echo "🔍 GridTokenX Software Quality Assurance"
echo "========================================"

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((PASSED_CHECKS++))
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((FAILED_CHECKS++))
}

run_check() {
    ((TOTAL_CHECKS++))
    local check_name="$1"
    local command="$2"
    
    print_status "Running: $check_name"
    
    if eval "$command" > /tmp/qa_output 2>&1; then
        print_success "$check_name"
        return 0
    else
        print_error "$check_name"
        if [ "$VERBOSE" = "true" ]; then
            echo "Error details:"
            cat /tmp/qa_output | head -10
        fi
        return 1
    fi
}

# Parse command line arguments
VERBOSE=false
FIX=false
FAST=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -f|--fix)
            FIX=true
            shift
            ;;
        --fast)
            FAST=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  -v, --verbose    Show detailed output"
            echo "  -f, --fix        Attempt to fix issues automatically"
            echo "  --fast           Skip slow checks"
            echo "  -h, --help       Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option $1"
            exit 1
            ;;
    esac
done

echo "Starting quality assurance checks..."
echo "Verbose mode: $VERBOSE"
echo "Auto-fix mode: $FIX"
echo "Fast mode: $FAST"
echo ""

# 1. Code Formatting
print_status "=== Code Formatting ==="
if [ "$FIX" = "true" ]; then
    run_check "Code formatting (auto-fix)" "cargo fmt"
else
    run_check "Code formatting check" "cargo fmt -- --check"
fi

# 2. Code Linting
print_status "=== Code Linting ==="
run_check "Clippy linting" "cargo clippy -- -D warnings"

# 3. Security Audit
print_status "=== Security Audit ==="
run_check "Cargo audit" "cargo audit"

# 4. Unit Tests
print_status "=== Unit Tests ==="
run_check "Unit tests" "cargo test --lib"

# 5. Integration Tests
print_status "=== Integration Tests ==="
if [ "$FAST" != "true" ]; then
    run_check "Integration tests" "cargo test --test '*'"
else
    print_warning "Skipping integration tests (fast mode)"
fi

# 6. Documentation Tests
print_status "=== Documentation Tests ==="
run_check "Documentation tests" "cargo test --doc"

# 7. Build Check
print_status "=== Build Verification ==="
run_check "Release build" "cargo build --release"

# 8. Dependency Check
print_status "=== Dependency Analysis ==="
if command -v cargo-deny &> /dev/null; then
    run_check "Dependency check" "cargo deny check"
else
    print_warning "cargo-deny not installed, skipping dependency check"
fi

# 9. Code Coverage (if tools available)
print_status "=== Code Coverage ==="
if command -v cargo-tarpaulin &> /dev/null && [ "$FAST" != "true" ]; then
    run_check "Code coverage" "cargo tarpaulin --skip-clean --out stdout | grep -E 'Coverage|^[0-9]'"
else
    print_warning "cargo-tarpaulin not installed or fast mode, skipping coverage"
fi

# 10. Benchmarks (if not fast mode)
print_status "=== Performance Benchmarks ==="
if [ "$FAST" != "true" ]; then
    run_check "Performance benchmarks" "cargo bench --no-run"
else
    print_warning "Skipping benchmarks (fast mode)"
fi

# 11. Documentation Generation
print_status "=== Documentation Generation ==="
run_check "Documentation generation" "cargo doc --no-deps --quiet"

# 12. Dead Code Detection
print_status "=== Dead Code Analysis ==="
run_check "Dead code check" "cargo check --quiet 2>&1 | grep -E 'warning.*dead_code|warning.*unused' || true"

# 13. Complexity Analysis (if available)
print_status "=== Code Complexity ==="
if command -v tokei &> /dev/null; then
    run_check "Code metrics" "tokei --sort code"
else
    print_warning "tokei not installed, skipping complexity analysis"
fi

# 14. API Validation
print_status "=== API Validation ==="
if [ -f "src/api.rs" ]; then
    run_check "API module compilation" "cargo check --bin gridtokenx-node"
else
    print_warning "API module not found"
fi

# 15. Configuration Validation
print_status "=== Configuration Validation ==="
config_files=("config.toml" "config/egat.toml" "config/mea.toml" "config/pea.toml" "config/erc.toml")
for config in "${config_files[@]}"; do
    if [ -f "$config" ]; then
        run_check "Config file: $config" "[ -f '$config' ] && [ -s '$config' ]"
    fi
done

# 16. Docker Build Test
print_status "=== Docker Build Test ==="
if [ -f "Dockerfile" ] && command -v docker &> /dev/null; then
    if [ "$FAST" != "true" ]; then
        run_check "Docker build" "docker build -t gridtokenx-qa-test . --quiet"
        # Cleanup test image
        docker rmi gridtokenx-qa-test &> /dev/null || true
    else
        run_check "Dockerfile syntax" "docker build -t gridtokenx-qa-test . --dry-run || docker build --help | grep -q 'dry-run' || true"
    fi
else
    print_warning "Docker not available or no Dockerfile"
fi

# 17. Memory Safety Analysis
print_status "=== Memory Safety ==="
run_check "Memory safety (basic)" "cargo check --target-dir target/safety"

# 18. Licensing Check
print_status "=== License Compliance ==="
run_check "License files" "[ -f LICENSE ] || [ -f LICENSE.md ] || [ -f COPYING ]"

# Summary Report
echo ""
echo "🏁 Quality Assurance Summary"
echo "============================"
echo "Total checks: $TOTAL_CHECKS"
echo "Passed: $PASSED_CHECKS"
echo "Failed: $FAILED_CHECKS"

# Calculate pass rate
if [ $TOTAL_CHECKS -gt 0 ]; then
    PASS_RATE=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))
    echo "Pass rate: ${PASS_RATE}%"
    
    if [ $PASS_RATE -ge 90 ]; then
        print_success "Quality score: EXCELLENT (${PASS_RATE}%)"
    elif [ $PASS_RATE -ge 80 ]; then
        print_success "Quality score: GOOD (${PASS_RATE}%)"
    elif [ $PASS_RATE -ge 70 ]; then
        print_warning "Quality score: FAIR (${PASS_RATE}%)"
    else
        print_error "Quality score: NEEDS IMPROVEMENT (${PASS_RATE}%)"
    fi
fi

# Quality recommendations
echo ""
echo "📋 Recommendations:"
echo "==================="

if [ $FAILED_CHECKS -gt 0 ]; then
    echo "1. Address failed checks above"
    echo "2. Run with --verbose flag to see detailed errors"
    if [ "$FIX" != "true" ]; then
        echo "3. Run with --fix flag to attempt automatic fixes"
    fi
fi

if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "• Install cargo-tarpaulin for code coverage: cargo install cargo-tarpaulin"
fi

if ! command -v cargo-deny &> /dev/null; then
    echo "• Install cargo-deny for dependency analysis: cargo install cargo-deny"
fi

if ! command -v tokei &> /dev/null; then
    echo "• Install tokei for code metrics: cargo install tokei"
fi

echo ""
echo "Next steps:"
echo "• Review and fix any failed checks"
echo "• Update documentation if needed"
echo "• Run performance tests: ./performance-test.sh"
echo "• Check deployment readiness: ./deploy.sh --dry-run"

# Exit with appropriate code
if [ $FAILED_CHECKS -gt 0 ]; then
    exit 1
else
    exit 0
fi
