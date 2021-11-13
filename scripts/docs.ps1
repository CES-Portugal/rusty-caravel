echo "Creating documentation"
cargo doc --no-deps

echo "Deploying to ./docs"
cp -r .\target\doc\* .\docs\
cp -r .\public\* .\docs\
