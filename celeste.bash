#!/bin/bash
set -euxo pipefail;

-q() {
    "$@" &> /dev/null
}

cd ~/Library/"Application Support"/Celeste/Saves &> /dev/null || \
cd /mnt/d/Program\ Files/Celeste/Saves &> /dev/null; 

user="$(whoami)@$(hostname)";

-q git commit -m "⚠ $user old staged" --allow-empty-message || true;

-q git commit . -m "⚠ $user old unstaged" --allow-empty-message || true;

-q git fetch;

-q git pull --ff-only || (
    echo "⚠ Sync conflict. Archiving remote data and replacing with local." &&
    git pull -s ours --no-edit);

-q git push;

echo "✅ Synced"

echo "🍓 Celeste";

-q open -W ~/Library/"Application Support"/itch/apps/celeste/Celeste.app || \
-q open -W /Applications/Celeste.app || \
/init /mnt/d/Program\ Files/Celeste/Celeste.exe;

if -q git commit . -m "🍓 $user" --allow-empty-message; then
    -q git push && echo "✅ Synced" || echo "⚠ Sync failed";
else
    echo "🆗 No changes to sync"
fi
