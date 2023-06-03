#!/bin/bash

platform=`uname`

if [ "$platform" = "Linux" ]
then
    install_path="/usr/bin/analyzer.py"
fi
if [ "$platform" = "Darwin" ]
then
    install_path="/usr/local/bin/analyzer.py"
fi

echo "downloading..."
repo="ali77gh/ProjectAnalyzer"
tag_name=$(curl --silent https://api.github.com/repos/$repo/releases/latest \
                  | grep '"tag_name"' \
                  | sed --regexp-extended 's/.*"([^"]+)".*/\1/')
curl -sfL "https://github.com/$repo/releases/download/$tag_name/analyze.py" --output temp.py
echo "done"

echo "installing..."
mv temp.py "$install_path"
echo "done"

echo "making script executable..."
chmod +x "$install_path"
echo "done"

echo "analyze.py installed successfully"
analyzer.py help
