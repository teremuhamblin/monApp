###### CHANGELOG.md >> markdown
- complet, propre, structuré, militaire, conforme aux standards **SemVer**, et parfaitement aligné avec ta roadmap et l’évolution réelle de monApp.
```text
monApp/CHANGELOG.md
```

---

# 📜 CHANGELOG
- monApp

Toutes les modifications majeures du projet monApp sont documentées ici.  
Format conforme à Keep a Changelog + SemVer.

---

### 🟩 [1.0.0] — Release stable
- Date : 2026-08-26
     - ***🚀 Ajouté***
- Release stable de l’application Android + noyau Rust
- Documentation complète :
  - README principal
  - README Rust
  - README Android
  - ROADMAP.md
- Pipeline GitHub Actions entièrement fonctionnel :
  - Build Rust (cdylib)
  - Intégration JNI
  - Compilation APK
  - Publication des Artifacts
- Architecture finale validée :
  - Rust → JNI → Java → UI Android
  - CMakeLists propre
  - MainActivity minimaliste et stable

      - ***🔧 Modifié***
- Stabilisation du pont JNI
- Nettoyage du code Rust (FFI, CString, into_raw)
- Optimisation du layout Android

     - ***🛡️ Corrigé***
- Problèmes de chargement de librairie Rust
- Chemins JNI incorrects
- Incohérences dans les builds multi‑ABI

---

### 🟧 [0.4.0] — Fonctionnalités
Date : 2026-08-25

🚀 Ajouté
- Nouvelles fonctions Rust exposées via FFI
- UI Android améliorée (boutons, interactions)
- Gestion d’erreurs Rust → Android
- Logs Rust → Android via JNI
- Optimisation du pipeline CI/CD

🔧 Modifié
- Structure du module Android
- CMakeLists pour meilleure compatibilité NDK

🛡️ Corrigé
- Crash JNI lors de conversions de chaînes
- Problèmes de mémoire liés à CString

---

### 🟥 [0.3.0] — Communication Rust ↔ Android
Date : 2026-08-24

🚀 Ajouté
- Pont JNI complet (native-lib.cpp)
- Plusieurs fonctions Rust exposées
- Gestion des types Rust → JNI → Java
- Tests de communication via GitHub Actions

🔧 Modifié
- MainActivity pour supporter plusieurs appels natifs

🛡️ Corrigé
- Conversion UTF‑8 incorrecte
- Mauvais mapping des signatures JNI

---

### 🟦 [0.2.0] — UI Android simple
Date : 2026-08-23

🚀 Ajouté
- Première interface Android :
  - activity_main.xml
  - affichage du message Rust
- Structure Android clarifiée :
  - Java
  - C++
  - CMake

🔧 Modifié
- Organisation du module Android

🛡️ Corrigé
- Erreurs de layout XML
- Problèmes de compatibilité SDK

---

### 🟩 [0.1.0] — Hello World Rust → Android
Date : 2026-08-22

🚀 Ajouté
- Première fonction Rust exposée via FFI
- Compilation Rust en cdylib (libmonapp.so)
- Intégration JNI minimale
- App Android affichant un message Rust
- Pipeline GitHub Actions initial :
  - Build Rust
  - Build APK
  - Artifacts

🔧 Modifié
- Structure initiale du projet

🛡️ Corrigé
- Problèmes de linkage Rust → Android

---

### 🗂️ Format du changelog

- Ajouté : nouvelles fonctionnalités  
- Modifié : changements dans le comportement existant  
- Corrigé : bugs résolus  
- Supprimé : fonctionnalités retirées  
- Sécurité : correctifs liés à la sécurité  

---
