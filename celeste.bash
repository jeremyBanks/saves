#!/bin/bash
set -euxo pipefail;

cd ~/Library/"Application Support"/Celeste/Saves || \
cd /mnt/d/Program\ Files/Celeste/Saves; 

user="$(whoami)@$(hostname)";

git commit -m "⚠ $user old staged" --allow-empty-message &> /dev/null || true;

git commit . -m "⚠ $user old unstaged" --allow-empty-message &> /dev/null || true;

git fetch &> /dev/null;

git pull --ff-only &> /dev/null || (
    echo "⚠ Sync conflict. Archiving remote data and replacing with local." &&
    git pull -s ours --no-edit);

git push &> /dev/null;

echo "✅ Synced"

echo "🍓 Celeste";

open -W ~/Library/"Application Support"/itch/apps/celeste/Celeste.app &> /dev/null || \
open -W /Applications/Celeste.app || \
/mnt/d/Program\ Files/Celeste/Celeste.exe;

if git commit . -m "🍓 $user" --allow-empty-message &> /dev/null; then
    git push &> /dev/null && echo "✅ Synced" || echo "⚠ Sync failed";
else
    echo "🆗 No changes to sync"
fi
