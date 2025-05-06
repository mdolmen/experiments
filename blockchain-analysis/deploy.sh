#!/bin/bash

# CONFIGURATION
PROJECT_ID="bubblemaps-test3e4f"
IMAGE_NAME="blockchain-analysis"
REGION="us-central1"
REPO="gcr.io/$PROJECT_ID/$IMAGE_NAME"

# AUTHENTICATE & SETUP
echo "[+] Setting gcloud project..."
gcloud config set project $PROJECT_ID

# BUILD AND PUSH DOCKER IMAGE
echo "[+] Building Docker image for linux/amd64 and pushing to $REPO..."
docker buildx build --platform linux/amd64 -t $REPO . --push

# DEPLOY TO CLOUD RUN
echo "Deploying to Cloud Run..."
gcloud run deploy $IMAGE_NAME \
  --image $REPO \
  --platform managed \
  --region $REGION \
  --allow-unauthenticated

echo "[+] Deployment complete."
