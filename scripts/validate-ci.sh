#!/bin/bash
# Validate CI/CD setup locally before pushing

set -e

echo "Validating CI/CD setup..."
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track failures
FAILED=0

# Function to run checks
check() {
    local name=$1
    local command=$2
    
    echo -n "Checking $name... "
    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${RED}✗${NC}"
        FAILED=$((FAILED + 1))
    fi
}

# Check required tools
echo "Checking required tools..."
check "Rust" "rustc --version"
check "Cargo" "cargo --version"
check "wasm-pack" "wasm-pack --version"
check "Node.js" "node --version"
check "Yarn" "yarn --version"
echo ""

# Check Rust formatting
echo "Checking Rust formatting..."
if cargo fmt --all -- --check; then
    echo -e "${GREEN}✓ Code is properly formatted${NC}"
else
    echo -e "${RED}✗ Code needs formatting. Run: cargo fmt --all${NC}"
    FAILED=$((FAILED + 1))
fi
echo ""

# Check Clippy
echo "Running Clippy..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo -e "${GREEN}✓ No Clippy warnings${NC}"
else
    echo -e "${RED}✗ Clippy found issues${NC}"
    FAILED=$((FAILED + 1))
fi
echo ""

# Run tests
echo "Running tests..."
echo "  Testing stegano-core..."
if (cd stegano-core && cargo test --quiet); then
    echo -e "  ${GREEN}✓ Core tests passed${NC}"
else
    echo -e "  ${RED}✗ Core tests failed${NC}"
    FAILED=$((FAILED + 1))
fi

echo "  Testing stegano-cli..."
if (cd stegano-cli && cargo test --quiet); then
    echo -e "  ${GREEN}✓ CLI tests passed${NC}"
else
    echo -e "  ${RED}✗ CLI tests failed${NC}"
    FAILED=$((FAILED + 1))
fi
echo ""

# Build WASM
echo "Building WASM..."
if (cd stegano-wasm && wasm-pack build --target web > /dev/null 2>&1); then
    echo -e "${GREEN}✓ WASM build successful${NC}"
else
    echo -e "${RED}✗ WASM build failed${NC}"
    FAILED=$((FAILED + 1))
fi
echo ""

# Check web app dependencies
echo "Checking web app dependencies..."
if (cd web-app && yarn install --frozen-lockfile > /dev/null 2>&1); then
    echo -e "${GREEN}✓ Dependencies installed${NC}"
else
    echo -e "${YELLOW}⚠ Dependencies may need updating${NC}"
fi
echo ""

# Build web app
echo "Building web app..."
if (cd web-app && yarn build > /dev/null 2>&1); then
    echo -e "${GREEN}✓ Web app build successful${NC}"
else
    echo -e "${RED}✗ Web app build failed${NC}"
    FAILED=$((FAILED + 1))
fi
echo ""

# Check GitHub workflows syntax
echo "Checking workflow files..."
for workflow in .github/workflows/*.yml; do
    if [ -f "$workflow" ]; then
        echo -n "  $(basename $workflow)... "
        # Basic YAML syntax check (requires Python)
        if command -v python3 > /dev/null 2>&1; then
            if python3 -c "import yaml; yaml.safe_load(open('$workflow'))" 2>/dev/null; then
                echo -e "${GREEN}✓${NC}"
            else
                echo -e "${RED}✗ Invalid YAML${NC}"
                FAILED=$((FAILED + 1))
            fi
        else
            echo -e "${YELLOW}⚠ (skipped - python3 not found)${NC}"
        fi
    fi
done
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All checks passed! Ready to push.${NC}"
    exit 0
else
    echo -e "${RED} $FAILED check(s) failed. Please fix before pushing.${NC}"
    exit 1
fi

