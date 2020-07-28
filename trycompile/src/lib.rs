//! A prototype for a small partial trybuild alternative. I personally use it complementary to
//! trybuild, since I've been experiencing a few problems with it (and no time yet to debug
//! an unfamiliar crate).
//!
//! Not production ready; in a messy state; prototype only

use eyre::{eyre, Result};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use tempfile::{tempdir, TempDir};

pub type TestFilePath = std::path::PathBuf;

#[macro_export]
macro_rules! should_pass {
    ( $( $path:expr),* ) => {

        let mut tc = $crate::TestCases::new();

        $(
            tc = tc.should_pass($path);
        )*

        let runner = tc.into_runner().expect("[trycompile::should_pass!] Unable to create test runner");
        assert!(runner.run_test_cases().is_ok());
    };
}

#[macro_export]
macro_rules! should_fail {
    ( $( $path:expr),* ) => {

        let mut tc = $crate::TestCases::new();

        $(
            tc = tc.should_fail($path);
        )*

        let runner = tc.into_runner().expect("[trycompile::should_fail!] Unable to create test runner");
        assert!(runner.run_test_cases().is_ok());
    };
}

pub struct TestCases {
    pub(crate) queue: Vec<TestCase>,
}

impl Default for TestCases {
    fn default() -> Self {
        TestCases {
            queue: Vec::with_capacity(4096),
        }
    }
}

impl TestCases {
    pub fn new() -> TestCases {
        TestCases::default()
    }

    pub fn should_pass<P: AsRef<std::path::Path>>(self, path: P) -> TestCases {
        self.push(path, true)
    }

    pub fn should_fail<P: AsRef<std::path::Path>>(self, path: P) -> TestCases {
        self.push(path, false)
    }

    fn push<P: AsRef<std::path::Path>>(mut self, path: P, should_compile: bool) -> TestCases {
        self.queue.push(TestCase {
            should_compile,
            test_file: path.as_ref().to_path_buf(),
        });

        self
    }

    pub fn into_runner(self) -> Result<impl Runner> {
        let cargo_file_dir = std::env::current_dir()?;

        Ok(SeqRunner {
            cargo_file_dir,
            check_compile: self,
        })
    }
}

struct TestCase {
    should_compile: bool,
    test_file: TestFilePath,
}

#[derive(Clone, Debug)]
pub struct Status {
    on_test_file: PathBuf,
    compile_pass_expected: bool,
    compile_pass_result: bool,
    is_expected: IsExpected,
    stdout: String,
    stderr: String,
}

impl Status {
    pub fn report(&self) -> String {
        let stdout = if !self.stdout.is_empty() {
            format!("\n.. stdout:\n{}", self.stdout)
        } else {
            String::new()
        };

        let stderr = if !self.stderr.is_empty() {
            format!("\n.. stderr:\n{}", self.stderr)
        } else {
            String::new()
        };

        format!(
            "TestCase ({}) {} with:{}{}\n.. TestCase [should pass = {}, actual = {}]\n",
            self.on_test_file.display(),
            self.is_expected.as_success_message(),
            stdout,
            stderr,
            self.compile_pass_expected,
            self.compile_pass_result,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IsExpected {
    True,
    False,
}

impl IsExpected {
    fn from_exit_status(exit_status: ExitStatus, expected: bool) -> IsExpected {
        if !(exit_status.success() ^ expected) {
            IsExpected::True
        } else {
            IsExpected::False
        }
    }

    fn as_success_message(&self) -> &str {
        match self {
            IsExpected::True => "succeeds",
            IsExpected::False => "failed",
        }
    }

    fn passed(&self) -> bool {
        self == &IsExpected::True
    }
}

pub trait Runner {
    fn run_test_cases(&self) -> Result<()>;
}

struct SeqRunner {
    cargo_file_dir: PathBuf,
    check_compile: TestCases,
}

impl Runner for SeqRunner {
    #[cfg(windows)]
    fn run_test_cases(&self) -> Result<()> {
        eprintln!("[warn(trycompile)] Windows is currently not supported by this prototype... No actual tests will be run, but the runner will succeed...");

        Ok(())
    }

    #[cfg(not(windows))]
    fn run_test_cases(&self) -> Result<()> {
        // this should probably be rustc, but to make it easy for ourselves, we use cargo instead,
        // as this is a prototype 🥰

        let deps = crate_under_test_as_dep(&self.cargo_file_dir)?;
        let root = tempdir()?;
        let root = make_crate_with(root, &deps)?;

        let results: Vec<Status> = self
            .check_compile
            .queue
            .iter()
            .map(|tc| {
                // update lib.rs to current temp contents
                let test_case_path = tc.test_file.canonicalize()?;
                write_lib_rs_contents(&root.path().join("src"), &test_case_path)?;

                Ok(run_test_case(root.path(), tc)?)
            })
            .collect::<Result<Vec<Status>>>()?;

        // keep the temp dir in scope til the end of the test phase
        let _ = root;

        // this is dumb, but for now it will do
        let option = results.iter().fold(String::new(), |mut acc: String, next| {
            if !next.is_expected.passed() {
                acc.push_str(&next.report());
                acc
            } else {
                acc
            }
        });

        // ... great use of the type system <3
        if option == "" {
            Ok(())
        } else {
            eprintln!("{}", option);
            Err(eyre!("{}", option))
        }
    }
}

// importantly, we use cargo test --no-run, as to enable the test environment
// since sif generates #[cfg(test)] on a module, this is sometimes necessary
fn run_test_case(crate_root: &Path, tc: &TestCase) -> Result<Status> {
    let output = Command::new("cargo")
        .current_dir(crate_root)
        .env("RUSTFLAGS", "-Awarnings")
        .args(&["test", "--no-run"])
        .output()?;

    Ok(Status {
        on_test_file: tc.test_file.clone(),
        compile_pass_expected: tc.should_compile,
        compile_pass_result: output.status.success(),
        is_expected: IsExpected::from_exit_status(output.status, tc.should_compile),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

fn make_crate_with(crate_root: TempDir, dep_contents: &str) -> Result<TempDir> {
    let minimals = include_bytes!("../minimalcrate/Cargo.toml");

    std::fs::create_dir_all(crate_root.path())?;

    let new_cargofile = crate_root.path().join("Cargo.toml");

    let mut file = File::create(&new_cargofile)?;
    file.write_all(minimals)?;

    // add [dependencies]
    // assume for this prototype we don't need dev-dependencies etc
    file.write_all(dep_contents.as_bytes())?;
    file.flush()?;

    // create src dir
    let src = crate_root.path().join("src");
    std::fs::create_dir(&src)?;

    // return temp dir to keep it in scope
    Ok(crate_root)
}

fn write_lib_rs_contents(src: &Path, test_case_path: &Path) -> Result<()> {
    let test_case_content = std::fs::read(test_case_path)?;
    let _ = std::fs::write(src.join("lib.rs"), &test_case_content)?;

    Ok(())
}

// input = root of our project under testing
fn crate_under_test_as_dep<P: AsRef<Path>>(input: P) -> Result<String> {
    let cargo_file = std::fs::read_to_string(input.as_ref().join("Cargo.toml"))?;

    let value = cargo_file.parse::<toml::Value>()?;
    let dep = value["package"]["name"].to_string().replace('"', "");
    let dep_path = input
        .as_ref()
        .to_str()
        .ok_or_else(|| eyre!("Unable to convert path to utf-8"))?;

    let write = &[dep.as_str(), "= { path = \"", dep_path, "\"}"].join("");

    Ok(write.to_string())
}
