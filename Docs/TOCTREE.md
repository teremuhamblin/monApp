###### TOCTREE.md >> rst

```text
monApp/docs/
├─ guide_utilisation.rst
├─ manuel_operateur.rst
├─ guidedeveloppementrust.rst
└─ guidecompilationinterne.rst
```

Tu peux les copier-coller tels quels.  
Ils sont cohérents avec ton architecture Android + Rust + CMake + GitHub Actions.

---

📘 1. guide_utilisation.rst

`rst
===============================
Guide d'utilisation de monApp
===============================

Ce document explique comment utiliser l'application Android monApp une fois installée
sur un appareil Android. Il s'adresse aux utilisateurs finaux souhaitant comprendre
le fonctionnement général de l'application.

-----------------------
1. Présentation
-----------------------

monApp est une application Android simple, utilisant un noyau Rust pour exécuter une
fonction native. L'application affiche un message généré par la librairie Rust compilée
pour Android.

-----------------------
2. Lancement de l'application
-----------------------

Après installation de l'APK :

1. Ouvrir l'application monApp depuis le lanceur Android.
2. L'écran principal affiche automatiquement un message provenant du noyau Rust.
3. Aucun paramétrage supplémentaire n'est requis.

-----------------------
3. Fonctionnalités
-----------------------

- Affichage d'un message natif généré par Rust.
- Liaison JNI entre Rust, C++ et Java.
- Interface minimaliste pour démonstration technique.

-----------------------
4. Limitations
-----------------------

- L'application ne dispose pas encore d'interface avancée.
- Aucune interaction utilisateur n'est disponible dans la version actuelle.
- Le message affiché est statique.

-----------------------
5. Support
-----------------------

Pour toute question ou amélioration, consulter le dépôt GitHub du projet.
`

---

📘 2. manuel_operateur.rst

`rst
===============================
Manuel opérateur - monApp
===============================

Ce manuel est destiné aux opérateurs techniques, développeurs ou mainteneurs du projet
monApp. Il décrit les responsabilités, procédures et bonnes pratiques pour manipuler
le projet.

-----------------------
1. Rôle de l'opérateur
-----------------------

L'opérateur est responsable de :

- Vérifier le bon fonctionnement du pipeline GitHub Actions.
- Maintenir la cohérence du code Rust, C++ et Java.
- Documenter toute modification dans le dossier docs/.
- S'assurer que l'APK généré est fonctionnel.

-----------------------
2. Procédures standard
-----------------------

2.1. Vérification du pipeline

1. Accéder à l'onglet Actions du dépôt.
2. Vérifier que le workflow Build Android APK s'exécute correctement.
3. En cas d'échec, consulter les logs et corriger le code.

2.2. Mise à jour du noyau Rust

1. Modifier rust/src/lib.rs.
2. Vérifier la compatibilité avec le pont C++.
3. Pousser les modifications sur main.

2.3. Mise à jour de l'application Android

1. Modifier les fichiers Java ou C++.
2. Vérifier que le CMakeLists reste cohérent.
3. Pousser les modifications.

-----------------------
3. Bonnes pratiques
-----------------------

- Garder le code Rust minimal et stable.
- Ne jamais casser la signature des fonctions JNI.
- Documenter chaque changement dans docs/.
- Tester l'APK sur un appareil Android réel.

-----------------------
4. Sécurité
-----------------------

- Ne pas inclure de données sensibles dans le code.
- Utiliser la licence The Unlicense conformément aux règles du domaine public.
`

---

📘 3. guidedeveloppementrust.rst

`rst
=========================================
Guide de développement Rust pour monApp
=========================================

Ce guide décrit la manière de développer le noyau Rust utilisé par l'application Android
monApp. Il s'adresse aux développeurs souhaitant étendre ou modifier la logique native.

-----------------------
1. Structure du module Rust
-----------------------

Le module Rust se trouve dans :

rust/  
rust/src/lib.rs  
rust/Cargo.toml

Le crate est configuré en cdylib pour produire une librairie native compatible Android.

-----------------------
2. Fonction native
-----------------------

La fonction principale exposée est :

.. code-block:: rust

[no_mangle]
   pub extern "C" fn rusthello() -> *const cchar {
       let s = CString::new("Bonjour depuis Rust (monApp Core).").unwrap();
       s.into_raw()
   }

Contraintes :

- #[no_mangle] obligatoire.
- Signature C compatible.
- Retour sous forme de *const c_char.

-----------------------
3. Compilation Android
-----------------------

La compilation est effectuée automatiquement via GitHub Actions :

- Ajout de la cible Android : aarch64-linux-android
- Compilation via cargo ndk
- Dépôt de la librairie dans android/app/src/main/jniLibs/

-----------------------
4. Ajout de nouvelles fonctions
-----------------------

Pour ajouter une nouvelle fonction native :

1. Créer une fonction Rust avec signature C.
2. Ajouter l'équivalent JNI dans native-lib.cpp.
3. Ajouter la méthode Java dans MainActivity.java.
4. Tester via compilation GitHub Actions.

-----------------------
5. Bonnes pratiques Rust
-----------------------

- Utiliser des types simples et compatibles C.
- Éviter les allocations complexes.
- Garder le code minimal et stable.
- Documenter chaque fonction dans docs/.

`

---

📘 4. guidecompilationinterne.rst

`rst
=========================================
Guide de compilation interne - monApp
=========================================

Ce document explique le fonctionnement interne de la compilation de monApp via GitHub
Actions. Il s'adresse aux développeurs et mainteneurs du pipeline CI/CD.

-----------------------
1. Pipeline GitHub Actions
-----------------------

Le pipeline se trouve dans :

.github/workflows/build.yml

Il exécute les étapes suivantes :

1. Installation de Rust
2. Installation de cargo-ndk
3. Ajout de la cible Android
4. Compilation du noyau Rust
5. Dépôt de la librairie native dans jniLibs
6. Compilation de l'APK Android
7. Export de l'APK en Artifact

-----------------------
2. Compilation Rust
-----------------------

La commande utilisée est :

.. code-block:: bash

   cargo ndk -t arm64-v8a -o ../android/app/src/main/jniLibs build

Cette commande :

- compile le crate Rust,
- génère libmonapp_core.so,
- place la librairie dans le dossier Android.

-----------------------
3. Compilation Android
-----------------------

La compilation Android est effectuée via Gradle :

.. code-block:: bash

   ./gradlew assembleDebug

Le résultat est :

android/app/build/outputs/apk/debug/monApp-debug.apk

-----------------------
4. Dépôt de l'APK
-----------------------

L'APK est automatiquement ajouté aux Artifacts du workflow :

- Nom : monApp-debug
- Format : .apk

-----------------------
5. Maintenance du pipeline
-----------------------

- Vérifier régulièrement les versions de Gradle et du NDK.
- S'assurer que les chemins Rust → JNI → Java restent cohérents.
- Documenter toute modification dans docs/.

-----------------------
6. Dépannage
-----------------------

- Si la compilation Rust échoue : vérifier Cargo.toml.
- Si la compilation Android échoue : vérifier build.gradle.
- Si l'APK n'est pas généré : vérifier les logs GitHub Actions.

`

---
