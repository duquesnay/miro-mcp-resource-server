#!/bin/bash
# Script de test local du serveur MCP Miro
# Usage: ./test_local.sh

set -e

echo "🔍 Vérification de l'environnement..."

# Vérifier que .env existe
if [ ! -f .env ]; then
    echo "❌ Fichier .env manquant"
    echo "📝 Copiez .env.example vers .env et configurez vos credentials Miro"
    exit 1
fi

# Vérifier les variables requises
required_vars=("MIRO_CLIENT_ID" "MIRO_CLIENT_SECRET" "MIRO_REDIRECT_URI" "MIRO_ENCRYPTION_KEY")
missing_vars=()

for var in "${required_vars[@]}"; do
    if ! grep -q "^${var}=" .env; then
        missing_vars+=("$var")
    fi
done

if [ ${#missing_vars[@]} -gt 0 ]; then
    echo "❌ Variables manquantes dans .env:"
    printf '   - %s\n' "${missing_vars[@]}"
    exit 1
fi

echo "✅ Fichier .env configuré"

# Vérifier que le port est disponible
PORT=$(grep "^MCP_SERVER_PORT=" .env | cut -d'=' -f2 || echo "3000")
if lsof -Pi :$PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo "⚠️  Port $PORT déjà utilisé"
    echo "   Process: $(lsof -Pi :$PORT -sTCP:LISTEN | tail -n1)"
    echo ""
    echo "Options:"
    echo "  1. Arrêter le process existant"
    echo "  2. Changer MCP_SERVER_PORT dans .env"
    exit 1
fi

echo "✅ Port $PORT disponible"
echo ""

# Build du projet
echo "🔨 Compilation du projet..."
if ! cargo build 2>&1 | tail -5; then
    echo "❌ Erreur de compilation"
    exit 1
fi

echo "✅ Compilation réussie"
echo ""

# Démarrer le serveur en arrière-plan
echo "🚀 Démarrage du serveur MCP..."
cargo run &
SERVER_PID=$!

# Fonction pour arrêter le serveur proprement
cleanup() {
    echo ""
    echo "🛑 Arrêt du serveur..."
    kill $SERVER_PID 2>/dev/null || true
    wait $SERVER_PID 2>/dev/null || true
}
trap cleanup EXIT INT TERM

# Attendre que le serveur démarre
echo "⏳ Attente du démarrage (5 secondes)..."
sleep 5

# Tester le health check
echo ""
echo "🔍 Test du health check..."
if curl -s -f http://localhost:$PORT/health > /dev/null; then
    echo "✅ Health check OK"
else
    echo "❌ Health check échoué"
    echo "   Le serveur n'a peut-être pas démarré correctement"
    echo "   Vérifiez les logs ci-dessus"
    exit 1
fi

# Afficher les endpoints disponibles
echo ""
echo "✅ Serveur MCP démarré avec succès!"
echo ""
echo "📍 Endpoints disponibles:"
echo "   - Health check:     http://localhost:$PORT/health"
echo "   - OAuth authorize:  http://localhost:$PORT/oauth/authorize"
echo "   - OAuth callback:   http://localhost:$PORT/oauth/callback"
echo ""
echo "🔐 Pour tester le flow OAuth2:"
echo "   1. Ouvrez: http://localhost:$PORT/oauth/authorize"
echo "   2. Autorisez l'application Miro dans votre navigateur"
echo "   3. Vérifiez la page de succès après redirection"
echo ""
echo "🖥️  Pour utiliser avec Claude Desktop:"
echo "   Consultez: CLAUDE_DESKTOP_SETUP.md"
echo ""
echo "📋 Le serveur continue de tourner en arrière-plan"
echo "   Appuyez sur Ctrl+C pour arrêter"
echo ""

# Garder le serveur actif et afficher les logs
wait $SERVER_PID
