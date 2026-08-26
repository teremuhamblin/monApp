###### ROADMAP.md >> markdown
```text
monApp/ROADMAP.md
```

---

# 🛣️ monApp
- ROADMAP Officielle

Cette feuille de route décrit l’évolution stratégique de monApp, depuis le premier Hello World Rust → Android jusqu’à la version stable v1.0.  
Chaque version est un jalon clair, mesurable, et orienté vers la robustesse du noyau Rust et l’intégration Android.

---

### 🟩 v0.1.0
>Hello World Rust → Android
- Objectif :
   - Etablir la base Rust ↔ Android
   - Compilation Rust en cdylib
   - Génération de libmonapp.so
   - Intégration JNI minimale
   - App Android affichant un message provenant de Rust
   - Mise en place du pipeline GitHub Actions (build Rust + APK)

Livrables :
   - Fonction Rust exposée via FFI
   - MainActivity + JNI opérationnels  
   - APK fonctionnel minimal
 
---

### 🟦 v0.2.0
>Interface Android simple
Objectif : donner une forme à l’application

- Création d’une UI Android minimaliste
- Affichage propre du message Rust
- Structure Android clarifiée (Java + C++ + CMake)
- Gestion des erreurs JNI basique
- Ajout d’un layout XML propre

Livrables :
- activity_main.xml  
- UI stable, sans dépendances  
- Interaction utilisateur minimale

---

### 🟥 v0.3.0
- Communication Rust ↔ Android avancée
Objectif : transformer Rust en moteur logique

- Ajout de plusieurs fonctions Rust exposées via FFI
- Gestion des chaînes, entiers, structures simples
- Conversion Rust → JNI → Java propre
- Nettoyage mémoire (CString, into_raw, etc.)
- Tests de communication via GitHub Actions

Livrables :
- API Rust minimaliste  
- Pont JNI robuste  
- Documentation Rust ↔ JNI

---

### 🟧 v0.4.0
- Fonctionnalités
Objectif : transformer monApp en application réelle

- Ajout de fonctionnalités Rust (calculs, logique, parsing…)
- UI Android améliorée (boutons, interactions)
- Appels Rust depuis plusieurs éléments UI
- Gestion d’erreurs Rust → Android
- Logs Rust → Android (via JNI)
- Optimisation du pipeline CI/CD

Livrables :
- Fonctionnalités Rust utilisables depuis Android  
- UI interactive  
- Build stable et reproductible

---

### 🟩 v1.0.0 — Release stable
Objectif : livrer une version officielle, propre, documentée

- Stabilisation du noyau Rust
- Stabilisation du module Android
- Documentation complète (docs/, READMEs)
- Release GitHub officielle (APK + sources)
- Licence The Unlicense confirmée
- Architecture validée pour évolutions futures

Livrables :
- Release v1.0 stable  
- APK final  
- Documentation complète  
- Projet prêt pour extension v1.1+

---

### 🚀 Vision long-terme (post‑v1.0)
- v1.1 — UI avancée + animations  
- v1.2 — Module Rust async + tokio  
- v1.3 — Système de logs Rust → Android  
- v1.4 — Mode Ghost Recon (UI militaire)  
- v2.0 — Architecture modulaire Rust + Android

---
