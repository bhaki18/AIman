# 🤖 AIman

**AIman** is a simple, fast, and lightweight manager (written in Rust 🦀) that lets you download, organize, and run Artificial Intelligence models (`.gguf`) directly on your computer — with complete privacy and no subscriptions.

You don't need to memorize complex terminal commands: **AIman** works through an **interactive guided menu** that you navigate using your keyboard arrow keys (`↑` / `↓` and `Enter`).

---

## ✨ What you can do with AIman

* **💬 Run and chat with your AI models:** pick an installed model and start chatting right from your terminal.
* **📥 Download models from Hugging Face:** paste the link to any `.gguf` model from Hugging Face or use the built-in integration with a real-time progress bar.
* **⚙️ Choose your engine (Server):** easily install and switch between different inference engines (such as `mistralrs`, `llama.cpp`, `ik_llama.cpp`) depending on your hardware (CPU or GPU).
* **🗑️ Manage disk space:** uninstall models or servers you no longer use in just one step.
* **🔒 100% Local & Private:** all models and conversations run on your PC; no data is ever sent to external servers.

---

## 🚀 Getting Started (Quick Guide)

### 1. Launch AIman
Download **AIman**, build and run it from your terminal with:

```bash
cargo run
```

On first launch, AIman will automatically create its working directory in your Home folder (`~/.AIman`).

---

### 2. Use the Interactive Menu
Navigate with the **`↑` and `↓` arrow keys** and press **`Enter`** to choose an action:

1. **Start an AI model** — Pick one of your downloaded models and start using it right away.
2. **Install an AI model** — Download a new `.gguf` model from Hugging Face.
3. **Uninstall an AI model** — Delete a model to free up disk space.
4. **Install a server** — Download the engine required to run AI models (e.g., `mistralrs` or `llama.cpp`).
5. **Uninstall a server** — Remove an installed server.
6. **Exit** — Close the program.

---

### 3. Your First Run in 3 Steps
If this is your first time using AIman:
1. Select **`Install a server`** to download a runtime engine (you only need to do this once).
2. Select **`Install an AI model`** and download your preferred `.gguf` model from Hugging Face.
3. Select **`Start an AI model`**, pick the model you just downloaded, and start chatting!

---

## 📂 Where are files stored?

AIman keeps everything neatly organized inside a single hidden folder in your Home directory (`~/.AIman`), without cluttering your system:

* **`~/.AIman/models/`** — Stores the downloaded `.gguf` AI model files.
* **`~/.AIman/servers/`** — Stores the installed engines used to run the models.
* **`~/.AIman/models_config/`** — Stores saved preferences for each model (so AIman remembers which server to use).

---

## 📜 License & Author

Created by **Angelo Del Piano** ([@bhaki18](https://github.com/bhaki18)).  
Distributed under the free **GNU General Public License v3.0 (GPL-3.0)** — see the [LICENSE](./LICENSE) file for more details.
