extern crate bindgen;
extern crate cc;
extern crate pkg_config;
extern crate vcpkg;

use std::{env, error::Error, path::PathBuf};

fn main() {
    let mut found_alembic = false;
    let mut include_paths = Vec::new();
    env::set_var("VCPKGRS_DYNAMIC", "1");

    let lib = vcpkg::Config::new().copy_dlls(true).probe("alembic");
    match lib {
        Ok(lib) => {
            eprintln!("found alembic through vcpkg");
            eprintln!("-> libs: {:?}", lib.found_libs);
            eprintln!("-> link paths: {:?}", lib.link_paths);
            eprintln!("-> include paths: {:?}", lib.include_paths);
            eprintln!("-> DLLs: {:?}", lib.found_dlls);
            eprintln!("-> DLL paths: {:?}", lib.dll_paths);
            found_alembic = true;
            include_paths = lib.include_paths;
        }
        Err(err) => {
            eprintln!("ERROR: {}", err.description());
        }
    }

    if !found_alembic {
        // try pkg-config
        //eprintln!("-> trying to detect OpenImageIO through pkg-config...");
        let lib = pkg_config::probe_library("alembic");
        match lib {
            Ok(lib) => {
                eprintln!("Found alembic through pkg-config");
                eprintln!("-> libs: {:?}", lib.libs);
                eprintln!("-> link paths: {:?}", lib.link_paths);
                eprintln!("-> include paths: {:?}", lib.include_paths);
                found_alembic = true;
                include_paths = lib.include_paths;
            }
            Err(err) => {
                eprintln!("ERROR: {}", err.description());
            }
        }
    }

    // fix includes because OpenEXR hates us
    let mut inc = include_paths.first().unwrap().clone();
    inc.push("OpenEXR");
    include_paths.push(inc);

    // nothing worked, bail out
    if !found_alembic {
        panic!("Could not find alembic");
    }

    // bindgen our functions
    let bindings = {
        let mut builder = bindgen::Builder::default();
        // The input header we would like to generate
        builder = builder
            .header("src/glue.hpp")
            .clang_arg("-std=c++14")
            .clang_arg("-v")
            //.enable_cxx_namespaces()
            //.disable_name_namespacing()
            // don't include methods and functions: we replace them with a C API
            .derive_copy(true)
            .with_codegen_config(
                bindgen::CodegenConfig::TYPES
                    | bindgen::CodegenConfig::VARS
                    | bindgen::CodegenConfig::FUNCTIONS,
            )
            // Hide std (shouldn't appear anyway)
            //.blacklist_type("std::.*")
            // whitelist all easily representable types
            .whitelist_type("Alembic::AbcCoreAbstract::.*")
            .whitelist_type("CStringRef")
            .whitelist_type("SliceU64")
            // whitelist our C API
            .whitelist_function("Alembic_.*")
            // make unneeded types opaque
            // note: don't make base classes opaque as this can interfere with layout
            // in case of empty base class optimization
            .opaque_type("std::.*")
            .opaque_type("Alembic::Util::v11::Dimensions");

        // add include paths
        for p in include_paths.iter() {
            builder = builder.clang_arg(format!("-I{}", p.to_str().unwrap()));
            println!("-I{}", p.to_str().unwrap());
        }
        // Finish the builder and generate the bindings.
        builder.generate()
    }
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("alembic_bindings.rs"))
        .expect("Couldn't write bindings!");

    // compile bindings
    let mut build = cc::Build::new();
    build.file("src/glue.cpp");
    build.include("wrapper");
    for p in include_paths.iter() {
        build.include(p);
    }
    build.compile("wrapper");
}
