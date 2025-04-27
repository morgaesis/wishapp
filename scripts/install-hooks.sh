#!/bin/sh
echo "Installing git hooks..."
cp .githooks/pre-push .git/hooks/pre-push
chmod +x .git/hooks/pre-push
echo "Hooks installed successfully"