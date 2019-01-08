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

-qq git add .;
-qq git commit . -m "⚠ $user old unstaged" --allow-empty-message || true;

-qq git fetch;

-qq git pull --ff-only || (
    echo "⚠ Sync conflict. Archiving remote data and replacing with local." &&
    git pull -s ours --no-edit);

-qq git push;

echo "✅ Synced"

echo "🍓 Celeste";

test "ON" != "${CELESTE:-ON}" ||\
-qq open -W ~/Library/"Application Support"/itch/apps/celeste/Celeste.app || \
-qq open -W /Applications/Celeste.app || \
/mnt/d/Program\ Files/Celeste/Celeste.exe;

cargo build 2> /dev/null || cargo build;
-qq yarn install;

cat template.html > index.html;
for n in 0 1 2; do 
    rm -f ${n}.txt || true; target/debug/celeste-saves ${n}.celeste 1> ${n}.txt 2> /dev/null || rm -f ${n}.txt;
    rm -f ${n}.html || true; (cat template.html; (CELESTE_SAVE_COLOR=ON target/debug/celeste-saves ${n}.celeste | tee --append index.html) | node_modules/.bin/ansi-to-html) 1> ${n}.html 2> /dev/null || rm -f ${n}.html;
done

-qq git add .;
if -qq git commit . -m "🍓 $user" --allow-empty-message; then
    -qq git push && echo "✅ Synced" || echo "⚠ Sync failed";
    set +e;
    diff="$(git diff -U5 --ws-error-highlight=none HEAD~1..HEAD 0.txt | tail -n +6 | egrep '^[\+\-]' -B 5 | grep -v '@@')";
    set -e;
    echo "$diff";
    -qq yarn run send "$diff" || true;
else
    echo "🆗 No changes to sync"
fi

echo;
target/debug/celeste-saves 0.celeste;
