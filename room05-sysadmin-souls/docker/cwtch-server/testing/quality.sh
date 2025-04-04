#!/bin/sh

echo "Checking code quality (you want to see no output here)"
echo ""

echo "Vetting:"
go list ./... | xargs go vet

echo ""
echo "Linting:"

go list ./... | staticcheck ./...

echo "Running nilaway..."
nilaway -include-pkgs="cwtch.im/server,cwtch.im/cwtch,cwtch.im/tapir,git.openprivacy.ca/openprivacy/connectivity" -exclude-file-docstrings="nolint:nilaway" ./...


echo "Time to format"
gofmt -l -s -w .

# misspell (https://github.com/client9/misspell/cmd/misspell)
echo "Checking for misspelled words..."
misspell . | grep -v "vendor/" | grep -v "go.sum" | grep -v ".idea"
