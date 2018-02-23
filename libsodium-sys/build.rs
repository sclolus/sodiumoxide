use std::env;
extern crate pkg_config;
use std::process::Command;


#[cfg(target_os = "android")]
const INSTALL_SCRIPT: &str = "build_scripts/libsodium-android.sh";

#[cfg(not(target_os = "android"))]
const INSTALL_SCRIPT: &str = "build_scripts/libsodium-linux.sh";

fn main() {

    println!("cargo:rerun-if-env-changed=SODIUM_LIB_DIR");
    println!("cargo:rerun-if-env-changed=SODIUM_STATIC");
    if let Ok(lib_dir) = env::var("SODIUM_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);

        let mode = match env::var_os("SODIUM_STATIC") {
            Some(_) => "static",
            None => "dylib",
        };
        println!("cargo:rustc-link-lib={0}=sodium", mode);

    } else {
        eprintln!("Running: {}", INSTALL_SCRIPT);
        let pwd = env::var("OUT_DIR").unwrap();
        let output = Command::new(INSTALL_SCRIPT)
            .arg(pwd.clone())
            .output()
            .expect("Failed to execute libsodium build script");
        if output.status.success() == false {
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("Trying to find libsodium via pkg_config");
            pkg_config::find_library("libsodium").unwrap();
            return ;
        }
        let libsodium_path;
        #[cfg(target_os = "android")] {
            libsodium_path = "libsodium-stable/libsodium-android-armv6";
        }
        #[cfg(not(target_os = "android"))] {
            libsodium_path = "libsodium-stable/libsodium-linux";
        }
        println!("cargo:rustc-link-search=native={}/{}/lib", pwd, libsodium_path);
        println!("cargo:rustc-env=SODIUM_LIB_DIR={}/{}/lib", pwd, libsodium_path);
        println!("cargo:rustc-env=SODIUM_INCLUDE_DIR={}/{}/include", pwd, libsodium_path);
        println!("cargo:rustc-link-lib={0}=sodium", "static");
    }
}
