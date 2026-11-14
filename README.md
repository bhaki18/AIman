# AIman

    AIman is an AI manager for local models,something like ollama,but in rust!
    #ILOVERUST

## struttura del progetto

 .AIman/
    models/
        // i vari modelli AI tipo:
        ai.gguf
        modello.gguf
        modello2.gguf
    models_config/
        // tipo di struttura:
            NomeModello/
                config.json
    servers/
        // i vari tipi di server per modelli llm,tts,stt,ecc...
        server.bin

## struttura di un config.json

```javascript

{
    "nome_modello":"ai",
    "path_modello":".AIman/models/ai.gguf",
    "famiglia_modello":"qwen",
    "tipo_di_modello":"llm",
    "repo_hf":"https://huggingface.co/company/ai.gguf",
    "server_utilizzato":".AIman/server/mistralrs.bin",
    "quantizzazione":"16"
}

```
