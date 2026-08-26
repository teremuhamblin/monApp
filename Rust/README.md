###### README.md >> markdown 
```README 
monApp/Rust/README.md
```

---

# 🦀 Module Rust
- monApp

>Le dossier rust/ contient le noyau logique de l’application monApp.  
Il est compilé en cdylib pour produire la librairie native Android (libmonapp.so) utilisée par le module Android via JNI.

- Ce module représente la partie moteur de l’application : rapide, sûre, minimale, et entièrement compilée via GitHub Actions.

---

### 🧱 1. Structure du module Rust
```text
rust/
├─ Cargo.toml
└─ src/lib.rs
```

####🔹 Cargo.toml
Déclare le projet Rust, son type de compilation (cdylib), ses dépendances, et les cibles Android.

####🔹 src/lib.rs
Contient les fonctions Rust exposées à Android via FFI.  
Ces fonctions sont appelées par le pont JNI dans android/app/src/main/cpp/native-lib.cpp.

---

### ⚙️ 2. Compilation en cdylib
>Android
- Le module Rust est compilé en librairie native :
```text
libmonapp.so
```

>Pour les architectures Android (ex. aarch64-linux-android).
- Cette librairie est ensuite intégrée dans :
```text
android/app/src/main/jniLibs/<ABI>/libmonapp.so
```

---

### 🔗 3. Intégration Rust → JNI → Android
>Fonction Rust exposée
```rust
[no_mangle]
pub extern "C" fn rustmessage() -> *const cchar {
    let msg = CString::new("Hello from Rust!").unwrap();
    msg.into_raw()
}
```

>Appel côté C++ (JNI)
```cpp
extern "C" JNIEXPORT jstring JNICALL
JavacomexamplemonappMainActivity_rustMessage(JNIEnv env, jobject / this */) {
    return env->NewStringUTF(rust_message());
}
```

>Appel côté Java
```java
public native String rustMessage();
```

---

### 🏗️ 4. Compilation via GitHub Actions
>Le pipeline CI/CD situé dans :
```yaml
.github/workflows/build.yml
```

effectue automatiquement :

1. Installation du NDK  
2. Compilation Rust pour Android  
3. Génération de libmonapp.so  
4. Intégration dans le module Android  
5. Compilation de l’APK  
6. Publication de l’APK dans les Artifacts GitHub

- Aucune installation locale n’est nécessaire.

---

### 🎯 5. Objectifs du module Rust

- Fournir un noyau logique performant et sécurisé  
- Minimiser le code et les dépendances  
- Exposer des fonctions simples via FFI  
- Servir de base pour des extensions Rust futures  
- Garantir une compilation reproductible via GitHub Actions

---

### 🪖 6. Notes pour les contributeurs

- Le code Rust doit rester simple, stable, documenté  
- Toute nouvelle fonction doit être exposée proprement via FFI  
- Les modifications doivent être documentées dans docs/  
- Tester chaque changement via GitHub Actions  
- Respecter la chaîne Rust → JNI → Android

---

### 👤 Auteur

- Major Hamblin (Teremu)  
>Rust — Android — CI/CD GitHub

---
