# 🤖 AIman

**AIman** is a simple, fast, and lightweight manager (written in Rust 🦀) that lets you download, organize, and run Artificial Intelligence models (`.gguf`) directly on your computer — with complete privacy and no subscriptions.

You don't need to memorize complex terminal commands: **AIman** works through an **interactive guided menu** that you navigate using your keyboard arrow keys (`↑` / `↓` and `Enter`).

---

## ✨ What you can do with AIman

* **💬 Run and chat with your AI models:** pick an installed model and start chatting right from your terminal.
* **🛠️ Agentic AI Mode:** run your local model as an autonomous assistant capable of executing shell commands on your system (with your explicit approval before every command).
* **📥 Download models from Hugging Face:** enter a Hugging Face repository and quantization (e.g. `Q4_K_M`) to download any `.gguf` model straight into your local library.
* **⚙️ Choose your engine (Server):** easily install and switch between prebuilt inference engines (`llama.cpp`, `ik_llama.cpp`, `mistralrs`) for Linux, Windows, or macOS (CPU, CUDA, Metal).
* **🗑️ Manage disk space:** uninstall models or servers you no longer use in just one step.
* **🔒 100% Local & Private:** all models, agent tasks, and conversations run locally on your PC; no data is ever sent to external cloud AI providers.

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

1. **Start an AI model** — Pick one of your downloaded models and start chatting or launch its HTTP server.
2. **Install an AI model** — Download a new `.gguf` model from Hugging Face.
3. **Uninstall an AI model** — Delete a model and its configuration to free up disk space.
4. **Install a server** — Download a prebuilt engine required to run AI models (`llama-cli`, `llama-server`, `ik-llama-cli`, `ik-llama-server`, or `mistralrs`).
5. **Uninstall a server** — Remove an installed server.
6. **Start an Agent AI** — Launch a model in Agentic Mode so it can inspect your system and execute shell commands on your behalf.
7. **Exit** — Close the program.

---

### 3. Your First Run in 3 Steps
If this is your first time using AIman:
1. Select **`Install a server`** to download a runtime engine (you only need to do this once).
2. Select **`Install an AI model`** and download your preferred `.gguf` model from Hugging Face.
3. Select **`Start an AI model`** (or **`Start an Agent AI`**), pick the model you just downloaded, and get started!

---

## 🛠️ Agentic AI Mode

AIman includes a built-in **Agentic Mode** powered by **`llama-cli`** and **`ik-llama-cli`**:

* **Real-Time Token Streaming:** watch the agent think and respond token-by-token directly in your terminal.
* **Interactive Safety Confirmation:** whenever the AI wants to run a shell command, AIman pauses and asks you **`Execute '<command>'?` (`Yes` / `No`)** using the arrow keys — nothing runs without your permission.
* **Live Progress Indicator:** while an approved command is running, AIman displays an animated `working...` spinner.
* **Automatic Feedback Loop:** the command's output is displayed to you and automatically sent back to the model so it can analyze the result and complete your task.

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
