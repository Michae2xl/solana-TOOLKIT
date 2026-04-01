#!/bin/bash
set -e

echo "══════════════════════════════════════"
echo "  Solana TOOLKIT — Installer"
echo "══════════════════════════════════════"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

check() {
  if command -v "$1" &>/dev/null; then
    echo -e "  ${GREEN}✓${NC} $1 $(command $1 --version 2>/dev/null | head -1)"
  else
    echo -e "  ${RED}✗${NC} $1 — not installed"
    return 1
  fi
}

install_solana() {
  echo -e "\n${YELLOW}Installing Solana CLI...${NC}"
  sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
  export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
}

install_rust() {
  echo -e "\n${YELLOW}Installing Rust...${NC}"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
}

install_anchor() {
  echo -e "\n${YELLOW}Installing Anchor...${NC}"
  cargo install --git https://github.com/coral-xyz/anchor avm --force
  avm install 0.31.1
  avm use 0.31.1
}

install_lean() {
  echo -e "\n${YELLOW}Installing Lean 4 (elan)...${NC}"
  curl -sSf https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh | sh -s -- -y
  export PATH="$HOME/.elan/bin:$PATH"
}

install_qedgen() {
  echo -e "\n${YELLOW}Installing qedgen...${NC}"
  TMPDIR=$(mktemp -d)
  git clone --depth 1 https://github.com/qedgen/solana-skills.git "$TMPDIR/qedgen"
  cargo install --path "$TMPDIR/qedgen/crates/qedgen"
  rm -rf "$TMPDIR"
}

install_cargo_tools() {
  echo -e "\n${YELLOW}Installing cargo tools...${NC}"
  cargo install cargo-audit
  cargo install cargo-expand
}

install_solana_claude() {
  if [ -d ".claude" ]; then
    echo -e "\n${YELLOW}solana-claude already installed${NC}"
  else
    echo -e "\n${YELLOW}Installing solana-claude...${NC}"
    curl -fsSL https://raw.githubusercontent.com/solanabr/solana-claude/main/install.sh | bash
  fi
}

# Check existing installations
echo "Checking installed tools..."
echo ""

MISSING=0

check solana || MISSING=1
check rustc || MISSING=1
check anchor || MISSING=1
check lean || MISSING=1
check qedgen || MISSING=1
check cargo-audit || MISSING=1

echo ""

if [ "$MISSING" -eq 0 ]; then
  echo -e "${GREEN}All tools already installed!${NC}"
else
  echo -e "${YELLOW}Some tools are missing. Installing...${NC}"

  # Source cargo if available
  [ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"
  export PATH="$HOME/.local/share/solana/install/active_release/bin:$HOME/.avm/bin:$HOME/.elan/bin:$PATH"

  command -v rustc &>/dev/null || install_rust
  source "$HOME/.cargo/env" 2>/dev/null || true

  command -v solana &>/dev/null || install_solana
  command -v anchor &>/dev/null || install_anchor
  command -v lean &>/dev/null || install_lean
  command -v qedgen &>/dev/null || install_qedgen
  command -v cargo-audit &>/dev/null || install_cargo_tools
fi

# Install solana-claude config
install_solana_claude

echo ""
echo "══════════════════════════════════════"
echo -e "  ${GREEN}Solana TOOLKIT — Ready!${NC}"
echo "══════════════════════════════════════"
echo ""
echo "Next steps:"
echo "  1. cp .env.example .env"
echo "  2. Edit .env with your Helius API key"
echo "  3. Run 'claude' to start with AI agents"
echo "  4. Try /build-program or /audit-solana"
echo ""
