#!/bin/bash
set -euo pipefail;

-q() {
    "$@" 1> /dev/null
}

-qq() {
    "$@" 1> /dev/null 2> /dev/null
}

cd ~/Library/"Application Support"/Celeste/Saves &> /dev/null || \
cd /mnt/d/Program\ Files/Celeste/Saves &> /dev/null; 

user="$(whoami)@$(hostname)";

-qq git add ./*.celeste;

-qq git commit -m "⚠ $user old staged" --allow-empty-message || true;

-qq git commit . -m "⚠ $user old unstaged" --allow-empty-message || true;

-qq git fetch;

-qq git pull --ff-only || (
    echo "⚠ Sync conflict. Archiving remote data and replacing with local." &&
    git pull -s ours --no-edit);

-qq git push;

echo "✅ Synced"

echo "🍓 Celeste";

test "OFF" = "${CELESTE:-ON}" ||\
-qq open -W ~/Library/"Application Support"/itch/apps/celeste/Celeste.app || \
-qq open -W /Applications/Celeste.app || \
/mnt/d/Program\ Files/Celeste/Celeste.exe;

set -v;
cargo run build;
rm -f 0.celeste.txt; target/debug/celeste-saves 0.celeste 1> 0.celeste.txt 2> /dev/null || rm -f 0.celeste.txt;
rm -f 1.celeste.txt; target/debug/celeste-saves 1.celeste 1> 1.celeste.txt 2> /dev/null || rm -f 1.celeste.txt;
rm -f 2.celeste.txt; target/debug/celeste-saves 2.celeste 1> 2.celeste.txt 2> /dev/null || rm -f 2.celeste.txt;
target/debug/celeste-saves 0.celeste;

-qq git add .;

if -q git commit . -m "🍓 $user" --allow-empty-message; then
    -q git push && echo "✅ Synced" || echo "⚠ Sync failed";
else
    echo "🆗 No changes to sync"
fi
