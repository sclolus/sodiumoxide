use std::env;
extern crate pkg_config;
use std::process::Command;


fn main() {

    println!("cargo:rerun-if-env-changed=SODIUM_LIB_DIR");
    println!("cargo:rerun-if-env-changed=SODIUM_STATIC");
    if let Ok(lib_dir) = env::var("SODIUM_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);

        let mode = match env::var("SODIUM_STATIC") {
            Ok(_) => "static",
            Err(_) => "dylib",
        };
        println!("cargo:rustc-link-lib={0}=sodium", mode);

    } else {
        let pwd = env::var("OUT_DIR").unwrap();

        let libsodium_path;
        let install_script;
        if std::env::var("TARGET").unwrap() == "arm-linux-androideabi" {
            install_script = "build_scripts/libsodium-android.sh";
            libsodium_path = "libsodium-stable/libsodium-android-armv6";
        } else {
            install_script = "build_scripts/libsodium-linux.sh";
            libsodium_path = "libsodium-stable/libsodium-linux";
        }
        eprintln!("Running: {}", install_script);

        let output = Command::new(install_script)
            .arg(pwd.clone())
            .output()
            .expect("Failed to execute libsodium build script");
        if output.status.success() == false {
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("Trying to find libsodium via pkg_config");
            pkg_config::find_library("libsodium").unwrap();
            return ;
        }

        println!("cargo:rustc-link-search=native={}/{}/lib", pwd, libsodium_path);
        println!("cargo:rustc-env=SODIUM_LIB_DIR={}/{}/lib", pwd, libsodium_path);
        println!("cargo:rustc-env=SODIUM_INCLUDE_DIR={}/{}/include", pwd, libsodium_path);
        println!("cargo:rustc-link-lib={0}=sodium", "static");
    }
}
