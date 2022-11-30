# Maintaining Quasar Proto Files
All Quasar proto files are defined here. We use buf (http://buf.build) to manage proto dependencies and generate go files and openapi docs from proto files.
## Updating the dependencies of third_party proto
To update the proto dependencies run the following commands at the root of repository:
```bash
# Update the deps hash in buf.yaml
cd proto
buf mod update

# Generate go from proto files
cd ..
make proto-gen

# Generate swagger/openapi docs from proto files
make proto-doc
```