#!/usr/bin/env bash
curl -v https://flyagileapipx8njvei-miro-mcp.functions.fnc.fr-par.scw.cloud/mcp \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize"}' 2>&1
