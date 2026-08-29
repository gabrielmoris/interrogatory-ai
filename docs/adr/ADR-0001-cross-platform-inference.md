# ADR-0001 — Cross-platform local inference (Windows + Android)

**Status:** Accepted
**Date:** 2026-08-21
**Context:** Interrogator must run local-first on Windows (dev machine, NVIDIA 8 GB+) and on Android.

---

## Decision

**One inference backend — llama.cpp via `llama-cpp-2` — compiled twice with different feature flags.** Windows builds with CUDA; Android builds for `aarch64-linux-android` via the NDK, with Vulkan (or OpenCL/Adreno) where the device supports it and CPU/NEON as the floor.

`llama-cpp-sys-2`'s build script has first-class Android support: it detects `aarch64-linux-android` / `armv7-linux-androideabi`, reads `ANDROID_NDK` / `ANDROID_NDK_ROOT` / `NDK_ROOT`, emits per-ABI flags for arm64-v8a, armeabi-v7a, x86_64 and x86, handles `c++_static` vs `c++_shared` linking, and supports `GGML_VULKAN` and `GGML_OPENCL` (Qualcomm Adreno). CUDA and Metal are not configured for Android, as expected.

This means **the `InferenceEngine` trait already defined in Phase 1 is the entire portability story for the code.** No second engine, no JNI bridge, no Kotlin.

---

## What actually differs between platforms

Not the code. The **model**, and the **memory budget**.

| | Windows | Android |
|---|---|---|
| Compute | CUDA, full offload | CPU/NEON floor; Vulkan or OpenCL where available |
| Practical model | ~8B instruct, Q4_K_M (~4.7 GB) | 1–4B instruct, Q4_K_M (~0.7–2.5 GB) |
| Resident budget | VRAM + host RAM, generous | ~1.5–2.5 GB before the OS starts killing you |
| Process lifetime | until you close it | terminated at the OS's discretion when backgrounded |
| Prefill cost | negligible | the real bottleneck on long transcripts |
| Thermals | irrelevant | throttles within minutes of sustained generation |

Two different models means two different chat templates, stop tokens, context sizes and offload settings. Therefore:

> **`ModelProfile` must be data, not code — from day one.**

```rust
pub struct ModelProfile {
    pub id: String,
    pub file: String,            // e.g. "qwen3-1.7b-q4_k_m.gguf"
    pub sha256: String,
    pub chat_template: ChatTemplate,
    pub n_ctx: u32,
    pub n_gpu_layers: i32,       // -1 desktop, 0..n mobile
    pub n_threads: Option<u32>,  // None => big-cores only, resolved at runtime
    pub stop: Vec<String>,
    pub max_tokens: u32,         // lower on mobile
}
```

Ship one profile per platform as a TOML file next to the case files. No domain module may hardcode a template, a context size, or a token budget.

---

## The four Android-specific problems

These are the actual work. The trait is the easy part.

### 1. Model delivery
You cannot ship a multi-gigabyte GGUF inside the app bundle — Google Play's base download limit is far below it, and asset packs are both capped and awkward. **Download on first run** into app-private storage.

This is a real feature, not a detail:

```rust
enum ModelState {
    Absent,
    Downloading { received: u64, total: u64 },
    Verifying,
    Ready(PathBuf),
    Corrupt(String),
}
```

Requirements: resumable (HTTP range requests), SHA-256 verified before first load, cancellable, and a UI state for each variant. Stream to disk with `reqwest` — never buffer 2 GB in memory. Same code path works on Windows, so build it once and use it on both.

### 2. Memory and process lifecycle
- Let llama.cpp **mmap** the GGUF (its default). Mapped pages are evictable under pressure; a heap copy is not.
- **Drop the `LlamaContext` when the app backgrounds** and reload on resume. The KV cache is the expensive resident allocation, and Android's low-memory killer does not negotiate.
- Persist the transcript to disk on every turn, so a kill mid-interrogation is recoverable. On desktop this is a nicety; on Android it is required.
- Tauri exposes no lifecycle hook for this directly — you will write a small Android plugin (or hook webview visibility) to get pause/resume into Rust.

### 3. Threads and thermals
- `n_threads` = the number of **performance** cores, not `num_cpus::get()`. Saturating little cores makes it slower and hotter.
- Cap `max_tokens` more aggressively than on desktop. A 400-token answer on a phone is a thermal event.
- Prefill dominates. Every extra turn of transcript you resend costs real seconds. This is why §5 below matters.

### 4. Build toolchain
`cargo-ndk`, `ANDROID_NDK_ROOT`, `tauri android init` / `dev` / `build`, Android SDK + NDK version pinning, C++ stdlib link mode. **Budget a full session for this and schedule no feature work alongside it** — same rule as the Windows/CUDA build. Record the exact working versions in `docs/BUILD.md`.

---

## 5. The design consequence: the model becomes a renderer, not the game master

A 1B model cannot hold a deceptive persona with gated knowledge across twenty turns. If the game depends on the model's judgement, the Android build will be broken and no amount of prompt tuning will fix it.

The fix is the design you already committed to in ADR terms — push it further:

- **Rust decides, the model speaks.** The interrogation state machine decides pressure level, whether a contradiction was caught, and which fact (if any) the suspect releases this turn. That decision is injected as an explicit instruction (`"You may now admit fact F7, reluctantly."`). The model's only job is to render dialogue in character.
- **Constrained decoding for anything structured.** llama.cpp supports GBNF grammars — use them for the scoring extraction pass so even a 1B model emits parseable output. Do not parse free-form JSON from a small model.
- **Aggressive transcript summarization.** Keep system prompt + last N turns + a Rust-maintained running summary. Bounds prefill cost, which is the mobile bottleneck.
- **Tier-1 scoring stays deterministic.** It already is. That is precisely why the score survives a weaker model — only the narrative critique degrades.

This makes the desktop build better too. It is not a mobile concession.

---

## 6. Escape hatch: `RemoteEngine`

Add a third `InferenceEngine` implementation that speaks HTTP to a `llama-server` (your desktop, or a cloud endpoint). Cost: roughly one afternoon, because the trait already exists.

Use it for: low-RAM devices, iterating on mobile UI without waiting for NDK builds, and end-to-end tests. **Do not let it become the default on Android** — the moment it is, the app is no longer local-first and the project's premise is gone.

Engine selection at runtime, not compile time:

```rust
fn engine_for(cfg: &Config) -> Box<dyn InferenceEngine> { /* Mock | Llama | Remote */ }
```

---

## 7. Sequencing

**Android is not a Phase 2 concern.** Phase 2 stays Windows/CUDA only. Fighting two toolchains before the game works is how this project dies.

Insert **Phase 2.5 — Android bring-up** *after* Phase 3 (case engine and scoring are stable):

1. `tauri android init`, get the existing app running on a device with `MockEngine`. No inference. Proves the UI, the IPC and the toolchain.
2. Build `llama-cpp-2` for `aarch64-linux-android`, CPU only. Prove tokens come out at all.
3. Model download + verification flow.
4. Lifecycle handling (pause/resume, context drop, transcript persistence).
5. Vulkan/OpenCL offload — last, and only if CPU is too slow. It is an optimization, not a requirement.

**What to do *now*, at zero cost, so Phase 2.5 is possible:**

- Domain modules never touch `std::fs`. Paths are passed in; Tauri resolves them per platform. *(Amended 2026-08-21: `crates/core` here means the domain modules in `src-tauri/src` — the workspace split was rejected, see `DECISIONS.md`.)*
- `ModelProfile` exists as data from the first commit that loads a model.
- No hardcoded chat template, context size, thread count, or token cap anywhere.
- `InferenceEngine` returns a stream and takes a cancellation token — already the plan, and non-negotiable once mobile is in scope.

---

## Alternatives considered

| Option | Why rejected |
|---|---|
| **MediaPipe / Google AI Edge LiteRT** | Kotlin + JNI, a second engine to maintain, no desktop story. Good mobile performance, but you would be writing and debugging two inference paths in a project whose point is learning Rust. |
| **ONNX Runtime GenAI** | Cross-platform, but weaker Rust bindings and a more painful model conversion pipeline than GGUF. |
| **Gemini Nano / AICore** | Device-gated, no control over sampling parameters or system prompt depth, and nothing equivalent on Windows. Fails the "one engine" requirement outright. |
| **`candle`** | Pure Rust and pleasant to build, but slower, thinner quantization support, and no meaningful mobile GPU story. |
| **Remote-only on Android** | Abandons local-first. Kept as a fallback tier, never as the default. |

---

## Expected performance

On a current flagship SoC, a 1–3B Q4 model is conversational — usable for a turn-based interrogation game where the player is reading, not waiting on a cursor. Midrange devices are materially slower, and prefill on a long transcript is what the player will feel, not token generation. Budget accordingly: summarize hard, cap output length, stream every token so time-to-first-token is what the player perceives.

Verify on real hardware before you tune anything. Emulator numbers are meaningless.

## Sources

- [`llama-cpp-sys-2` build.rs](https://docs.rs/crate/llama-cpp-sys-2/latest/source/build.rs) — Android/NDK detection, ABI flags, Vulkan and OpenCL backends
- [`llama-cpp-2` crate](https://crates.io/crates/llama-cpp-2)
- [cargo-ndk](https://docs.rs/crate/cargo-ndk/0.6.2)
- [llama.cpp — Android performance discussion](https://github.com/ggml-org/llama.cpp/discussions/14356)
- [llama.cpp — Vulkan performance discussion](https://github.com/ggml-org/llama.cpp/discussions/10879)
- [Google Play app size limits](https://support.google.com/googleplay/android-developer/answer/9859372?hl=en)
