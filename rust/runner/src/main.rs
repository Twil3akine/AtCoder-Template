use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use tempfile::tempdir;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::timeout;
use tower_http::cors::{Any, CorsLayer};

// ── リクエスト/レスポンス型 ────────────────────────────────────────

#[derive(Deserialize)]
#[serde(tag = "mode", rename_all = "lowercase")]
enum Request {
    List,
    Run {
        #[serde(rename = "compilerName")]
        compiler_name: String,
        #[serde(rename = "sourceCode")]
        source_code: String,
        stdin: String,
    },
}

#[derive(Serialize)]
struct CompilerInfo {
    language: String,
    #[serde(rename = "compilerName")]
    compiler_name: String,
    label: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RunResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    exit_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stderr: Option<String>,
}

// ── バージョン検出 ─────────────────────────────────────────────────

#[derive(Clone)]
struct Versions {
    rust: String,
    python: String,
    pypy: String,
}

async fn detect_version(cmd: &str, args: &[&str]) -> String {
    let Ok(out) = Command::new(cmd).args(args).output().await else {
        return "?".into();
    };
    let raw = if out.stdout.is_empty() {
        String::from_utf8_lossy(&out.stderr).into_owned()
    } else {
        String::from_utf8_lossy(&out.stdout).into_owned()
    };
    let first = raw.lines().next().unwrap_or("?").trim().to_string();
    // "rustc 1.82.0 (...)" → "1.82.0" / "Python 3.13.1" → "3.13.1"
    first.split_whitespace().nth(1).unwrap_or(&first).to_string()
}

impl Versions {
    async fn detect() -> Self {
        Self {
            rust: detect_version("rustc", &["--version"]).await,
            python: detect_version("python3", &["--version"]).await,
            pypy: detect_version("pypy3", &["--version"]).await,
        }
    }
}

// ── ハンドラ ───────────────────────────────────────────────────────

async fn handle(
    axum::extract::State(versions): axum::extract::State<Arc<Versions>>,
    Json(req): Json<Request>,
) -> Json<serde_json::Value> {
    match req {
        Request::List => {
            let list: Vec<CompilerInfo> = vec![
                CompilerInfo {
                    language: "rust".into(),
                    compiler_name: "rust".into(),
                    label: format!("Rust ({})", versions.rust),
                },
                CompilerInfo {
                    language: "python".into(),
                    compiler_name: "python".into(),
                    label: format!("Python (CPython {})", versions.python),
                },
                CompilerInfo {
                    language: "pypy".into(),
                    compiler_name: "pypy".into(),
                    label: format!("Python (PyPy {})", versions.pypy),
                },
            ];
            Json(serde_json::to_value(list).unwrap())
        }
        Request::Run { compiler_name, source_code, stdin } => {
            let res = run(&compiler_name, &source_code, &stdin).await;
            Json(serde_json::to_value(res).unwrap())
        }
    }
}

// ── コンパイル + 実行 ──────────────────────────────────────────────

const COMPILE_TIMEOUT: Duration = Duration::from_secs(30);
const RUN_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_OUTPUT: usize = 1 * 1024 * 1024; // 1 MiB

async fn run(compiler_name: &str, source_code: &str, stdin: &str) -> RunResponse {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(e) => return internal_error(format!("tempdir: {e}")),
    };

    match compiler_name {
        "rust" => run_rust(source_code, stdin, dir).await,
        "python" => run_interpreted("python3", source_code, stdin, dir).await,
        "pypy" => run_interpreted("pypy3", source_code, stdin, dir).await,
        other => internal_error(format!("unknown compilerName: {other}")),
    }
}

async fn run_rust(source_code: &str, stdin: &str, dir: tempfile::TempDir) -> RunResponse {
    let src = dir.path().join("solution.rs");
    let bin = dir.path().join("solution");

    if let Err(e) = tokio::fs::write(&src, source_code).await {
        return internal_error(format!("write source: {e}"));
    }

    // コンパイル
    let compile_result = timeout(
        COMPILE_TIMEOUT,
        Command::new("rustc")
            .args([src.to_str().unwrap(), "-o", bin.to_str().unwrap(), "-C", "opt-level=2"])
            .output(),
    )
    .await;

    match compile_result {
        Err(_) => return compile_error("コンパイルがタイムアウトしました"),
        Ok(Err(e)) => return internal_error(format!("rustc 起動失敗: {e}")),
        Ok(Ok(out)) if !out.status.success() => {
            return compile_error(String::from_utf8_lossy(&out.stderr).trim());
        }
        Ok(Ok(_)) => {}
    }

    execute(&bin.to_string_lossy(), &[], stdin).await
}

async fn run_interpreted(
    interpreter: &str,
    source_code: &str,
    stdin: &str,
    dir: tempfile::TempDir,
) -> RunResponse {
    let src = dir.path().join("solution.py");

    if let Err(e) = tokio::fs::write(&src, source_code).await {
        return internal_error(format!("write source: {e}"));
    }

    // 構文チェック
    let check = timeout(
        Duration::from_secs(10),
        Command::new(interpreter)
            .args(["-m", "py_compile", src.to_str().unwrap()])
            .output(),
    )
    .await;

    match check {
        Err(_) => return compile_error("構文チェックがタイムアウトしました"),
        Ok(Err(e)) => return internal_error(format!("{interpreter} 起動失敗: {e}")),
        Ok(Ok(out)) if !out.status.success() => {
            return compile_error(String::from_utf8_lossy(&out.stderr).trim());
        }
        Ok(Ok(_)) => {}
    }

    execute(interpreter, &[src.to_str().unwrap()], stdin).await
}

async fn execute(cmd: &str, args: &[&str], stdin_data: &str) -> RunResponse {
    let mut child = match Command::new(cmd)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => return internal_error(format!("実行ファイル起動失敗: {e}")),
    };

    // stdin を書き込んで閉じる
    if let Some(mut si) = child.stdin.take() {
        let _ = si.write_all(stdin_data.as_bytes()).await;
    }

    let start = Instant::now();
    let result = timeout(RUN_TIMEOUT, child.wait_with_output()).await;

    match result {
        Err(_) => RunResponse {
            status: "success".into(),
            exit_code: None,
            time: Some(RUN_TIMEOUT.as_millis() as u64),
            memory: None,
            stdout: None,
            stderr: Some("TLE: 実行時間制限 (10s) を超えました".into()),
        },
        Ok(Err(e)) => internal_error(format!("wait失敗: {e}")),
        Ok(Ok(out)) => {
            let elapsed_ms = start.elapsed().as_millis() as u64;
            let stdout = truncate(String::from_utf8_lossy(&out.stdout).into_owned());
            let stderr = truncate(String::from_utf8_lossy(&out.stderr).into_owned());
            RunResponse {
                status: "success".into(),
                exit_code: out.status.code(),
                time: Some(elapsed_ms),
                memory: None,
                stdout: Some(stdout),
                stderr: Some(stderr),
            }
        }
    }
}

fn truncate(s: String) -> String {
    if s.len() > MAX_OUTPUT {
        format!("{}...(出力が長すぎるため切り捨て)", &s[..MAX_OUTPUT])
    } else {
        s
    }
}

fn compile_error(msg: impl Into<String>) -> RunResponse {
    RunResponse {
        status: "compileError".into(),
        exit_code: None,
        time: None,
        memory: None,
        stdout: None,
        stderr: Some(msg.into()),
    }
}

fn internal_error(msg: impl Into<String>) -> RunResponse {
    RunResponse {
        status: "internalError".into(),
        exit_code: None,
        time: None,
        memory: None,
        stdout: None,
        stderr: Some(msg.into()),
    }
}

// ── main ───────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let versions = Arc::new(Versions::detect().await);
    println!("Local Runner");
    println!("  Rust   : {}", versions.rust);
    println!("  CPython: {}", versions.python);
    println!("  PyPy   : {}", versions.pypy);

    let port: u16 = std::env::var("RUNNER_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(4000);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_private_network(true);

    let app = Router::new()
        .route("/", post(handle))
        .layer(cors)
        .with_state(versions);

    let addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
