use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn rocm_path() -> PathBuf {
    env::var_os("ROCM_PATH")
        .or_else(|| env::var_os("HIP_PATH"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/opt/rocm"))
}

fn rocm_tool(name: &str) -> PathBuf {
    let candidate = rocm_path().join("bin").join(name);
    if candidate.exists() {
        candidate
    } else {
        PathBuf::from(name)
    }
}

fn target_archs() -> Vec<String> {
    if let Ok(list) = env::var("HIP_OFFLOAD_ARCH") {
        let archs: Vec<String> = list
            .split([',', ';', ' '])
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        if !archs.is_empty() {
            return archs;
        }
    }

    let tool = rocm_tool("rocm_agent_enumerator");
    let out = Command::new(&tool)
        .arg("-name")
        .output()
        .unwrap_or_else(|e| panic!("could not run {} ({e}). Is ROCm installed?", tool.display()));
    if !out.status.success() {
        panic!(
            "{} exited with {}: {}",
            tool.display(),
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }

    let mut archs: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && *l != "gfx000")
        .map(String::from)
        .collect();
    archs.sort();
    archs.dedup();

    if archs.is_empty() {
        panic!(
            "No AMD GPU detected. Set HIP_OFFLOAD_ARCH (e.g. gfx90a) to choose targets explicitly."
        );
    }
    archs
}

fn compile(hipcc: &Path, src: &Path, include_dir: &Path, out: &Path, archs: &[String], opt: &str) {
    let mut cmd = Command::new(hipcc);
    cmd.arg("--genco").arg(opt);
    for arch in archs {
        cmd.arg(format!("--offload-arch={arch}"));
    }
    cmd.arg("-I").arg(include_dir).arg(src).arg("-o").arg(out);

    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("failed to run {}: {e}", hipcc.display()));
    if !status.success() {
        panic!("hipcc failed on {} ({status})", src.display());
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_dir = manifest_dir.join("kernels");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed={}", src_dir.display());
    println!("cargo:rerun-if-env-changed=HIP_OFFLOAD_ARCH");
    println!("cargo:rerun-if-env-changed=ROCM_PATH");
    println!("cargo:rerun-if-env-changed=HIP_PATH");
    println!("cargo:rustc-link-search=native={}", rocm_path().join("lib").display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", rocm_path().join("lib").display());

    let archs = target_archs();

    let opt = format!("-O{}", env::var("OPT_LEVEL").unwrap_or_else(|_| "3".into()));

    let mut hip_files: Vec<PathBuf> = std::fs::read_dir(&src_dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", src_dir.display()))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "hip"))
        .collect();
    hip_files.sort();

    let hipcc = rocm_tool("hipcc");
    std::thread::scope(|s| {
        for src in &hip_files {
            let stem = src.file_stem().unwrap().to_string_lossy();
            let out = out_dir.join(format!("{stem}.co"));
            let (hipcc, src_dir, archs, opt) = (&hipcc, &src_dir, &archs, &opt);
            s.spawn(move || compile(hipcc, src, src_dir, &out, archs, opt));
        }
    });
}
