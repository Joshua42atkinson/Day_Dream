# Gemma 3 Model Download Guide

## Server-Side Model (Required First)

### Gemma 3 27B-IT Q4_0 (128K Context)

```bash
cd frontend/public/models

# Download 27B model (21GB)
curl -L -o gemma-3-27b-it-Q4_0.gguf \
  https://huggingface.co/google/gemma-3-27b-it-GGUF/resolve/main/gemma-3-27b-it-Q4_0.gguf

# Download tokenizer (shared across all Gemma 3 models)
curl -L -o tokenizer.json \
  https://huggingface.co/google/gemma-3-27b-it/resolve/main/tokenizer.json
```

**Verification:**

```bash
ls -lh gemma-3-27b-it-Q4_0.gguf  # Should be ~21GB
ls -lh tokenizer.json             # Should be ~4MB
```

---

## Client-Side Model (Phase 3)

### Gemma 3 270M-IT Q4_0 (32K Context, Text-Only)

```bash
cd frontend/public/models

# Download 270M model (240MB)
curl -L -o gemma-3-270m-it-Q4_0.gguf \
  https://huggingface.co/google/gemma-3-270m-it-GGUF/resolve/main/gemma-3-270m-it-Q4_0.gguf
```

**Verification:**

```bash
ls -lh gemma-3-270m-it-Q4_0.gguf  # Should be ~240MB
```

---

## Alternative: Using Hugging Face CLI

If you have `huggingface-cli` installed:

```bash
# Install if needed
pip install huggingface-hub

# Download 27B (server)
huggingface-cli download google/gemma-3-27b-it-GGUF \
  gemma-3-27b-it-Q4_0.gguf \
  --local-dir frontend/public/models

# Download 270M (client)
huggingface-cli download google/gemma-3-270m-it-GGUF \
  gemma-3-270m-it-Q4_0.gguf \
  --local-dir frontend/public/models

# Download tokenizer
huggingface-cli download google/gemma-3-27b-it \
  tokenizer.json \
  --local-dir frontend/public/models
```

---

## Server Startup Log (Expected)

When backend starts successfully:

```
Starting Ask Pete Backend Server...
Initializing Gemma 27B with 131072 context (~32.0GB KV cache)
Loading Gemma 27B from: frontend/public/models/gemma-3-27b-it-Q4_0.gguf
Max context length: 131072 tokens
Gemma 27B loaded successfully.
AI Mirror Socratic Engine initialized and connected to Gemma 27B
Backend listening on http://127.0.0.1:3000
```

---

## Troubleshooting

**Error: "Failed to open model file"**

- Verify file exists: `ls frontend/public/models/gemma-3-27b-it-Q4_0.gguf`
- Check file size matches (~21GB)
- Ensure no partial downloads (re-download if needed)

**Error: "Failed to load tokenizer"**

- Verify `tokenizer.json` exists in same directory
- Check file is not corrupted (should be ~4MB)

**Warning: "KV cache may exceed available memory"**

- This is expected if context > 32K on systems with <40GB RAM
- On 128GB server, this warning should not appear
- If it does, check `max_context_length` parameter

---

## Next Steps After Download

1. **Test server startup:** `cd backend && cargo run`
2. **Verify 128K context:** Check startup logs for "131072 tokens"
3. **Test RAG endpoint:** POST to `/api/knowledge/search` (after DB setup)
4. **Phase 3:** Download 270M model for client deployment
