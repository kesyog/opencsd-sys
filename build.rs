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
    num::NonZero,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
))]
const MAKE_CMD: &str = "gmake";

#[cfg(not(any(
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
)))]
const MAKE_CMD: &str = "make";

#[cfg(not(target_os = "windows"))]
fn generate_bindings(_opencsd_path: &Path, _output_dir: &Path) {
    // no-op. Pre-generated bindings are used to save build time and reduce dependencies.
}

#[cfg(target_os = "windows")]
fn generate_bindings(opencsd_path: &Path, output_dir: &Path) {
    let c_api_include_path: PathBuf = [
        opencsd_path,
        Path::new("decoder"),
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
    let opencsd_path: PathBuf = ["vendor", "OpenCSD"].iter().collect();
    // OpenCSD's build generates files in-tree. Since build scripts should only write to OUT_DIR,
    // copy the library there before building it.
    fs_extra::dir::copy(
        &opencsd_path,
        &out_path,
        &fs_extra::dir::CopyOptions::new().overwrite(true),
    )
    .unwrap();
    let copied_opencsd = out_path.join("OpenCSD");

    let num_jobs = std::thread::available_parallelism().unwrap_or(NonZero::new(1).unwrap());
    let makefile_path: PathBuf = [
        &copied_opencsd.to_string_lossy(),
        "decoder",
        "build",
        "linux",
    ]
    .iter()
    .collect();
    let mut make_command = Command::new(MAKE_CMD);
    // Cargo populates the DEBUG environment variable, which clashes with a make variable in
    // OpenCSD
    make_command.env_remove("DEBUG");

    assert!(make_command
        .args(["-j", &num_jobs.to_string()])
        .args(["-C", &makefile_path.to_string_lossy()])
        .status()
        .unwrap()
        .success());
    println!("cargo:rustc-link-lib=static=opencsd");
    println!("cargo:rustc-link-lib=static=opencsd_c_api");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo::rerun-if-changed=vendor/OpenCSD/decoder/include");
    println!("cargo::rerun-if-changed=vendor/OpenCSD/decoder/source");

    let build_dir: PathBuf = [
        &copied_opencsd.to_string_lossy(),
        "decoder",
        "lib",
        "builddir",
    ]
    .iter()
    .collect();
    println!(
        "cargo:rustc-link-search=native={}",
        &build_dir.to_string_lossy()
    );

    generate_bindings(&opencsd_path, &out_path);
}
