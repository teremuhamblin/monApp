###### README.md >> markdown
- Pour Android avec :
   + Rust
   + CMake
   + GitHub Actions

---

# 🟥 Mon Projet 
- **monApp**
Application Android tactique, propulsée par un noyau Rust, compilée automatiquement via GitHub Actions.

---

### 🪖 1. Présentation
- opérationnelle
monApp est une application Android minimaliste mais robuste, conçue selon une architecture militaire :
```md
- Rust pour le noyau logique (monapp_core)  
- C++ / JNI pour le pont tactique  
- Java Android pour l’interface  
- CMake pour l’intégration native  
- GitHub Actions pour la compilation automatique de l’APK (aucun outil local requis)
```

>Ce projet est pensé pour fonctionner uniquement avec :
- Un navigateur web,
- Sans Android Studio,
- Sans SDK local,
- Sans cargo-ndk installé sur ta machine.  
   - GitHub exécute toute la chaîne de compilation comme un centre de commandement automatisé.

---

### 🟦 2. Architecture du projet

[![pages-build-deployment](https://github.com/teremuhamblin/monApp/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/teremuhamblin/monApp/actions/workflows/pages/pages-build-deployment)

```text
monApp/
├─ rust/                     # Noyau Rust (cdylib)
│  ├─ Cargo.toml
│  └─ src/lib.rs
│
├─ android/                  # Application Android
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
│
├─ .github/workflows/        # Pipeline GitHub Actions
│  └─ build.yml
│
├─ LICENSE                   # The Unlicense (domaine public)
├─ .gitignore
└─ README.md
```

---

### 🟩 3. Fonctionnement tactique
**🔹 Rust (monapp_core)**
Le noyau Rust fournit une fonction native :
```rust
rust_hello() → "Bonjour depuis Rust (monApp Core)."
```
>Cette fonction est exportée en C, puis récupérée par le pont ***C++/JNI***, et enfin affichée dans l’interface Android.

**🔹 C++ / JNI**
```md
Le fichier native-lib.cpp agit comme liaison tactique entre Rust et Java.
```

**🔹 Java Android**
```md
MainActivity charge la librairie native et affiche le message Rust dans un TextView.
```

**🔹 CMake**
```md
Compile le pont C++ et lie la librairie Rust générée par GitHub Actions.
```

**🔹 GitHub Actions**
>Le pipeline :
```text
1. Installe Rust  
2. Installe cargo-ndk  
3. Compile la librairie Rust pour Android  
4. Intègre la librairie dans jniLibs/  
5. Compile l’APK  
6. Dépose l’APK dans les Artifacts de GitHub
```

---

### 🟧 4. Compilation automatique
>CI/CD militaire
- Chaque push sur main déclenche :
   - Compilation Rust (target Android)  
   - Génération de la librairie native .so  
   - Compilation Android via Gradle  
   - Export de l’APK dans les Artifacts

>📦 Récupération de l’APK
```md
1. Va dans Actions  
2. Sélectionne le workflow Build Android APK  
3. Télécharge l’Artifact monApp-debug.apk
```

- Aucune installation locale requise.  
Ton navigateur Chrome suffit.

---

### 🟪 5. Installation sur Android
>Une fois l’APK téléchargé :
```md
1. Transfère l’APK sur ton smartphone  
2. Active Installer des applications inconnues  
3. Installe monApp-debug.apk  
4. Lance l’application  
5. Le message Rust s’affiche immédiatement
```
---

### 🟫 6. Licence
>Ce projet est placé sous The Unlicense, ce qui signifie :
```text
- Domaine public  
- Libre utilisation  
- Libre modification  
- Libre distribution  
- Libre commercialisation  
- Aucun copyright  
- Aucun droit réservé
```

***Tu es libre d’en faire ce que tu veux, sans restriction.***

---

### 🟨 7. Objectifs tactique
- (ROADMAP)
#### 🎯 v0.1.0 — État actuel
- Rust → C++ → Java opérationnel  
- APK compilé automatiquement  
- Architecture complète fonctionnelle  
- Licence Unlicense intégrée

#### 🎯 v0.2.0 — Améliorations
- Interface Android plus avancée  
- Boutons, interactions, affichage dynamique  
- Appels multiples Rust → Android

#### 🎯 v0.3.0 — Mode opérateur
- Menu militaire  
- Thème sombre tactique  
- Logo ASCII monApp

#### 🎯 v1.0.0 — Release stable
- Publication automatique dans GitHub Releases  
- Signature APK  
- Documentation complète

---

### 🟥 8. Auteur
- Major Hamblin (Teremu)  
   - Développeur tactique — Architecte    - Rust/Android — Opérateur GitHub CI/CD

---

### 🟦 9. Message final
```md
Ce dépôt est conçu comme une application de démonstration militaire, robuste, simple, efficace, entièrement compilée dans le cloud.  
Tu n’as besoin que de GitHub et d’un navigateur pour déployer une application Android native en Rust.
```
