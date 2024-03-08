#!/bin/bash

# Check if the correct number of arguments are provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <folder_name>"
    exit 1
fi

# Check if the source folder exists
if [ ! -d "1000" ]; then
    echo "Source folder '1000' does not exist."
    exit 1
fi

# Create the destination folder
mkdir -p "$1"

# Copy contents from source folder to destination folder
cp -r "1000"/* "$1"

echo "Contents copied from '1000' to '$1'."

