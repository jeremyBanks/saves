#!/bin/bash -euo pipefail

function celeste {
    cd ~/Library/"Application Support"/Celeste/Saves;
    
    git commit -m "" --allow-empty-message &> /dev/null &&
        echo "Committed old partially saved changes.";
    
    git commit . -m "" --allow-empty-message &> /dev/null &&
        echo "Committed old unsaved changes.";

    git pull --ff-only || (
      echo "Sync conflict. Archiving local saves and replacing with remote saves." && git pull -s theirs);

    git push 2&> /dev/null;

    open -W ~/Library/"Application Support"/itch/apps/celeste/Celeste.app;
    
    git commit . -m "" --allow-empty-message &> /dev/null &&
        echo "Committed new changes." || echo "No changes to commit.";

    git push;

    cd - 2&> /dev/null;
}

celeste;
