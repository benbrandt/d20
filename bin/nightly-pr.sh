#!/bin/bash

# YYYY-mm-dd format
CURRENT_DATE=`date +"%F"`
TOOLCHAIN="nightly-$CURRENT_DATE"
MESSAGE="Update toolchain to $TOOLCHAIN"

# Checkout a branch for today
git checkout -b rust-$TOOLCHAIN

# Save the date to the toolchain file
echo $TOOLCHAIN > 'rust-toolchain'

# Stage and commit the change
git add 'rust-toolchain'
git commit -m "$MESSAGE"

# Create pull request
# Make sure `hub` is installed
hub pull-request -m "$MESSAGE" --push --reviewer benbrandt --labels rust,toolchain

# Cleanup
git checkout master
