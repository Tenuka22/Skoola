#!/bin/bash
set -e

# Repository name
REPO="Tenuka22/Skoola"
# Workflow filename or name
WORKFLOW="build.yml"
# Expected artifact name and binary name
BINARY_NAME="backend"

echo "🔎 Checking latest commit on remote..."

# Get the latest commit SHA from the remote repository
LATEST_COMMIT=$(gh api "repos/$REPO/commits" --jq '.[0].sha')
echo "  Latest commit: $LATEST_COMMIT"

# Search for the most recent run for this commit
echo "🔎 Searching for build runs for $LATEST_COMMIT..."
RUN_INFO=$(gh run list \
  --repo "$REPO" \
  --workflow="$WORKFLOW" \
  --commit "$LATEST_COMMIT" \
  --limit 1 \
  --json databaseId,conclusion,status \
  --jq '.[0]')

if [[ -z "$RUN_INFO" || "$RUN_INFO" == "null" ]]; then
  echo "🚀 No build found for this commit. Triggering build..."
  gh workflow run "$WORKFLOW" --repo "$REPO"
  
  echo "⏳ Waiting for workflow to register..."
  # Wait for the run to appear in the list
  sleep 5
  
  # Get the NEWEST run ID (the one we just triggered)
  LAST_RUN_ID=$(gh run list \
    --repo "$REPO" \
    --workflow="$WORKFLOW" \
    --limit 1 \
    --json databaseId \
    --jq '.[0].databaseId')
  
  echo "👀 Watching run $LAST_RUN_ID..."
  gh run watch "$LAST_RUN_ID" --repo "$REPO" --exit-status
  echo "✅ Build completed!"
else
  LAST_RUN_ID=$(echo "$RUN_INFO" | jq -r '.databaseId')
  LAST_RUN_CONCLUSION=$(echo "$RUN_INFO" | jq -r '.conclusion')
  LAST_RUN_STATUS=$(echo "$RUN_INFO" | jq -r '.status')
  
  echo "  Found run $LAST_RUN_ID (Status: $LAST_RUN_STATUS, Conclusion: $LAST_RUN_CONCLUSION)"
  
  if [[ "$LAST_RUN_CONCLUSION" == "success" ]]; then
    echo "✅ Latest commit already built successfully."
  elif [[ "$LAST_RUN_STATUS" == "in_progress" || "$LAST_RUN_STATUS" == "queued" ]]; then
    echo "👀 Build already in progress for this commit. Watching..."
    gh run watch "$LAST_RUN_ID" --repo "$REPO" --exit-status
    echo "✅ Build completed!"
  else
    echo "🚀 Last build for this commit was $LAST_RUN_CONCLUSION. Re-triggering build..."
    gh workflow run "$WORKFLOW" --repo "$REPO"
    
    echo "⏳ Waiting for workflow to register..."
    sleep 5
    
    LAST_RUN_ID=$(gh run list \
      --repo "$REPO" \
      --workflow="$WORKFLOW" \
      --limit 1 \
      --json databaseId \
      --jq '.[0].databaseId')
    
    echo "👀 Watching run $LAST_RUN_ID..."
    gh run watch "$LAST_RUN_ID" --repo "$REPO" --exit-status
    echo "✅ Build completed!"
  fi
fi

echo "📦 Downloading artifact from run $LAST_RUN_ID..."
# Clean up old binary/directory to avoid confusion
rm -f "$BINARY_NAME"
rm -rf "$BINARY_NAME/"
rm -rf "my-binary/"

# Try the new artifact name first
if ! gh run download "$LAST_RUN_ID" --repo "$REPO" --name "$BINARY_NAME" --dir . 2>/dev/null; then
  echo "⚠️ Artifact '$BINARY_NAME' not found. Trying fallback 'my-binary'..."
  if ! gh run download "$LAST_RUN_ID" --repo "$REPO" --name "my-binary" --dir . 2>/dev/null; then
    echo "❌ Neither '$BINARY_NAME' nor 'my-binary' artifacts were found!"
    ls -la
    exit 1
  fi
  # If downloaded as 'my-binary', move it
  if [[ -d "my-binary" && -f "my-binary/$BINARY_NAME" ]]; then
    mv "my-binary/$BINARY_NAME" .
    rmdir "my-binary"
  fi
fi

# If the artifact was downloaded into a directory named 'backend'
if [[ -d "$BINARY_NAME" && -f "$BINARY_NAME/$BINARY_NAME" ]]; then
  mv "$BINARY_NAME/$BINARY_NAME" .
  rmdir "$BINARY_NAME"
fi

if [[ ! -f "$BINARY_NAME" ]]; then
  echo "❌ Artifact '$BINARY_NAME' not found after download!"
  # List files to see what we got
  ls -la
  exit 1
fi

chmod +x "$BINARY_NAME"
echo "🎉 Done! ./$BINARY_NAME"
