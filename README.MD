# Actix Web API

Une API développée avec Actix Web en Rust, utilisant JWT pour l'authentification.

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) installé
- [Diesel CLI](https://diesel.rs/) installé :
  ```sh
  cargo install diesel_cli --no-default-features --features sqlite
  ```
- Base de données MySQL

## Installation

1. Clonez le dépôt :
   ```sh
   git clone https://git@github.com:fnoual/rust-cover-letter.git
   cd rust-cover-letter
   ```

2. Configurez la base de données :
   ```sh
   diesel setup
   ```

3. Copiez `.env.example` en `.env` et modifiez les valeurs :
   ```sh
   cp .env.example .env
   ```

4. Installez les dépendances :
   ```sh
   cargo build
   ```

5. Démarrez le serveur :
   ```sh
   cargo run
   ```

L'API sera disponible sur `http://127.0.0.1:8080`.

## Endpoints

### Authentification

#### 🔐 Connexion
- **POST /login**
- **Corps JSON :**
  ```json
  {
    "email": "user@example.com",
    "password": "motdepasse"
  }
  ```
- **Réponse (200) :**
  ```json
  {
    "token": "<JWT_TOKEN>"
  }
  ```

### Utilisateurs

#### 🆕 Inscription
- **POST /register**
- **Corps JSON :**
  ```json
  {
    "username": "johndoe",
    "email": "johndoe@example.com",
    "password": "securepassword"
  }
  ```
- **Réponse (201) :**
  ```json
  {
    "message": "User registered successfully"
  }
  ```

#### 👤 Récupérer son profil
- **GET /users/profile**
- **Headers :**
  ```
  Authorization: Bearer <JWT_TOKEN>
  ```
- **Réponse (200) :**
  ```json
  {
    "user_id": "123",
    "username": "johndoe"
  }
  ```

### Autres

#### 🏗️ Hello World
- **GET /**
- **Réponse :** `Hello, Rust API!`

## Configuration

Fichier `.env` :
```
OPENAI_API_KEY=
DATABASE_URL=mysql://username:password@localhost/database_name
JWT_SECRET=supersecretkey
```

## Technologies utilisées
- [Actix Web](https://actix.rs/)
- [Diesel ORM](https://diesel.rs/)
- [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/)

## Contribution

1. Forkez le projet
2. Créez une branche (`git checkout -b feature-xyz`)
3. Faites vos modifications et committez (`git commit -m 'Add feature xyz'`)
4. Poussez vos modifications (`git push origin feature-xyz`)
5. Ouvrez une Pull Request

## Licence

Ce projet est sous licence MIT.

