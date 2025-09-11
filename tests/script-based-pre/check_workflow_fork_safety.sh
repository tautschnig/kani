#!/bin/bash
# Copyright Kani Contributors
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# This script checks that all workflow files that create PRs have the necessary checks
# to prevent them from running on forked repositories.

set -eu

# Change to the repository root
cd "$(git rev-parse --show-toplevel)"

# Files to check
FILES=(
  ".github/workflows/cargo-update.yml"
  ".github/workflows/cbmc-update.yml"
  ".github/workflows/toolchain-upgrade.yml"
)

# Check that each file has the necessary checks
for file in "${FILES[@]}"; do
  echo "Checking $file..."
  
  # Check for job-level repository check
  if ! grep -q "if: github.repository == 'model-checking/kani'" "$file"; then
    echo "ERROR: $file does not have a job-level repository check"
    exit 1
  fi
  
  # Check for step-level repository verification
  if ! grep -q "Verify repository" "$file"; then
    echo "ERROR: $file does not have a step-level repository verification"
    exit 1
  fi
  
  # Check for PR creation conditional check
  if ! grep -A 2 "Create Pull Request" "$file" | grep -q "if:.*github.repository == 'model-checking/kani'"; then
    echo "ERROR: $file does not have a conditional check for PR creation"
    exit 1
  fi
  
  echo "$file passes all checks"
done

echo "All workflow files have the necessary checks to prevent them from running on forked repositories"
exit 0