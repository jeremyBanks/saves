#!/bin/bash
set -euxo pipefail;

quiet() {
    "$@" &> /dev/null
}

cd ~/Library/"Application Support"/Celeste/Saves &> /dev/null || \
cd /mnt/d/Program\ Files/Celeste/Saves &> /dev/null; 

user="$(whoami)@$(hostname)";

quiet git commit -m "⚠ $user old staged" --allow-empty-message || true;

quiet git commit . -m "⚠ $user old unstaged" --allow-empty-message || true;

quiet git fetch;

quiet git pull --ff-only || (
    echo "⚠ Sync conflict. Archiving remote data and replacing with local." &&
    git pull -s ours --no-edit);

quiet git push;

echo "✅ Synced"

echo "🍓 Celeste";

quiet open -W ~/Library/"Application Support"/itch/apps/celeste/Celeste.app || \
quiet open -W /Applications/Celeste.app || \
/init /mnt/d/Program\ Files/Celeste/Celeste.exe;

if quiet git commit . -m "🍓 $user" --allow-empty-message; then
    quiet git push && echo "✅ Synced" || echo "⚠ Sync failed";
else
    echo "🆗 No changes to sync"
fi
