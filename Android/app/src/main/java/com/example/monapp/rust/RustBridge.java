package com.example.monapp.rust;

public class RustBridge {
    static {
        System.loadLibrary("monapp_rust_android");
    }

    public static native String getMessage();
}
