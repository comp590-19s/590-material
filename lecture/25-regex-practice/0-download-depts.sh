#!/bin/bash

# So that this script will be idempotent, meaning you
# can rerun it multiple times and have the same effect,
# we're deleting the data files you're tasked with 
# downloading below. You do not need to do anything here.
rm -rf ./data
mkdir -p data/depts

# === TODO ===
# Use `wget` command-line utility to download a web page to data folder:
# URL to Download:  http://catalog.unc.edu/courses/
# Target Output File: ./data/depts.html
# Hint: Search the manual of wget for the 'Output File' flag

wget # TODO: delete this comment and add the correct args
