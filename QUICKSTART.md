# Quick Start - Test Local Miro MCP

Guide rapide pour démarrer et tester le serveur MCP Miro localement en 3 étapes.

## Étape 1: Configuration

```bash
# Copier l'exemple de configuration
cp .env.example .env

# Générer une clé de chiffrement
openssl rand -hex 32

# Éditer .env avec vos credentials Miro
# Remplissez:
#   - MIRO_CLIENT_ID (depuis Miro Developer Portal)
#   - MIRO_CLIENT_SECRET (depuis Miro Developer Portal)
#   - MIRO_ENCRYPTION_KEY (résultat de openssl ci-dessus)
```

### Obtenir les credentials Miro

1. Aller sur [Miro Developer Portal](https://miro.com/app/settings/user-profile/apps)
2. Sélectionner votre application existante
3. Vérifier que `http://localhost:3000/oauth/callback` est dans les redirect URIs
4. Copier Client ID et Client Secret

## Étape 2: Test du serveur

```bash
# Test automatique complet
./test_local.sh

# Ou démarrage manuel
cargo run
```

Le script `test_local.sh` va:
- ✅ Vérifier que .env est configuré
- ✅ Compiler le projet
- ✅ Démarrer le serveur
- ✅ Tester le health check
- 📍 Afficher les endpoints disponibles

## Étape 3: Test OAuth2 dans le navigateur

Une fois le serveur démarré:

```bash
# Ouvrir dans votre navigateur
open http://localhost:3000/oauth/authorize
```

Vous devriez:
1. Être redirigé vers Miro pour autoriser l'application
2. Autoriser l'accès
3. Être redirigé vers une page de succès
4. Voir dans les logs serveur: "OAuth tokens saved successfully"

## Configuration Claude Desktop (Optionnel)

Pour utiliser le serveur avec Claude Desktop:

```bash
# Installation automatique
./install_claude_desktop.sh

# Puis redémarrer Claude Desktop
```

Voir [CLAUDE_DESKTOP_SETUP.md](CLAUDE_DESKTOP_SETUP.md) pour plus de détails.

## Scripts disponibles

| Script | Description |
|--------|-------------|
| `test_local.sh` | Test complet du serveur local |
| `install_claude_desktop.sh` | Installation de la config Claude Desktop |

## Vérification rapide

```bash
# Health check
curl http://localhost:3000/health
# Attendu: OK

# Liste des endpoints
curl -i http://localhost:3000/oauth/authorize
# Attendu: redirect 302 vers Miro
```

## Troubleshooting rapide

### Port 3000 déjà utilisé

```bash
# Trouver le process
lsof -i :3000

# Ou changer le port dans .env
echo "MCP_SERVER_PORT=3001" >> .env
```

### Erreur "OAuth state cookie not found"

- Vérifiez que vous utilisez le même navigateur
- Testez en navigation privée
- Vérifiez que les cookies sont activés

### Compilation échoue

```bash
# Nettoyer et recompiler
cargo clean
cargo build
```

## Documentation complète

- [TESTING.md](TESTING.md) - Guide de test complet avec tous les détails
- [CLAUDE_DESKTOP_SETUP.md](CLAUDE_DESKTOP_SETUP.md) - Configuration Claude Desktop
- [.env.example](.env.example) - Exemple de configuration

## Prochaines étapes

Après avoir validé le test local:
1. Implémenter AUTH5 (access token cookies)
2. Déployer sur Scaleway Functions (DEPLOY2)
3. Configurer les secrets (SEC1)

Voir [planning/backlog.md](planning/backlog.md) pour la roadmap complète.
