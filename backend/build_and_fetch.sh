#!/bin/bash
REPO="Tenuka22/Skoola"        # change this
BINARY_NAME="backend"   # change this

echo "🚀 Triggering build..."
gh workflow run build.yml --repo $REPO

echo "⏳ Waiting for build to complete..."
sleep 10

while true; do
  STATUS=$(gh run list --repo $REPO --workflow=build.yml --limit 1 --json status,conclusion -q '.[0].status')
  CONCLUSION=$(gh run list --repo $REPO --workflow=build.yml --limit 1 --json status,conclusion -q '.[0].conclusion')

  echo "  Status: $STATUS"

  if [ "$STATUS" = "completed" ]; then
    if [ "$CONCLUSION" = "success" ]; then
      echo "✅ Build succeeded!"
      break
    else
      echo "❌ Build failed!"
      exit 1
    fi
  fi

  sleep 15
done

echo "📦 Downloading artifact..."
gh run download --repo $REPO --name my-binary

chmod +x "$BINARY_NAME"
echo "🎉 Done! ./$BINARY_NAME"
