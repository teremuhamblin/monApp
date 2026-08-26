###### README.md >> markdown
```text
monApp/Android/README.md
```

---

#📱 Module Android
- monApp

>Ce dossier contient la partie Android native de l’application monApp, chargée d’intégrer la librairie Rust compilée en cdylib et d’exposer ses fonctions via JNI.  
- L’objectif : une application Android minimaliste, fonctionnelle, et 100% compilée via GitHub Actions.

---

### 🧱 1. Structure du module Android
```text
android/
├─ settings.gradle
├─ build.gradle
└─ app/
   ├─ build.gradle
   └─ src/main/
      ├─ AndroidManifest.xml
      ├─ java/com/example/monapp/MainActivity.java
      └─ cpp/
         ├─ CMakeLists.txt
         └─ native-lib.cpp
```

####🔹 settings.gradle
Déclare le module app de l’application.

####🔹 build.gradle (racine Android)
Configuration globale du projet Android (versions SDK, plugins, etc.).

####🔹 app/build.gradle
Module principal de l’application :  
- configuration de l’APK,  
- intégration du CMake,  
- lien avec la librairie native Rust.

####🔹 MainActivity.java
Activité Android minimale.  
Charge la librairie Rust et appelle la fonction native exposée via JNI.

####🔹app/native-lib.cpp
Pont C++ entre Java ↔ Rust.  
Déclare les fonctions JNI et appelle les symboles Rust.

####🔹cpp/CMakeLists.txt
Définit la compilation du module natif et le lien avec la librairie Rust (libmonapp.so)

---

### ⚙️ 2. Intégration Rust → Android
- Le noyau Rust est situé dans :
```text
monApp/rust/
├─ Cargo.toml
└─ src/lib.rs
```

- Il est compilé en cdylib via GitHub Actions, produisant :

```text
target/aarch64-linux-android/release/libmonapp.so
```

>Cette librairie est ensuite intégrée dans le module Android.
- Chargement côté Java

```java
static {
    System.loadLibrary("monapp");
}
```

- Déclaration JNI
```java
public native String rustMessage();
```

- Pont C++ (extrait)
```cpp
extern "C" JNIEXPORT jstring JNICALL
JavacomexamplemonappMainActivity_rustMessage(JNIEnv env, jobject / this */) {
    return env->NewStringUTF(rust_message());
}
```

---

### 🏗️ 3. Compilation via GitHub Actions
>Le pipeline CI/CD est défini dans :
```build
.github/workflows/build.yml
```

- Il effectue :
1. Installation du NDK  
2. Compilation Rust → .so  
3. Intégration dans android/app/src/main/jniLibs/  
4. Compilation de l’APK  
5. Publication de l’APK dans les Artifacts GitHub

>Aucune compilation locale n’est nécessaire.

---

### 🎯 4. Objectifs du module Android
- Fournir une interface simple pour tester le noyau Rust  
- Assurer une compatibilité Android universelle  
- Minimiser le code Java/C++  
- Permettre une compilation 100% GitHub  
- Servir de base pour des projets Rust embarqués plus complexes

---

### 🪖 5. Notes pour les contributeurs
- Garder le code Java minimal  
- Ne pas ajouter de dépendances inutiles  
- Documenter toute modification dans docs/  
- Tester chaque changement via GitHub Actions  
- Respecter la structure Rust ↔ JNI ↔ Android

---

### 👤 Auteur
- Major Hamblin Teremu
>Rust — Android — CI/CD GitHub

---
