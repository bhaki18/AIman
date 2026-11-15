# 🤖 AIman

**AIman** è un gestore semplice, veloce e leggero (scritto in Rust 🦀) che ti permette di scaricare, organizzare e usare modelli di Intelligenza Artificiale (`.gguf`) direttamente sul tuo computer, in totale privacy e senza abbonamenti.

Non devi ricordare comandi complicati da terminale: **AIman** funziona tramite un **menu interattivo guidato** navigabile con le frecce della tastiera (`↑` / `↓` e `Invio`).

---

## ✨ Cosa puoi fare con AIman

* **💬 Avvia e chatta con i tuoi modelli AI:** scegli un modello installato e inizia subito a conversare direttamente dal terminale.
* **📥 Scarica modelli da Hugging Face:** incolla il link di qualsiasi modello `.gguf` da Hugging Face o usa l'integrazione dedicata con barra di avanzamento in tempo reale.
* **⚙️ Scegli il tuo motore (Server):** installa e passa facilmente tra diversi motori di esecuzione (come `mistralrs`, `llama.cpp`, `ik_llama.cpp`) in base al tuo hardware (CPU o scheda video).
* **🗑️ Gestisci lo spazio su disco:** disinstalla con un click i modelli o i server che non usi più.
* **🔒 100% Locale e Privato:** tutti i modelli e le conversazioni girano sul tuo PC; nessun dato viene inviato a server esterni.

---

## 🚀 Come iniziare (Guida Rapida)

### 1. Avvia AIman
Scarica l'eseguibile di **AIman** dalla sezione [Releases](https://github.com/bhaki18/AIman/releases) e avvialo dal tuo terminale:

```bash
./AIman
```

Al primo avvio, AIman creerà automaticamente la sua cartella di lavoro nella tua Home (`~/.AIman`).

---

### 2. Usa il Menu Interattivo
Muoviti con le **frecce `↑` e `↓`** e premi **`Invio`** per scegliere cosa fare:

1. **Avvia un modello Ai** — Scegli uno dei modelli che hai scaricato e inizia subito a usarlo.
2. **Installa un modello Ai** — Scarica un nuovo modello `.gguf` da Hugging Face.
3. **Disinstalla un modello Ai** — Elimina un modello per liberare spazio sul computer.
4. **Installa un server** — Scarica il motore necessario per far girare i modelli AI (es. `mistralrs` o `llama.cpp`).
5. **Disinstalla un server** — Rimuovi un server installato.
6. **Esci** — Chiude il programma.

---

### 3. Il tuo primo avvio in 3 passi
Se è la prima volta che usi AIman:
1. Vai su **`Installa un server`** per scaricare un motore di avvio (ti basta farlo solo la prima volta).
2. Vai su **`Installa un modello Ai`** e scarica il modello `.gguf` che preferisci da Hugging Face.
3. Vai su **`Avvia un modello Ai`**, seleziona il modello appena scaricato e inizia a chattare!

---

## 📂 Dove vengono salvati i file?

AIman tiene tutto ordinato in un'unica cartella nascosta nella tua Home (`~/.AIman`), senza sporcare il sistema:

* **`~/.AIman/models/`** — Contiene i file `.gguf` dei modelli AI scaricati.
* **`~/.AIman/servers/`** — Contiene i motori installati per avviare i modelli.
* **`~/.AIman/models_config/`** — Contiene le preferenze salvate per ciascun modello (così AIman ricorda quale server usare).

---

## 📜 Licenza e Autore

Creato da **Angelo Del Piano** ([@bhaki18](https://github.com/bhaki18)).  
Distribuito sotto licenza libera **GNU General Public License v3.0 (GPL-3.0)** — consulta il file [LICENSE](./LICENSE) per maggiori dettagli.
