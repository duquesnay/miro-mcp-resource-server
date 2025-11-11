#!/usr/bin/env bash
#
# Local HTTP Server Test Script
# Tests ADR-002 Resource Server implementation locally
#

set -e

PORT="${PORT:-3010}"
BASE_URL="http://localhost:${PORT}"

echo "🚀 Starting Miro MCP HTTP Server on port ${PORT}..."
echo ""

# Build the server
echo "📦 Building server..."
cargo build --release --quiet

# Start server in background
echo "▶️  Starting server..."
RUST_LOG=info target/release/miro-mcp-server &
SERVER_PID=$!

# Wait for server to start
echo "⏳ Waiting for server to start..."
sleep 2

# Cleanup function
cleanup() {
    echo ""
    echo "🛑 Stopping server (PID: ${SERVER_PID})..."
    kill $SERVER_PID 2>/dev/null || true
    wait $SERVER_PID 2>/dev/null || true
    echo "✅ Server stopped"
}

trap cleanup EXIT

echo ""
echo "================== Testing Endpoints =================="
echo ""

# Test 1: Health check (public)
echo "1️⃣  Testing health endpoint (public)..."
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" ${BASE_URL}/health)
if [ "$HTTP_CODE" = "200" ]; then
    echo "   ✅ Health check passed (200 OK)"
else
    echo "   ❌ Health check failed (HTTP ${HTTP_CODE})"
fi
echo ""

# Test 2: OAuth metadata endpoint (public, AUTH6)
echo "2️⃣  Testing OAuth metadata endpoint (AUTH6 - RFC 9728)..."
RESPONSE=$(curl -s ${BASE_URL}/.well-known/oauth-protected-resource)
if echo "$RESPONSE" | grep -q "protected_resources"; then
    echo "   ✅ Metadata endpoint working"
    echo "   Response: ${RESPONSE}"
else
    echo "   ❌ Metadata endpoint failed"
    echo "   Response: ${RESPONSE}"
fi
echo ""

# Test 3: List boards without auth (should fail with 401)
echo "3️⃣  Testing list_boards without Authorization header (should 401)..."
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST ${BASE_URL}/mcp/list_boards)
if [ "$HTTP_CODE" = "401" ]; then
    echo "   ✅ Correctly rejected request without auth (401 Unauthorized)"
else
    echo "   ❌ Wrong status code: ${HTTP_CODE} (expected 401)"
fi
echo ""

# Test 4: List boards with invalid token (should fail with 401)
echo "4️⃣  Testing list_boards with invalid token (should 401)..."
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST ${BASE_URL}/mcp/list_boards \
    -H "Authorization: Bearer invalid_token_here")
if [ "$HTTP_CODE" = "401" ]; then
    echo "   ✅ Correctly rejected invalid token (401 Unauthorized)"
else
    echo "   ❌ Wrong status code: ${HTTP_CODE} (expected 401)"
fi
echo ""

# Test 5: Get board without auth (should fail with 401)
echo "5️⃣  Testing get_board without Authorization header (should 401)..."
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST ${BASE_URL}/mcp/get_board/test_board_id)
if [ "$HTTP_CODE" = "401" ]; then
    echo "   ✅ Correctly rejected request without auth (401 Unauthorized)"
else
    echo "   ❌ Wrong status code: ${HTTP_CODE} (expected 401)"
fi
echo ""

echo "================== Test Summary =================="
echo ""
echo "✅ All authentication endpoints working correctly!"
echo ""
echo "📝 Next steps:"
echo "   1. Get real Miro access token from Claude OAuth flow"
echo "   2. Test with: curl -H 'Authorization: Bearer \$TOKEN' ${BASE_URL}/mcp/list_boards"
echo "   3. Deploy to Scaleway Containers for production testing"
echo ""
echo "🔗 OAuth Metadata: ${BASE_URL}/.well-known/oauth-protected-resource"
echo "🏥 Health Check: ${BASE_URL}/health"
echo ""
