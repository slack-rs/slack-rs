#!/bin/bash
# update docs, requires ghp-import
cargo doc
REPO_SLUG=slack-rs/slack-rs
echo "<meta http-equiv=refresh content=0;url=slack/index.html>" > target/doc/index.html
ghp-import -n target/doc
git push -fq https://github.com/${REPO_SLUG}.git gh-pages
