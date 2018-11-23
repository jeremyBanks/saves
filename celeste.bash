#!/bin/bash -euo pipefail

cd ~/Library/"Application Support"/Celeste/Saves;

git commit -m "" --allow-empty-message &> /dev/null || true;

git commit . -m "" --allow-empty-message &> /dev/null || true;

git fetch &> /dev/null;

(git pull --ff-only &> /dev/null && echo "Synced") || (
    echo "Sync conflict. Archiving remote data and replacing with local." &&
    git pull -s ours --no-edit);

git push &> /dev/null;

open -W ~/Library/"Application Support"/itch/apps/celeste/Celeste.app &> /dev/null || open -W /Applications/Celeste.app;

git commit . -m "" --allow-empty-message &> /dev/null &&
    echo "Committed new changes." || echo "No changes to commit.";

git push &> /dev/null && echo "Synced.";
