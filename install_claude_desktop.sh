#!/bin/bash
# Script d'installation de la configuration Claude Desktop
# Usage: ./install_claude_desktop.sh

set -e

CLAUDE_CONFIG_DIR="$HOME/Library/Application Support/Claude"
CLAUDE_CONFIG_FILE="$CLAUDE_CONFIG_DIR/claude_desktop_config.json"
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "🔍 Installation de la configuration Claude Desktop"
echo ""

# Vérifier que Claude Desktop est installé
if [ ! -d "$CLAUDE_CONFIG_DIR" ]; then
    echo "❌ Claude Desktop n'est pas installé"
    echo "   Chemin attendu: $CLAUDE_CONFIG_DIR"
    echo ""
    echo "Installez Claude Desktop depuis: https://claude.ai/download"
    exit 1
fi

echo "✅ Claude Desktop détecté"

# Créer le fichier de config s'il n'existe pas
if [ ! -f "$CLAUDE_CONFIG_FILE" ]; then
    echo "📝 Création du fichier de configuration..."
    mkdir -p "$CLAUDE_CONFIG_DIR"
    echo '{"mcpServers":{}}' > "$CLAUDE_CONFIG_FILE"
fi

# Backup de la config existante
BACKUP_FILE="$CLAUDE_CONFIG_FILE.backup.$(date +%Y%m%d_%H%M%S)"
cp "$CLAUDE_CONFIG_FILE" "$BACKUP_FILE"
echo "💾 Backup créé: $BACKUP_FILE"

# Vérifier si miro-local existe déjà
if grep -q '"miro-local"' "$CLAUDE_CONFIG_FILE"; then
    echo "⚠️  Configuration 'miro-local' existe déjà"
    echo ""
    read -p "Voulez-vous la remplacer? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Installation annulée"
        echo "   Le backup a été conservé: $BACKUP_FILE"
        exit 0
    fi

    # Supprimer l'ancienne config miro-local
    echo "🗑️  Suppression de l'ancienne configuration..."
fi

# Ajouter la nouvelle configuration avec jq si disponible
if command -v jq &> /dev/null; then
    echo "📝 Ajout de la configuration avec jq..."

    jq --arg manifest "$PROJECT_DIR/Cargo.toml" \
       '.mcpServers["miro-local"] = {
          "command": "cargo",
          "args": ["run", "--manifest-path", $manifest],
          "env": {"RUST_LOG": "miro_mcp_server=debug"}
        }' "$CLAUDE_CONFIG_FILE" > "$CLAUDE_CONFIG_FILE.tmp"

    mv "$CLAUDE_CONFIG_FILE.tmp" "$CLAUDE_CONFIG_FILE"

    echo "✅ Configuration ajoutée avec jq"
else
    echo "⚠️  jq n'est pas installé, ajout manuel..."
    echo ""
    echo "📝 Ajoutez cette configuration à $CLAUDE_CONFIG_FILE:"
    echo ""
    cat << EOF
{
  "mcpServers": {
    "miro-local": {
      "command": "cargo",
      "args": [
        "run",
        "--manifest-path",
        "$PROJECT_DIR/Cargo.toml"
      ],
      "env": {
        "RUST_LOG": "miro_mcp_server=debug"
      }
    }
  }
}
EOF
    echo ""
    echo "💡 Pour installer jq: brew install jq"
    exit 0
fi

# Valider le JSON
if ! python3 -m json.tool "$CLAUDE_CONFIG_FILE" > /dev/null 2>&1; then
    echo "❌ Configuration JSON invalide"
    echo "   Restauration du backup..."
    cp "$BACKUP_FILE" "$CLAUDE_CONFIG_FILE"
    exit 1
fi

echo "✅ Configuration JSON validée"
echo ""

# Afficher la configuration
echo "📋 Configuration installée:"
jq '.mcpServers["miro-local"]' "$CLAUDE_CONFIG_FILE"
echo ""

echo "✅ Installation terminée!"
echo ""
echo "📝 Prochaines étapes:"
echo "   1. Fermez complètement Claude Desktop"
echo "   2. Relancez Claude Desktop"
echo "   3. Vérifiez que 'miro-local' apparaît dans les serveurs MCP"
echo "   4. Testez avec: ./test_local.sh"
echo ""
echo "📖 Documentation complète: CLAUDE_DESKTOP_SETUP.md"
echo ""
echo "💾 Backup disponible: $BACKUP_FILE"
