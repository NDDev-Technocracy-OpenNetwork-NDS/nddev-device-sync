//! Exercise the compiled CLI, native Git metadata and actual just recipes.
//! Every repository, artifact and subprocess belongs to an isolated fixture.
use serde_json::{Value, json};
use std::{
    ffi::OsStr,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

fn source() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned()
}

fn executable() -> &'static str {
    option_env!("CARGO_BIN_EXE_nds-source")
        .expect("Cargo must provide the actual nds-source test artifact")
}

fn isolated(command: &mut Command) -> &mut Command {
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("GIT_") {
            command.env_remove(key);
        }
    }
    command
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env(
            "GIT_CONFIG_GLOBAL",
            if cfg!(windows) { "NUL" } else { "/dev/null" },
        )
        .env("GIT_TERMINAL_PROMPT", "0")
}

fn successful(output: Output) -> Output {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn git(root: &Path, args: &[&str]) -> String {
    String::from_utf8(
        successful(
            isolated(Command::new("git").current_dir(root))
                .args(args)
                .output()
                .unwrap(),
        )
        .stdout,
    )
    .unwrap()
}

fn copy_inputs(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        if name == "target" {
            continue;
        }
        let target = to.join(name);
        if entry.file_type().unwrap().is_dir() {
            copy_inputs(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

struct Fixture {
    directory: tempfile::TempDir,
    catalog: Value,
}
impl Fixture {
    fn new() -> Self {
        let directory = tempfile::Builder::new()
            .prefix("nds-source-")
            .tempdir()
            .unwrap();
        let root = directory.path();
        copy_inputs(&source().join("automation"), &root.join("automation"));
        fs::create_dir(root.join("contracts")).unwrap();
        for name in [
            "module-catalog.schema.json",
            "module-manifest.json",
            "app-release.schema.json",
        ] {
            fs::copy(
                source().join("contracts").join(name),
                root.join("contracts").join(name),
            )
            .unwrap();
        }
        let catalog: Value = serde_json::from_slice(
            &fs::read(source().join("contracts/module-catalog.json")).unwrap(),
        )
        .unwrap();
        git(root, &["init", "--quiet", "--template="]);
        let mut sections = String::new();
        for item in catalog["components"].as_array().unwrap() {
            let path = item["path"].as_str().unwrap();
            sections.push_str(&format!(
                "[submodule \"{path}\"]\npath = {path}\nurl = https://github.com/{}/{}.git\n",
                catalog["organization"].as_str().unwrap(),
                item["repository"].as_str().unwrap()
            ));
            git(
                root,
                &[
                    "update-index",
                    "--add",
                    "--cacheinfo",
                    &format!("160000,{},{}", "a".repeat(40), path),
                ],
            );
        }
        fs::write(root.join(".gitmodules"), sections).unwrap();
        let result = Self { directory, catalog };
        result.write("contracts/module-catalog.json", &result.catalog);
        result
    }
    fn root(&self) -> &Path {
        self.directory.path()
    }
    fn write(&self, name: &str, value: &Value) {
        fs::write(self.root().join(name), serde_json::to_vec(value).unwrap()).unwrap();
    }
    fn command(&self) -> Command {
        let mut command = Command::new(executable());
        command
            .arg("--root")
            .arg(self.root())
            .current_dir(self.root());
        isolated(&mut command);
        command
    }
    fn run(&self, args: &[&str]) -> Output {
        self.command().args(args).output().unwrap()
    }
    fn accepts(&self, args: &[&str]) {
        successful(self.run(args));
    }
    fn refuses(&self, args: &[&str]) -> Output {
        let output = self.run(args);
        assert!(
            !output.status.success(),
            "invalid fixture was accepted: {args:?}"
        );
        output
    }
}

fn candidate() -> Value {
    json!({"schema":1,"application":"nds","version":"1.0.0-alpha.2",
        "platform":"linux","architecture":"x86_64","channel":"alpha",
        "sources":[{"role":"desktop","repository":"https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-desktop","commit":"a".repeat(40)}],
        "data":{"vault_journal":8,"sync_wire":3,"accounts_fence":1},
        "artifact":{"url":"https://example.invalid/app.bin","bytes":17,"sha256":"b".repeat(64),"app_source":"a".repeat(40),"native_source":"a".repeat(40)}})
}

#[test]
fn public_catalog_and_native_git_index_are_one_assembly() {
    let fixture = Fixture::new();
    fixture.accepts(&["check"]);
    assert!(!fixture.root().join("modules/core").exists());
    let patches = [
        ("repository", json!("nddev-unrelated")),
        ("platforms", json!(["all", "linux"])),
        ("status", json!("implemented")),
        ("path", json!("tools/core")),
        ("path", json!("../core")),
        ("path", Value::Null),
        ("visibility", json!("private")),
        ("kind", json!("deployment-configuration")),
    ];
    for (key, value) in patches {
        let mut bad = fixture.catalog.clone();
        bad["components"][0][key] = value;
        fixture.write("contracts/module-catalog.json", &bad);
        fixture.refuses(&["check"]);
    }
    let mut bad = fixture.catalog.clone();
    let duplicate = bad["components"][0].clone();
    bad["components"].as_array_mut().unwrap().push(duplicate);
    fixture.write("contracts/module-catalog.json", &bad);
    fixture.refuses(&["check"]);
    fixture.write("contracts/module-catalog.json", &fixture.catalog);
    let original = fs::read_to_string(fixture.root().join(".gitmodules")).unwrap();
    for bad in [
        original.replace("github.com/", "example.invalid/"),
        format!("{original}[submodule \"modules/core\"]\npath = modules/core\n"),
        format!("{original}[submodule \"modules/estate\"]\npath = modules/estate\nurl = private\n"),
    ] {
        fs::write(fixture.root().join(".gitmodules"), bad).unwrap();
        fixture.refuses(&["check"]);
    }
    fs::write(fixture.root().join(".gitmodules"), original).unwrap();
    git(
        fixture.root(),
        &["update-index", "--force-remove", "modules/core"],
    );
    fixture.refuses(&["check"]);
    git(
        fixture.root(),
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("160000,{},modules/core", "a".repeat(40)),
        ],
    );
    fs::write(
        fixture.root().join("synthetic.rs"),
        b"owned synthetic source",
    )
    .unwrap();
    let blob = git(fixture.root(), &["hash-object", "-w", "synthetic.rs"]);
    git(
        fixture.root(),
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("100644,{},modules/copied.rs", blob.trim()),
        ],
    );
    fixture.refuses(&["check"]);
}

#[test]
fn runtime_descriptors_keep_closed_permissions_and_dependency_rules() {
    let fixture = Fixture::new();
    let descriptor = json!({"id":"clipboard","version":"0.0.1-alpha.3","api_version":1,"kind":"tool","platforms":["android","ios","web"],"capabilities":["share_clipboard"],"permissions":["clipboard"],"dependencies":[]});
    fixture.write("module.json", &descriptor);
    fixture.accepts(&["check", "module.json"]);
    for (key, value) in [
        ("id", json!("bad/id")),
        ("api_version", json!(0)),
        ("api_version", json!(65536)),
        ("platforms", json!(["android", "android"])),
        ("platforms", json!(["invented"])),
        ("capabilities", json!(["invented"])),
        ("permissions", json!(["root"])),
        ("version", json!("   ")),
        ("dependencies", json!([{"id":"core","minimum_api":0}])),
        ("dependencies", json!([{"id":"clipboard","minimum_api":1}])),
        (
            "dependencies",
            json!([{"id":"core","minimum_api":1},{"id":"core","minimum_api":2}]),
        ),
    ] {
        let mut bad = descriptor.clone();
        bad[key] = value;
        fixture.write("module.json", &bad);
        fixture.refuses(&["check", "module.json"]);
    }
    let mut browser = descriptor;
    browser["id"] = json!("browser-use");
    browser["kind"] = json!("integration");
    browser["capabilities"] = json!(["control_browser"]);
    browser["permissions"] = json!(["browser_control"]);
    fixture.write("module.json", &browser);
    fixture.accepts(&["check", "module.json"]);
    browser["permissions"] = json!(["browser"]);
    fixture.write("module.json", &browser);
    fixture.refuses(&["check", "module.json"]);
    // This would be a valid descriptor if duplicate keys silently won.
    browser["permissions"] = json!(["browser_control"]);
    let encoded = serde_json::to_string(&browser)
        .unwrap()
        .replacen('{', "{\"api_version\":1,", 1);
    fs::write(fixture.root().join("module.json"), encoded).unwrap();
    fixture.refuses(&["check", "module.json"]);
    // The descriptor's version contract permits any non-whitespace string.
    browser["version"] = json!("SYNTHETIC_SENTINEL\n");
    fixture.write("module.json", &browser);
    fixture.accepts(&["check", "module.json"]);
    browser["id"] = json!("SYNTHETIC_SENTINEL/");
    fixture.write("module.json", &browser);
    let output = fixture.refuses(&["check", "module.json"]);
    assert!(!String::from_utf8_lossy(&output.stderr).contains("SYNTHETIC_SENTINEL"));
}

#[test]
fn application_candidates_bind_public_sources_and_artifact_identity() {
    let fixture = Fixture::new();
    let original = candidate();
    fixture.write("app.json", &original);
    fixture.accepts(&["check", "--app-release", "app.json"]);
    for version in ["1.0.0-alpha.1", "1.0.0-alpha.4294967295"] {
        let mut value = original.clone();
        value["version"] = json!(version);
        fixture.write("app.json", &value);
        fixture.accepts(&["check", "--app-release", "app.json"]);
    }
    for (pointer, replacement) in [
        ("/version", json!("1.0.0-alpha.0")),
        ("/version", json!("1.0.0-alpha.01")),
        ("/version", json!("1.0.0-alpha.4294967296")),
        ("/version", json!("1.0.0-alpha.1\n")),
        ("/channel", json!("release")),
        ("/platform", json!("windows")),
        ("/artifact/native_source", json!("c".repeat(40))),
        ("/artifact/bytes", json!(1073741825_u64)),
        ("/artifact/url", json!("file:///tmp/app")),
        ("/data", json!({"schema":3})),
    ] {
        let mut value = original.clone();
        *value.pointer_mut(pointer).unwrap() = replacement;
        fixture.write("app.json", &value);
        fixture.refuses(&["check", "--app-release", "app.json"]);
    }
    let mut value = original.clone();
    value["publisher"] = json!({"issuer":"candidate-selected"});
    fixture.write("app.json", &value);
    fixture.refuses(&["check", "--app-release", "app.json"]);
    for location in [
        "example/private-component",
        "NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-unknown",
    ] {
        let mut value = original.clone();
        value["sources"].as_array_mut().unwrap().push(json!({"role":"unknown","repository":format!("https://github.com/{location}"),"commit":"c".repeat(40)}));
        fixture.write("app.json", &value);
        fixture.refuses(&["check", "--app-release", "app.json"]);
    }
    for item in fixture.catalog["components"].as_array().unwrap() {
        let mut value = original.clone();
        value["sources"].as_array_mut().unwrap().push(json!({"role":"component","repository":format!("https://github.com/{}/{}.git", fixture.catalog["organization"].as_str().unwrap().to_lowercase(), item["repository"].as_str().unwrap()),"commit":"c".repeat(40)}));
        fixture.write("app.json", &value);
        fixture.accepts(&["check", "--app-release", "app.json"]);
    }
    let mut value = original.clone();
    let mut shared = value["sources"][0].clone();
    shared["role"] = json!("adapter-io");
    shared["commit"] = json!("c".repeat(40));
    value["sources"].as_array_mut().unwrap().push(shared);
    fixture.write("app.json", &value);
    fixture.accepts(&["check", "--app-release", "app.json"]);
    value["sources"][1]["role"] = json!("desktop");
    fixture.write("app.json", &value);
    fixture.refuses(&["check", "--app-release", "app.json"]);
    for url in [
        "http://127.0.0.1:8080/app",
        "http://localhost:8080/app",
        "http://127.1:8080/app",
        "http://127.0.0.1:08080/app",
        "https://user@example.invalid/app",
        "https://example.invalid/app#fragment",
    ] {
        let mut value = original.clone();
        value["artifact"]["url"] = json!(url);
        fixture.write("app.json", &value);
        fixture.refuses(&["check", "--app-release", "app.json"]);
        if url == "http://127.0.0.1:8080/app" {
            fixture.accepts(&["check", "--development-http", "--app-release", "app.json"]);
        } else {
            fixture.refuses(&["check", "--development-http", "--app-release", "app.json"]);
        }
    }
}

#[test]
fn verifier_freshness_and_owned_installation_survive_reruns() {
    let fixture = Fixture::new();
    fixture.accepts(&["install-tool"]);
    let installed = fixture
        .root()
        .join(".tools/bin")
        .join(format!("nds-source{}", std::env::consts::EXE_SUFFIX));
    let first = fs::read(&installed).unwrap();
    fixture.accepts(&["install-tool"]);
    assert_eq!(fs::read(&installed).unwrap(), first);
    successful(
        Command::new(&installed)
            .arg("--root")
            .arg(fixture.root())
            .arg("check")
            .output()
            .unwrap(),
    );
    fs::remove_file(fixture.root().join(".tools/nds-source.json")).unwrap();
    fixture.accepts(&["install-tool"]); // lost receipt after publishing the exact binary
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(fixture.root().join(".tools/nds-source-install.lock"))
        .unwrap();
    lock.lock().unwrap();
    fixture.refuses(&["install-tool"]);
    drop(lock);
    fs::write(&installed, b"unrelated file").unwrap();
    fixture.refuses(&["install-tool"]);
    assert_eq!(fs::read(&installed).unwrap(), b"unrelated file");
    fs::OpenOptions::new()
        .append(true)
        .open(fixture.root().join("automation/src/main.rs"))
        .unwrap()
        .write_all(b"\n// changed source\n")
        .unwrap();
    let refused = fixture.refuses(&["check"]);
    assert!(String::from_utf8_lossy(&refused.stderr).contains("stale"));
}

#[test]
fn component_dispatch_uses_actual_git_just_cwd_arguments_and_exit_status() {
    let fixture = Fixture::new();
    fixture.refuses(&["component", "unlisted"]);
    fixture.refuses(&["component", "core"]);
    let child = fixture.root().join("modules/core");
    fs::create_dir_all(&child).unwrap();
    git(&child, &["init", "--quiet", "--template="]);
    let origin = "https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-core.git";
    git(&child, &["remote", "add", "origin", origin]);
    fs::write(child.join("justfile"), "set shell := [\"bash\", \"-euo\", \"pipefail\", \"-c\"]\n[positional-arguments]\nprobe *args:\n    \"$NDS_FIXTURE_PROBE\" \"$NDS_FIXTURE_OUTPUT\" \"$@\"\n").unwrap();
    git(&child, &["add", "justfile"]);
    git(
        &child,
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-qm",
            "fixture",
        ],
    );
    let commit = git(&child, &["rev-parse", "HEAD"]);
    git(
        fixture.root(),
        &[
            "update-index",
            "--cacheinfo",
            &format!("160000,{},modules/core", commit.trim()),
        ],
    );
    let probe = compile_argument_probe(fixture.root());
    let output_path = fixture.root().join("arguments.json");
    let output = fixture
        .command()
        .args([
            "component",
            "core",
            "probe",
            "two words",
            "$(never-run)",
            "; literal",
        ])
        .env("NDS_FIXTURE_PROBE", &probe)
        .env("NDS_FIXTURE_OUTPUT", &output_path)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(7),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut actual = Vec::new();
    fs::File::open(output_path)
        .unwrap()
        .read_to_end(&mut actual)
        .unwrap();
    let mut expected = Vec::new();
    for arg in [
        child.as_os_str(),
        OsStr::new("two words"),
        OsStr::new("$(never-run)"),
        OsStr::new("; literal"),
    ] {
        let value = arg.as_encoded_bytes();
        expected.extend_from_slice(&(value.len() as u64).to_le_bytes());
        expected.extend_from_slice(value);
    }
    assert_eq!(actual, expected);
    git(
        &child,
        &[
            "remote",
            "set-url",
            "origin",
            "https://example.invalid/foreign.git",
        ],
    );
    fixture.refuses(&["component", "core", "probe"]);
    git(&child, &["remote", "set-url", "origin", origin]);
    git(
        fixture.root(),
        &[
            "update-index",
            "--cacheinfo",
            &format!("160000,{},modules/core", "a".repeat(40)),
        ],
    );
    fixture.refuses(&["component", "core", "probe"]);
}

fn compile_argument_probe(root: &Path) -> PathBuf {
    let directory = root.join("argument-probe");
    fs::create_dir(&directory).unwrap();
    fs::write(directory.join("Cargo.toml"), "[workspace]\n[package]\nname=\"nds-argument-probe\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[[bin]]\nname=\"nds-argument-probe\"\npath=\"main.rs\"\n").unwrap();
    fs::write(
        directory.join("main.rs"),
        r#"use std::{env,fs::File,io::Write};
fn main() {
    let mut args = env::args_os().skip(1);
    let mut output = File::create(args.next().unwrap()).unwrap();
    for arg in std::iter::once(env::current_dir().unwrap().into_os_string()).chain(args) {
        let bytes = arg.as_encoded_bytes();
        output.write_all(&(bytes.len() as u64).to_le_bytes()).unwrap();
        output.write_all(bytes).unwrap();
    }
    output.sync_all().unwrap(); std::process::exit(7);
}"#,
    )
    .unwrap();
    let output = successful(
        Command::new("cargo")
            .args([
                "build",
                "--offline",
                "--message-format=json",
                "--manifest-path",
            ])
            .arg(directory.join("Cargo.toml"))
            .stdin(Stdio::null())
            .output()
            .unwrap(),
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .find_map(|value| {
            (value["reason"] == "compiler-artifact")
                .then(|| value["executable"].as_str().map(PathBuf::from))
                .flatten()
        })
        .expect("Cargo must report the real probe artifact")
}
