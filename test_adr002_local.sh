#!/usr/bin/env bash
#
# ADR-002 Resource Server Local Test
# Validates Bearer token authentication without OAuth client code
#

# Don't use set -e since we want to continue even if tests fail

PORT=3010
BASE_URL="http://localhost:${PORT}"

echo "🧪 Testing ADR-002 Resource Server Implementation"
echo "=================================================="
echo ""

# Build
echo "📦 Building http-server-adr002..."
cargo build --bin http-server-adr002 2>&1 | grep -E "(Compiling|Finished)" || true
echo ""

# Start server
echo "🚀 Starting server on port ${PORT}..."
PORT=${PORT} RUST_LOG=warn target/debug/http-server-adr002 &
SERVER_PID=$!

# Cleanup on exit
trap "echo ''; echo '🛑 Stopping server...'; kill $SERVER_PID 2>/dev/null; exit" EXIT INT TERM

# Wait for server
sleep 2

echo "✅ Server started (PID: $SERVER_PID)"
echo ""

# Run tests
PASS=0
FAIL=0

test_endpoint() {
    local name="$1"
    local expected="$2"
    local url="$3"
    shift 3
    local curl_args=("$@")

    echo -n "  Testing $name... "
    HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "${curl_args[@]}" "$url" 2>/dev/null || echo "000")

    if [ "$HTTP_CODE" = "$expected" ]; then
        echo "✅ ($HTTP_CODE)"
        ((PASS++))
    else
        echo "❌ (expected $expected, got $HTTP_CODE)"
        ((FAIL++))
    fi
}

echo "📋 Running tests:"
echo ""

test_endpoint "health check (public)" "200" "${BASE_URL}/health"
test_endpoint "OAuth metadata (AUTH6)" "200" "${BASE_URL}/.well-known/oauth-protected-resource"
test_endpoint "list_boards without auth" "401" "${BASE_URL}/mcp/list_boards" -X POST
test_endpoint "list_boards with invalid token" "401" "${BASE_URL}/mcp/list_boards" -X POST -H "Authorization: Bearer invalid_token"
test_endpoint "get_board without auth" "401" "${BASE_URL}/mcp/get_board/test123" -X POST
test_endpoint "get_board with invalid token" "401" "${BASE_URL}/mcp/get_board/test123" -X POST -H "Authorization: Bearer fake"

echo ""
echo "=================================================="
echo "📊 Test Results: ${PASS} passed, ${FAIL} failed"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "✅ All tests passed! ADR-002 Resource Server working correctly."
    echo ""
    echo "📝 Next steps:"
    echo "   • Get real Miro access token from Claude OAuth flow"
    echo "   • Test with real token: curl -H 'Authorization: Bearer \$TOKEN' ${BASE_URL}/mcp/list_boards"
    echo "   • Deploy to Scaleway Containers for production testing"
    exit 0
else
    echo "❌ Some tests failed. Check server logs above."
    exit 1
fi
