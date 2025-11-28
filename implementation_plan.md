# Implementation Plan - Hybrid Sovereign Architecture

## Goal

Transform "Ask Pete" into a "Hybrid Sovereign" platform by integrating Google's Gemma 3 for local inference and Antigravity for enterprise-grade scaling, while implementing the "Coal" (Compute) & "Steam" (Mastery) economic model.

## User Review Required
>
> [!IMPORTANT]
> **Gemma 3 Integration**: We are replacing the generic Llama implementation with Gemma 3. This requires verifying that `candle-transformers` supports the specific Gemma 3 architecture (or using a compatible fallback like Gemma 2 until explicit support is confirmed).
> **Compute Token Scarcity**: We are introducing "Coal" as a consumable resource for AI inference. This is a significant gameplay change.

## Proposed Changes

### Phase 1: Gemma 3 Core (Local Sovereignty)

#### [MODIFY] [backend/Cargo.toml](file:///backend/Cargo.toml)

- Ensure `candle-transformers` and `hf-hub` are configured for Gemma 3.
- Add `burn` or specific `ort` features if needed for optimized inference.

#### [MODIFY] [backend/src/ai/llm/mod.rs](file:///backend/src/ai/llm/mod.rs)

- Refactor `LLMEngine` trait to support Gemma 3 specific tokenization and generation parameters.

#### [NEW] [backend/src/ai/llm/gemma_engine.rs](file:///backend/src/ai/llm/gemma_engine.rs)

- Implement `GemmaEngine` struct using `candle`.
- Handle model download (2B/7B quantized) via `hf-hub`.
- Implement "Coal" cost calculation per token.

#### [MODIFY] [frontend/src/ai/gemma_agent.rs](file:///frontend/src/ai/gemma_agent.rs)

- Implement WebGPU-based inference for Gemma 3 (270M/2B) in the browser.
- Use `wgpu` and `candle-wasm` for zero-latency, offline capability.

### Phase 2: Antigravity Bridge (Enterprise Scale)

#### [NEW] [backend/src/antigravity/mod.rs](file:///backend/src/antigravity/mod.rs)

- Create the `AntigravityClient` to communicate with the enterprise backend.
- Implement "Steam" synchronization (uploading verified progress vectors).

#### [MODIFY] [backend/src/handlers/weigh_station.rs](file:///backend/src/handlers/weigh_station.rs)

- Update `WeighStation` to act as the gateway between Local (Gemma) and Cloud (Antigravity).

## Verification Plan

### Automated Tests

- `cargo test` for `GemmaEngine` tokenization and generation.
- Unit tests for "Coal" deduction logic.

### Manual Verification

- **Local Inference**: Verify Gemma 3 runs on the student laptop (backend) and browser (frontend) without crashing.
- **Economic Loop**: Confirm that asking Pete a question consumes "Coal" and answering correctly generates "Steam".
- **Cloud Sync**: Verify that "Steam" generated locally appears in the Antigravity dashboard (mocked if necessary).
