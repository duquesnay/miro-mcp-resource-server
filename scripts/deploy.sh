#!/usr/bin/env bash
#
# Scaleway Container deployment script for Miro MCP Server
#
# Usage:
#   ./scripts/deploy.sh --project=miro-mcp-server --region=fr-par --registry=rg.fr-par.scw.cloud/miro-mcp
#
# Prerequisites:
#   - Docker installed and running
#   - Scaleway CLI installed (`scw` command available)
#   - Scaleway credentials configured (`scw init`)
#   - Scaleway Container Registry created
#   - Scaleway secrets configured (MIRO_CLIENT_ID, MIRO_CLIENT_SECRET, etc.)
#

set -euo pipefail

# Default values
PROJECT_NAME="miro-mcp-server"
REGION="fr-par"
REGISTRY=""
NAMESPACE=""
TAG="latest"
DRY_RUN=false

# Colors for output
RED='\033[0.31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --project=*)
      PROJECT_NAME="${1#*=}"
      shift
      ;;
    --region=*)
      REGION="${1#*=}"
      shift
      ;;
    --registry=*)
      REGISTRY="${1#*=}"
      shift
      ;;
    --namespace=*)
      NAMESPACE="${1#*=}"
      shift
      ;;
    --tag=*)
      TAG="${1#*=}"
      shift
      ;;
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Validate required parameters
if [[ -z "$REGISTRY" ]]; then
  echo -e "${RED}Error: --registry is required${NC}"
  echo "Example: --registry=rg.fr-par.scw.cloud/miro-mcp"
  exit 1
fi

# Extract namespace from registry if not provided
if [[ -z "$NAMESPACE" ]]; then
  NAMESPACE=$(basename "$REGISTRY")
fi

# Build full image name
IMAGE_NAME="$REGISTRY/$PROJECT_NAME:$TAG"

echo -e "${GREEN}=== Miro MCP Server Deployment ===${NC}"
echo "Project: $PROJECT_NAME"
echo "Region: $REGION"
echo "Registry: $REGISTRY"
echo "Namespace: $NAMESPACE"
echo "Image: $IMAGE_NAME"
echo "Dry run: $DRY_RUN"
echo ""

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v docker &> /dev/null; then
  echo -e "${RED}Error: Docker is not installed${NC}"
  exit 1
fi

if ! command -v scw &> /dev/null; then
  echo -e "${RED}Error: Scaleway CLI is not installed${NC}"
  echo "Install: https://www.scaleway.com/en/docs/developer-tools/scaleway-cli/"
  exit 1
fi

if ! docker info &> /dev/null; then
  echo -e "${RED}Error: Docker is not running${NC}"
  exit 1
fi

echo -e "${GREEN}✓ Prerequisites OK${NC}"
echo ""

# Build Docker image
echo -e "${YELLOW}Building Docker image...${NC}"
if [[ "$DRY_RUN" == "true" ]]; then
  echo "[DRY RUN] Would run: docker build -t $IMAGE_NAME ."
else
  docker build -t "$IMAGE_NAME" .
  echo -e "${GREEN}✓ Docker build complete${NC}"
fi
echo ""

# Push to Scaleway Container Registry
echo -e "${YELLOW}Pushing image to Scaleway Container Registry...${NC}"
if [[ "$DRY_RUN" == "true" ]]; then
  echo "[DRY RUN] Would run: docker push $IMAGE_NAME"
else
  docker push "$IMAGE_NAME"
  echo -e "${GREEN}✓ Image pushed${NC}"
fi
echo ""

# Deploy to Scaleway Container
echo -e "${YELLOW}Deploying to Scaleway Container...${NC}"
if [[ "$DRY_RUN" == "true" ]]; then
  echo "[DRY RUN] Would run: scw container deploy ..."
  echo "  --region=$REGION"
  echo "  --namespace-id=\$(scw container namespace list region=$REGION name=$NAMESPACE -o json | jq -r '.[0].id')"
  echo "  --registry-image=$IMAGE_NAME"
  echo "  --name=$PROJECT_NAME"
else
  # Get namespace ID
  NAMESPACE_ID=$(scw container namespace list region="$REGION" name="$NAMESPACE" -o json | jq -r '.[0].id')

  if [[ -z "$NAMESPACE_ID" || "$NAMESPACE_ID" == "null" ]]; then
    echo -e "${RED}Error: Namespace '$NAMESPACE' not found in region '$REGION'${NC}"
    echo "Create namespace first: scw container namespace create name=$NAMESPACE region=$REGION"
    exit 1
  fi

  echo "Namespace ID: $NAMESPACE_ID"

  # Check if container already exists
  EXISTING_CONTAINER=$(scw container container list region="$REGION" namespace-id="$NAMESPACE_ID" name="$PROJECT_NAME" -o json | jq -r '.[0].id // empty')

  if [[ -n "$EXISTING_CONTAINER" ]]; then
    echo "Updating existing container: $EXISTING_CONTAINER"
    scw container container update \
      region="$REGION" \
      container-id="$EXISTING_CONTAINER" \
      registry-image="$IMAGE_NAME"
  else
    echo "Creating new container"
    scw container container create \
      region="$REGION" \
      namespace-id="$NAMESPACE_ID" \
      name="$PROJECT_NAME" \
      registry-image="$IMAGE_NAME" \
      min-scale=1 \
      max-scale=1 \
      memory-limit=256 \
      cpu-limit=250
  fi

  echo -e "${GREEN}✓ Container deployed${NC}"
fi
echo ""

# Health check (if container has HTTP endpoint)
if [[ "$DRY_RUN" == "false" ]]; then
  echo -e "${YELLOW}Deployment complete!${NC}"
  echo ""
  echo "Next steps:"
  echo "1. Configure Scaleway secrets:"
  echo "   - MIRO_CLIENT_ID"
  echo "   - MIRO_CLIENT_SECRET"
  echo "   - MIRO_REDIRECT_URI"
  echo "   - MIRO_ENCRYPTION_KEY"
  echo "   - TOKEN_STORAGE_PATH=/app/data/tokens.enc"
  echo ""
  echo "2. Update Miro Developer Portal redirect URI:"
  echo "   - Get container URL: scw container container list region=$REGION namespace-id=$NAMESPACE_ID name=$PROJECT_NAME"
  echo "   - Register redirect URI: https://<container-url>/oauth/callback"
  echo ""
  echo "3. Test deployment:"
  echo "   scw container container logs region=$REGION container-id=<container-id>"
else
  echo -e "${YELLOW}[DRY RUN] Deployment simulated successfully${NC}"
fi
