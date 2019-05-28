#!/bin/bash

# Install Hub
HUB_DIR=hub-linux-amd64-2.11.2
curl -sL "https://github.com/github/hub/releases/download/v2.11.2/$HUB_DIR.tgz" | tar xz

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
git push --set-upstream origin rust-$TOOLCHAIN

# Create pull request
./$HUB_DIR/bin/hub pull-request -m "$MESSAGE" --reviewer benbrandt

# Cleanup
git checkout master
rm -rf $HUB_DIR
