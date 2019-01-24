extern crate cc;
extern crate pkg_config;
extern crate vcpkg;

use std::env;
use std::path::PathBuf;


fn main() {
    // Find and link OpenEXR and IlmBase
    let mut include_paths = Vec::new();

    let suffix = if let Ok(v) = env::var("OPENEXR_LIB_SUFFIX") {
        format!("-{}", v)
    } else {
        "".into()
    };

    let found_through_vcpkg = {
        // returns true if found through vcpkg: no need to look for dependencies then
        let mut find_openexr = || {
            if let Ok(path) = env::var("OPENEXR_DIR") {
                // There's an environment variable, so let's use that
                println!("cargo:rustc-link-search=native={}/lib", path);
                println!("cargo:rustc-link-lib=static=IlmImf{}", suffix);
                println!("cargo:rustc-link-lib=static=IlmImfUtil{}", suffix);
                include_paths.push(PathBuf::from(&format!("{}/include/OpenEXR", path)));
                return false;
            }

            // There's no enviroment variable, so use pkgconfig or vcpkg to find the libs
            // try pkgconfig
            let cfg = pkg_config::Config::new()
                .atleast_version("2.0.0")
                .probe("OpenEXR");

            let pkgconfig_err = match cfg {
                Ok(cfg) => {
                    include_paths.extend(cfg.include_paths);
                    return false;
                }
                Err(e) => {
                    e
                }
            };

            // try vcpkg
            let vcpkg_err = {
                env::set_var("VCPKGRS_DYNAMIC", "1");
                let lib = vcpkg::find_package("openexr");
                match lib {
                    Ok(lib) => {
                        eprintln!("found OpenEXR through vcpkg");
                        eprintln!("-> libs: {:?}", lib.found_libs);
                        eprintln!("-> link paths: {:?}", lib.link_paths);
                        eprintln!("-> include paths: {:?}", lib.include_paths);
                        eprintln!("-> DLLs: {:?}", lib.found_dlls);
                        eprintln!("-> DLL paths: {:?}", lib.dll_paths);

                        include_paths.extend(lib.include_paths.iter().cloned());
                        // fix includes because OpenEXR and vcpkg hate us
                        let mut inc = lib.include_paths.first().unwrap().clone();
                        inc.push("OpenEXR");
                        include_paths.push(inc);
                        
                        return true;
                    }
                    Err(e) => {
                        e
                    }
                }
            };

            // nothing worked
            panic!(
                "couldn't find OpenEXR: environment variable \
                         OPENEXR_DIR is unset; pkg-config failed: {}; vcpkg failed: {}",
                pkgconfig_err, vcpkg_err
            )
        };

        find_openexr()
    };

    if !found_through_vcpkg {
        if let Ok(path) = env::var("ILMBASE_DIR") {
            println!("cargo:rustc-link-search=native={}/lib", path);
            println!("cargo:rustc-link-lib=static=IexMath{}", suffix);
            println!("cargo:rustc-link-lib=static=Iex{}", suffix);
            println!("cargo:rustc-link-lib=static=Imath{}", suffix);
            println!("cargo:rustc-link-lib=static=IlmThread{}", suffix);
            println!("cargo:rustc-link-lib=static=Half");
            include_paths.push(PathBuf::from(&format!("{}/include/OpenEXR", path)));
        } else {
            let paths = pkg_config::Config::new()
                .atleast_version("2.0.0")
                .cargo_metadata(false) // OpenEXR already pulls in all the flags we need
                .probe("IlmBase")
                .map(|ilmbase_cfg| ilmbase_cfg.include_paths.clone())
                .map_err(|err| {
                    panic!(
                        "couldn't find IlmBase: environment variable \
                     ILMBASE_DIR is unset and pkg-config failed: {}",
                        err
                    )
                }).unwrap();
            include_paths.extend_from_slice(&paths);
        }


        // Find and link zlib, needed for OpenEXR
        // Use environment variable if it exists, and otherwise use pkgconfig.
        if let Ok(path) = env::var("ZLIB_DIR") {
            println!("cargo:rustc-link-search=native={}/lib", path);
            println!("cargo:rustc-link-lib=static=zlibstatic");
        } else if let Err(err) = pkg_config::probe_library("zlib") {
            panic!(
                "couldn't find zlib: environment variable ZLIB_DIR is unset \
             and pkg-config failed: {}",
                err
            );
        }
    }

    // Build C wrapper for OpenEXR
    let mut cc = cc::Build::new();
    cc.cpp(true).include("c_wrapper");
    #[cfg(target_env = "msvc")]
    cc.flag("/std:c++14");
    #[cfg(not(target_env = "msvc"))]
    cc.flag("-std=c++0x");
    for path in &include_paths {
        cc.include(path);
    }
    cc.file("c_wrapper/cexr.cpp")
        .file("c_wrapper/rust_istream.cpp")
        .file("c_wrapper/memory_istream.cpp")
        .file("c_wrapper/rust_ostream.cpp")
        .compile("libcexr.a");
}
