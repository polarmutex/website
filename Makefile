
cache:
	nix build --json .#site | jq -r '.[].outputs | to_entries[].value' | cachix push polarmutex
