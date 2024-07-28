# Oracle Database REST Service with Actix

## Objectif

Ce projet a pour objectif de fournir un service REST pour accéder à une base de données Oracle, en utilisant le framework Actix pour le serveur web. Il est conçu pour être une démonstration éducative de la création d'une API REST avec gestion des CORS (Cross-Origin Resource Sharing).

## Technologies Utilisées

- **Rust** : Langage de programmation utilisé pour développer le service.
- **Actix** : Framework web en Rust pour construire des serveurs HTTP performants.
- **Oracle** : Base de données relationnelle utilisée pour stocker les données.
- **dotenv** : Pour gérer les variables d'environnement.
- **serde** : Pour la sérialisation et la désérialisation des données.
- **validator** : Pour valider les données des requêtes.

## Fonctionnalités

- **Connexion à une base de données Oracle** : Connexion et opérations CRUD (Create, Read, Update, Delete) sur les données des clients.
- **API REST** : Fournit des endpoints REST pour interagir avec la base de données.
- **Gestion des CORS** : Configuration des CORS pour permettre l'accès depuis différents domaines.

## Configuration

### Prérequis

- **Rust** : Assurez-vous d'avoir Rust installé. Vous pouvez l'installer depuis [rust-lang.org](https://www.rust-lang.org/).
- **Oracle Database** : Assurez-vous d'avoir accès à une instance Oracle Database.

### Variables d'environnement

Créez un fichier `.env` à la racine du projet pour configurer les variables d'environnement nécessaires :

```env
ORACLE_USER=<votre_utilisateur_oracle>
ORACLE_PASSWORD=<votre_mot_de_passe_oracle>
ORACLE_HOST=<hôte_oracle>
ORACLE_SERVICE=<service_oracle>
