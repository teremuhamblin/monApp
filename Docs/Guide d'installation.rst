===============================
Guide d'installation de monApp
===============================

Ce document décrit l'installation, la compilation et la récupération de l'application
Android *monApp*, construite avec une architecture Rust + C++ + Java et compilée
automatiquement via GitHub Actions.

Ce guide est conçu pour un environnement minimal : un navigateur web et un compte GitHub.
Aucun outil local (Android Studio, SDK, NDK, cargo-ndk) n'est requis.

-----------------------
1. Pré-requis essentiels
-----------------------

Pour utiliser ou modifier le projet monApp, vous avez uniquement besoin de :

- Un navigateur web (Chrome recommandé)
- Un compte GitHub
- Le dépôt monApp (fork ou clone en ligne)
- Une connexion Internet

Aucun environnement de développement local n'est nécessaire.

-------------------------
2. Structure du projet
-------------------------

Le projet est organisé en trois blocs principaux :

- ``rust/`` : noyau Rust compilé en librairie native (cdylib)
- ``android/`` : application Android (Java + CMake)
- ``.github/workflows/`` : pipeline CI/CD pour compiler automatiquement l'APK
- ``docs/`` : documentation du projet
- ``LICENSE`` : licence publique *The Unlicense*

Arborescence simplifiée :

.. code-block:: text

   monApp/
   ├─ rust/
   │  ├─ Cargo.toml
   │  └─ src/lib.rs
   ├─ android/
   │  ├─ settings.gradle
   │  ├─ build.gradle
   │  └─ app/
   │     ├─ build.gradle
   │     └─ src/main/
   │        ├─ AndroidManifest.xml
   │        ├─ java/com/example/monapp/MainActivity.java
   │        └─ cpp/
   │           ├─ CMakeLists.txt
   │           └─ native-lib.cpp
   ├─ .github/workflows/build.yml
   ├─ docs/
   ├─ LICENSE
   ├─ .gitignore
   └─ README.md

----------------------------------------
3. Compilation automatique via GitHub CI
----------------------------------------

La compilation de l'application est entièrement automatisée grâce à GitHub Actions.
Chaque modification poussée sur la branche ``main`` déclenche :

1. Installation de Rust
2. Installation de cargo-ndk
3. Compilation du noyau Rust pour Android
4. Génération de la librairie native ``.so``
5. Compilation de l'APK Android via Gradle
6. Dépôt de l'APK dans les *Artifacts* du workflow

Aucune action manuelle n'est requise.

-----------------------------------
4. Récupération de l'APK compilé
-----------------------------------

Pour récupérer l'application Android :

1. Accéder au dépôt GitHub ``monApp``.
2. Ouvrir l'onglet **Actions**.
3. Sélectionner le workflow : **Build Android APK**.
4. Choisir la dernière exécution réussie.
5. Télécharger l'Artifact nommé :

   ``monApp-debug.apk``

Cet APK peut être installé sur n'importe quel appareil Android compatible.

-----------------------------------
5. Installation sur un appareil Android
-----------------------------------

Pour installer l'application :

1. Transférer ``monApp-debug.apk`` sur votre smartphone.
2. Activer l'option :

   ``Paramètres → Sécurité → Installer des applications inconnues``

3. Ouvrir l'APK et procéder à l'installation.
4. Lancer l'application *monApp*.
5. Le message natif provenant du noyau Rust s'affiche immédiatement.

-----------------------------------
6. Licence
-----------------------------------

Le projet est distribué sous la licence **The Unlicense**, plaçant l'intégralité du code
dans le domaine public. Vous êtes libre de :

- utiliser,
- modifier,
- redistribuer,
- commercialiser,

le projet sans aucune restriction.

-----------------------------------
7. Notes pour les contributeurs
-----------------------------------

- Le code doit rester simple, lisible et minimaliste.
- Toute modification doit être compatible avec la compilation GitHub Actions.
- Les fichiers Rust, C++ et Java doivent conserver une structure claire.
- Les contributions doivent être documentées dans le dossier ``docs/``.

-----------------------------------
8. Auteur
-----------------------------------

**Major Hamblin (Teremu)**  
Développement tactique — Rust / Android — CI/CD GitHub
