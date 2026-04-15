# Notepad

## Setup

```bash
# Install rbenv
brew install rbenv
# Add rbenv to shell environment
rbenv init
# Open a new terminal, the following should show what's in the ./.ruby-version file
ruby --version
# Install dependencies
bundle install
```

## Workflow

```bash
# Serve the website locally
make serve
# Build the mdbooks and put them in the right directory to publish
make build
```