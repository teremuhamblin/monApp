###### README.md >> markdown 
- Simple, propre, clair, spécialement conçu pour le dossier Docs/.  
Il résume le projet **monApp** avec les détails techniques du code, des documents divers tout en restant structuré.
```text
monApp/Docs/README.md
```

---

# 📘 Documentation du projet monApp
### 🪖 1. Présentation générale
- monApp est une application Android simple, construite avec **une architecture native Rust + C++ + Java.**  
   - Le projet est conçu pour fonctionner sans outils locaux, uniquement via GitHub, grâce à une compilation automatisée par GitHub Actions.

   - Ce dossier docs/ regroupe la documentation générale du projet :  
- objectifs  
- architecture  
- fonctionnement  
- instructions de build  
- informations de licence

---

### 🧱 2. Architecture du projet
>Le projet est divisé en trois blocs principaux :

#### 🔹 Rust (monapp_core)
Noyau logique de l’application.  
Compile en librairie native (.so) pour Android.

#### 🔹 Android (Java + CMake)
Interface Android minimale.  
Charge la librairie Rust via JNI.

#### 🔹 GitHub Actions (CI/CD)
Pipeline automatisé qui :  
- compile Rust pour Android  
- génère la librairie native  
- compile l’APK  
- fournit l’APK en artifact

---

### ⚙️ 3. Fonctionnement global
1. Le code Rust expose une fonction native.  
2. Le pont C++/JNI relie Rust à Java.  
3. L’application Android affiche le résultat dans une interface simple.  
4. GitHub Actions compile automatiquement l’ensemble du projet à chaque push.

---

### 🛠️ 4. Compilation via GitHub
>Aucune installation locale n’est nécessaire.
- Pour récupérer l’APK :
   1. Pousser du code sur la branche main  
   2. Aller dans Actions  
   3. Ouvrir le workflow Build Android APK  
   4. Télécharger l’APK dans les Artifacts

---

### 📦 5. Structure du dépôt
```text
monApp/
├─ rust/          → Noyau Rust
├─ android/       → Projet Android
├─ .github/       → Pipeline CI/CD
├─ docs/          → Documentation
├─ LICENSE        → The Unlicense
└─ README.md      → README principal du dépôt
```

---

### 📄 6. Licence
>Le projet est publié sous The Unlicense, ce qui le place dans le domaine public.  
Tu peux utiliser, modifier, distribuer ou commercialiser le code librement.

---

### 🎯 7. Objectifs du projet
- Fournir une base Android + Rust simple et fonctionnelle  
- Permettre la compilation uniquement via GitHub  
- Servir de modèle pour des projets Rust embarqués  
- Offrir une documentation claire et accessible

---

### 📝 8. Notes pour les contributeurs
   - Le code doit rester simple et lisible  
   - Les modifications doivent être documentées dans ce dossier  
   - Les améliorations UI/UX doivent rester minimalistes  
   - Toute nouvelle fonctionnalité doit être testée via GitHub Actions

---

### 🪖 9. Auteur

- Major Hamblin (Teremu)  
>Développement tactique — Rust / Android — CI/CD GitHub

---
