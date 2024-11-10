# Git Manager

Un outil en ligne de commande pour gérer facilement la creation des différents étapes dans l'organisation de vos projet sur Github & Gitalb. Il permet de créer et maintenir un ensemble cohérent de labels à travers différents projets.

## Fonctionnalités

- Création de labels à partir d'un fichier de configuration JSON
- Support des variables d'environnement pour la configuration
- Gestion des couleurs et descriptions pour chaque label
- Support des projets Github & GitLab

## Prérequis

- Rust 1.70 ou supérieur
- Un token d'accès GitLab avec les permissions appropriées
- Un fichier de configuration des labels au format JSON

## Installation

```bash
# Cloner le répertoire
git clone https://github.com/votre-username/gitlab-label-manager.git
cd gitlab-label-manager

# Compiler le projet
cargo build --release

# Optionnel : installer globalement
cargo install --path .
```

## Configuration

1. Créez un fichier `.env` à la racine du projet :

```env
GITLAB_API_URL=https://gitlab.com/api/v4
GITLAB_API_TOKEN=votre-token-gitlab
GITLAB_PROJECT_ID=votre-username/votre-project
```

2. Créez un fichier `labels.json` pour définir vos labels :

```json
{
  "labels": [
    {
      "name": "priority::1",
      "color": "#FF0000",
      "description": "Highest priority"
    },
    {
      "name": "type::feature",
      "color": "#428BCA",
      "description": "New feature"
    }
  ]
}
```

## Utilisation

```bash
# Utilisation avec les variables d'environnement
gitlab_label_manager

# Utilisation avec des arguments en ligne de commande
gitlab_label_manager \
  --api-url "https://gitlab.com/api/v4" \
  --token "votre-token" \
  --project-id "username/project" \
  --config "labels.json"
```

## Structure des Labels

Les labels suivent une convention de nommage spécifique :
- `priority::1`, `priority::2`, etc. pour les priorités
- `type::feature`, `type::bug`, etc. pour les types
- `status::todo`, `status::in-progress`, etc. pour les statuts

## Arguments en Ligne de Commande

| Argument | Description | Env Variable | Obligatoire |
|----------|-------------|--------------|-------------|
| --api-url | URL de l'API GitLab | GITLAB_API_URL | Oui |
| --token | Token d'accès GitLab | GITLAB_API_TOKEN | Oui |
| --project-id | ID ou chemin du projet | GITLAB_PROJECT_ID | Oui |
| --config | Chemin du fichier de config | - | Non (défaut: labels.json) |

## Développement

```bash
# Exécuter les tests
cargo test

# Vérifier le formatage
cargo fmt

# Vérifier avec clippy
cargo clippy
```

## TODO

- [ ] Ajouter la mise à jour des labels existants
- [ ] Ajouter la suppression des labels non utilisés
- [ ] Ajouter la synchronisation entre projets
- [ ] Ajouter la validation des couleurs
- [ ] Ajouter des tests

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

## Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.
