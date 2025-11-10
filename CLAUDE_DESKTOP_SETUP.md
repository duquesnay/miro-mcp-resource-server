# Claude Desktop - Configuration Locale

Guide pour tester le serveur MCP Miro localement avec Claude Desktop.

## Prérequis

- Claude Desktop installé
- Fichier `.env` configuré avec vos credentials Miro (voir TESTING.md)
- Rust toolchain installé

## Installation

### 1. Localiser le fichier de configuration Claude Desktop

**macOS:**
```bash
open ~/Library/Application\ Support/Claude/
```

Le fichier de configuration est: `claude_desktop_config.json`

### 2. Ajouter la configuration MCP locale

Ouvrez `~/Library/Application Support/Claude/claude_desktop_config.json` et ajoutez la configuration du serveur Miro:

```json
{
  "mcpServers": {
    "miro-local": {
      "command": "cargo",
      "args": [
        "run",
        "--manifest-path",
        "/Users/guillaume/dev/experiments/miro-mcp-server/Cargo.toml"
      ],
      "env": {
        "RUST_LOG": "miro_mcp_server=debug"
      }
    }
  }
}
```

**Note**: Si vous avez déjà d'autres serveurs MCP configurés, ajoutez `"miro-local"` à l'objet `mcpServers` existant.

### 3. Vérifier la configuration

Le fichier `claude_desktop_config.json` de ce projet contient un exemple complet. Vous pouvez:

```bash
# Voir l'exemple
cat claude_desktop_config.json

# Ou fusionner avec votre config existante
# (attention à ne pas écraser vos autres serveurs MCP!)
```

### 4. Redémarrer Claude Desktop

Fermez complètement Claude Desktop et relancez-le pour charger la nouvelle configuration.

## Vérification

### 1. Vérifier que le serveur MCP est détecté

Dans Claude Desktop, ouvrez une nouvelle conversation et vérifiez:
- Le serveur "miro-local" apparaît dans la liste des serveurs MCP disponibles
- Les logs de démarrage dans la console (si accessible)

### 2. Tester l'authentification OAuth2

Dans Claude Desktop, demandez:
```
Peux-tu m'authentifier avec Miro ?
```

Claude devrait:
1. Détecter qu'aucun token n'est disponible
2. Vous fournir un lien vers: `http://localhost:3000/oauth/authorize`
3. Ouvrir votre navigateur pour l'authentification Miro

### 3. Suivre le flow OAuth2

Dans le navigateur:
1. Cliquez sur le lien fourni par Claude
2. Autorisez l'application Miro
3. Vous êtes redirigé vers: `http://localhost:3000/oauth/callback?code=...&state=...`
4. Page de succès affichée: "Authorization Successful!"

### 4. Retourner à Claude Desktop

Après l'authentification réussie:
```
Peux-tu lister mes boards Miro ?
```

Claude devrait maintenant pouvoir appeler l'API Miro avec le token stocké.

## Troubleshooting

### "Server failed to start" ou "Connection refused"

**Cause**: Le serveur MCP ne démarre pas

**Solutions**:
```bash
# Vérifier que le .env est configuré
cat .env | grep MIRO_

# Tester le démarrage manuellement
cargo run

# Vérifier les logs d'erreur
RUST_LOG=debug cargo run
```

### "OAuth state cookie not found"

**Cause**: Les cookies ne sont pas envoyés/reçus correctement

**Solutions**:
- Vérifiez que vous utilisez le même navigateur pour toute la session OAuth
- Vérifiez que les cookies sont activés
- Testez dans une fenêtre de navigation privée (cookies de session uniquement)

### "Configuration invalid" dans Claude Desktop

**Cause**: JSON mal formaté ou chemin incorrect

**Solutions**:
```bash
# Valider le JSON
cat ~/Library/Application\ Support/Claude/claude_desktop_config.json | python -m json.tool

# Vérifier le chemin du projet
ls -la /Users/guillaume/dev/experiments/miro-mcp-server/Cargo.toml
```

### Le serveur démarre mais Claude ne voit pas les outils

**Cause**: MCP protocol communication issue

**Solutions**:
1. Vérifiez les logs stderr du serveur (Claude Desktop console)
2. Testez avec `cargo run` dans un terminal séparé
3. Vérifiez que stdout est propre (pas de logs, seulement JSON MCP)

### Port 3000 déjà utilisé

**Cause**: Autre application sur le port 3000

**Solutions**:
```bash
# Trouver ce qui utilise le port
lsof -i :3000

# Changer le port dans .env
echo "MCP_SERVER_PORT=3001" >> .env

# Mettre à jour le redirect URI dans Miro Developer Portal
# http://localhost:3001/oauth/callback
```

## Développement

### Hot reload

Pour tester rapidement les changements:

```bash
# Terminal 1: Watch mode
cargo watch -x run

# Terminal 2: Test via Claude Desktop
# (redémarrer Claude Desktop après chaque changement)
```

### Debugging

```bash
# Logs détaillés
RUST_LOG=debug cargo run

# Logs avec backtrace
RUST_BACKTRACE=1 cargo run

# Logs MCP protocol
RUST_LOG=miro_mcp_server=trace,rmcp=debug cargo run
```

### Tester sans Claude Desktop

```bash
# Serveur MCP standalone
cargo run

# Dans un autre terminal, tester avec curl
curl http://localhost:3000/health
curl http://localhost:3000/oauth/authorize
```

## Désinstallation

Pour retirer la configuration locale de Claude Desktop:

```bash
# Éditer le fichier de config
open ~/Library/Application\ Support/Claude/claude_desktop_config.json

# Supprimer la section "miro-local" de "mcpServers"
# Sauvegarder et redémarrer Claude Desktop
```

## Production

Une fois les tests locaux validés, suivez [TESTING.md](TESTING.md) pour le déploiement sur Scaleway Functions.

La configuration Claude Desktop pour la production utilisera:
```json
{
  "mcpServers": {
    "miro-production": {
      "command": "curl",
      "args": [
        "-X", "POST",
        "https://your-function.functions.scw.cloud/mcp"
      ]
    }
  }
}
```

(Configuration exacte à définir après DEPLOY2)

## Références

- [TESTING.md](TESTING.md) - Guide de test local complet
- [Claude Desktop MCP Documentation](https://docs.anthropic.com/claude/docs/mcp)
- [Miro Developer Portal](https://miro.com/app/settings/user-profile/apps)
