// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[cfg(not(target_os = "windows"))]
fn generate_bindings(_opencsd_path: &Path, _output_dir: &Path) {
    // no-op. Pre-generated bindings are used to save build time and reduce dependencies.
}

#[cfg(target_os = "windows")]
fn generate_bindings(opencsd_path: &Path, output_dir: &Path) {
    let c_api_include_path: PathBuf = [
        opencsd_path,
        Path::new("include"),
        Path::new("opencsd"),
        Path::new("c_api"),
    ]
    .iter()
    .collect();
    let opencsd_include_path: PathBuf = [opencsd_path, Path::new("decoder"), Path::new("include")]
        .iter()
        .collect();
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", &c_api_include_path.to_string_lossy()))
        .clang_arg(format!("-I{}", &opencsd_include_path.to_string_lossy()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_unsafe_ops(true)
        .generate()
        .expect("Unable to run bindgen");
    let bindings_path: PathBuf = [output_dir, Path::new("bindings.rs")].iter().collect();
    bindings
        .write_to_file(output_dir.join(&bindings_path))
        .expect("Couldn't write bindings!");
}

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // Path to vendored OpenCSD repository
    let opencsd_root: PathBuf = ["vendor", "OpenCSD", "decoder"].iter().collect();
    let opencsd_include = opencsd_root.join("include");
    let opencsd_source = opencsd_root.join("source");
    let opencsd_c_api_source = opencsd_source.join("c_api");

    // OpenCSD's makefiles have some downsides so we drive the build ourselves:
    // * They do not support out-of-tree builds
    // * They build all libraries, including unused test binaries and shared libraries
    cc::Build::new()
        .cpp(true)
        .std("c++14")
        .file(opencsd_c_api_source.join("ocsd_c_api.cpp"))
        .file(opencsd_c_api_source.join("ocsd_c_api_custom_obj.cpp"))
        .flag("-Wno-switch")
        .opt_level(2)
        .extra_warnings(false)
        .define("NDEBUG", None)
        .include(&opencsd_include)
        .include(&opencsd_c_api_source)
        .compile("opencsd_c_api");

    let decoder_files = glob::glob("vendor/OpenCSD/decoder/source/**/*.cpp")
        .unwrap()
        // Filter out the C API glue, which is compiled separately
        .filter(|path| {
            if let Ok(path) = path {
                !path
                    .components()
                    .any(|s| s == std::path::Component::Normal(OsStr::new("c_api")))
            } else {
                true
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    cc::Build::new()
        .cpp(true)
        .std("c++14")
        .files(decoder_files)
        .opt_level(2)
        .flag("-Wno-switch")
        .extra_warnings(false)
        .define("NDEBUG", None)
        .include(&opencsd_include)
        .include(&opencsd_source)
        .compile("opencsd");

    println!("cargo::rerun-if-changed=vendor/OpenCSD/decoder/include");
    println!("cargo::rerun-if-changed=vendor/OpenCSD/decoder/source");

    generate_bindings(&opencsd_root, &out_path);
}
