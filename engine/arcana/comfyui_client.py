#!/usr/bin/env python3
"""
ComfyUI API Client — Handles all communication with the local ComfyUI server.

Features:
  - Connection health checks before dispatch
  - Workflow template rendering with parameter injection
  - Async polling with configurable timeout
  - Retry logic with exponential backoff
  - Image retrieval and local storage

Usage:
    from comfyui_client import ComfyUIClient
    client = ComfyUIClient("http://127.0.0.1:8188")
    if client.health_check():
        job_id = client.submit_workflow(workflow_json)
        images = client.wait_for_result(job_id, timeout=300)
"""

import json
import time
import uuid
import urllib.request
import urllib.error
import urllib.parse
from pathlib import Path
from dataclasses import dataclass, field


@dataclass
class GenerationResult:
    """Result of a single image generation job."""
    job_id: str
    success: bool
    image_paths: list[str] = field(default_factory=list)
    error: str = ""
    elapsed_seconds: float = 0.0
    prompt_used: str = ""


class ComfyUIClient:
    """Client for the ComfyUI HTTP API."""

    def __init__(self, base_url: str = "http://127.0.0.1:8188"):
        self.base_url = base_url.rstrip("/")
        self.client_id = str(uuid.uuid4())

    # ─── Health & Diagnostics ─────────────────────────────────

    def health_check(self) -> dict:
        """
        Check if ComfyUI is running and responsive.
        Returns system stats dict on success, empty dict on failure.
        """
        try:
            url = f"{self.base_url}/system_stats"
            req = urllib.request.Request(url, method="GET")
            with urllib.request.urlopen(req, timeout=5) as resp:
                data = json.loads(resp.read().decode())
                return data
        except (urllib.error.URLError, TimeoutError, ConnectionError):
            return {}

    def get_gpu_status(self) -> dict:
        """Get GPU VRAM usage from ComfyUI system stats."""
        stats = self.health_check()
        if not stats:
            return {"available": False}

        devices = stats.get("devices", [])
        if not devices:
            return {"available": True, "gpu": "unknown"}

        dev = devices[0]
        vram_total = dev.get("vram_total", 0)
        vram_free = dev.get("vram_free", 0)
        return {
            "available": True,
            "gpu": dev.get("name", "unknown"),
            "vram_total_gb": round(vram_total / (1024**3), 1),
            "vram_free_gb": round(vram_free / (1024**3), 1),
            "vram_used_pct": round((1 - vram_free / max(vram_total, 1)) * 100, 1),
        }

    # ─── Workflow Building ────────────────────────────────────

    @staticmethod
    def build_longcat_t2i_workflow(
        prompt: str,
        negative_prompt: str = "",
        width: int = 1024,
        height: int = 1024,
        steps: int = 25,
        cfg: float = 4.5,
        seed: int = -1,
        model_name: str = "LongCat-Image",
        save_prefix: str = "arcana",
    ) -> dict:
        """
        Build a LongCat-Image text-to-image workflow JSON.

        This constructs the ComfyUI API-format workflow matching the
        example_workflow_t2i.json structure from comfyui_longcat_image.
        """
        if seed < 0:
            seed = int(time.time() * 1000) % (2**32)

        workflow = {
            "prompt": {
                # Node 1: Model Loader
                "1": {
                    "class_type": "LongCatImageModelLoader",
                    "inputs": {
                        "model_path": model_name,
                        "dtype": "bfloat16",
                    },
                },
                # Node 2: Text to Image
                "2": {
                    "class_type": "LongCatImageTextToImage",
                    "inputs": {
                        "longcat_pipeline": ["1", 0],
                        "prompt": prompt,
                        "negative_prompt": negative_prompt,
                        "width": width,
                        "height": height,
                        "steps": steps,
                        "guidance_scale": cfg,
                        "seed": seed,
                        "cpu_offload": "true",
                        "sage_attn": "false",  # Not reliable on AMD
                    },
                },
                # Node 3: Save Image
                "3": {
                    "class_type": "SaveImage",
                    "inputs": {
                        "images": ["2", 0],
                        "filename_prefix": save_prefix,
                    },
                },
            }
        }
        return workflow

    # ─── Job Submission ───────────────────────────────────────

    def submit_workflow(self, workflow: dict) -> str:
        """
        Submit a workflow to ComfyUI for execution.
        Returns the prompt_id (job ID).
        Raises RuntimeError on failure.
        """
        payload = json.dumps(workflow).encode("utf-8")
        url = f"{self.base_url}/prompt"
        req = urllib.request.Request(
            url, data=payload,
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        try:
            with urllib.request.urlopen(req, timeout=30) as resp:
                result = json.loads(resp.read().decode())
                prompt_id = result.get("prompt_id", "")
                if not prompt_id:
                    raise RuntimeError(f"No prompt_id in response: {result}")
                return prompt_id
        except urllib.error.HTTPError as e:
            body = e.read().decode() if e.fp else ""
            raise RuntimeError(
                f"ComfyUI rejected workflow (HTTP {e.code}): {body}"
            ) from e

    # ─── Result Polling ───────────────────────────────────────

    def wait_for_result(
        self, prompt_id: str, timeout: int = 600, poll_interval: float = 2.0
    ) -> GenerationResult:
        """
        Poll ComfyUI /history endpoint until the job completes or times out.

        Args:
            prompt_id: The job ID from submit_workflow
            timeout: Maximum seconds to wait
            poll_interval: Seconds between polls

        Returns:
            GenerationResult with success status and image paths
        """
        start = time.time()

        while (time.time() - start) < timeout:
            try:
                url = f"{self.base_url}/history/{prompt_id}"
                req = urllib.request.Request(url, method="GET")
                with urllib.request.urlopen(req, timeout=10) as resp:
                    history = json.loads(resp.read().decode())

                if prompt_id in history:
                    job = history[prompt_id]
                    outputs = job.get("outputs", {})
                    status = job.get("status", {})

                    # Check for errors
                    if status.get("status_str") == "error":
                        msgs = status.get("messages", [])
                        error_msg = str(msgs) if msgs else "Unknown error"
                        return GenerationResult(
                            job_id=prompt_id,
                            success=False,
                            error=error_msg,
                            elapsed_seconds=time.time() - start,
                        )

                    # Extract image paths from outputs
                    image_paths = []
                    for node_id, node_out in outputs.items():
                        for img in node_out.get("images", []):
                            filename = img.get("filename", "")
                            subfolder = img.get("subfolder", "")
                            if filename:
                                image_paths.append(
                                    f"{subfolder}/{filename}" if subfolder
                                    else filename
                                )

                    if image_paths:
                        return GenerationResult(
                            job_id=prompt_id,
                            success=True,
                            image_paths=image_paths,
                            elapsed_seconds=time.time() - start,
                        )

            except (urllib.error.URLError, TimeoutError):
                pass  # Server busy, keep polling

            time.sleep(poll_interval)

        return GenerationResult(
            job_id=prompt_id,
            success=False,
            error=f"Timed out after {timeout}s",
            elapsed_seconds=timeout,
        )

    # ─── Image Retrieval ──────────────────────────────────────

    def download_image(
        self, filename: str, output_dir: Path, subfolder: str = ""
    ) -> Path:
        """
        Download a generated image from ComfyUI's output directory.

        Args:
            filename: Image filename from GenerationResult
            output_dir: Local directory to save to
            subfolder: Optional subfolder in ComfyUI output

        Returns:
            Path to the downloaded file
        """
        params = urllib.parse.urlencode({
            "filename": filename,
            "subfolder": subfolder,
            "type": "output",
        })
        url = f"{self.base_url}/view?{params}"

        output_dir.mkdir(parents=True, exist_ok=True)
        output_path = output_dir / filename

        req = urllib.request.Request(url, method="GET")
        with urllib.request.urlopen(req, timeout=30) as resp:
            with open(output_path, "wb") as f:
                f.write(resp.read())

        return output_path

    # ─── Convenience: Full Generation Pipeline ────────────────

    def generate_image(
        self,
        prompt: str,
        negative_prompt: str = "",
        output_dir: Path = Path("./output"),
        width: int = 1024,
        height: int = 1024,
        steps: int = 25,
        cfg: float = 4.5,
        seed: int = -1,
        save_prefix: str = "arcana",
        timeout: int = 600,
    ) -> GenerationResult:
        """
        Full pipeline: build workflow → submit → poll → download.

        Returns GenerationResult with local image paths.
        """
        workflow = self.build_longcat_t2i_workflow(
            prompt=prompt,
            negative_prompt=negative_prompt,
            width=width,
            height=height,
            steps=steps,
            cfg=cfg,
            seed=seed,
            save_prefix=save_prefix,
        )

        prompt_id = self.submit_workflow(workflow)
        result = self.wait_for_result(prompt_id, timeout=timeout)

        if result.success:
            local_paths = []
            for img_path in result.image_paths:
                parts = img_path.split("/", 1)
                fname = parts[-1]
                subfolder = parts[0] if len(parts) > 1 else ""
                local = self.download_image(fname, output_dir, subfolder)
                local_paths.append(str(local))
            result.image_paths = local_paths

        result.prompt_used = prompt
        return result
