# 🤖 Bot de Comandos via Telegram

Este é um bot Telegram desenvolvido em Rust usando a biblioteca [`teloxide`](https://github.com/teloxide/teloxide). Ele permite executar comandos do sistema operacional de forma remota via interface de bot, com suporte para Windows e Linux.

## 📦 Dependências

- [Rust](https://www.rust-lang.org/)
- [teloxide](https://crates.io/crates/teloxide)
- [tokio](https://crates.io/crates/tokio)
- [dotenv](https://crates.io/crates/dotenv)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)

## 🛠 Estrutura do Projeto

- `main.rs`: Código principal do bot.
- `commands.json`: Define os comandos disponíveis para cada sistema operacional.
- `.env`: Contém variáveis de ambiente sensíveis (não versionar!).

## 🤖 Criando o Bot no Telegram

1. No Telegram, procure por [`@BotFather`](https://t.me/BotFather).
2. Envie o comando `/newbot` e siga as instruções para definir nome e usuário do bot.
3. Após a criação, o BotFather fornecerá um token de API. Esse será seu `TELEGRAM_BOT_TOKEN`.

## 🔐 Como obter o `ALLOWED_USER_ID`

1. Acesse o bot [`@userinfobot`](https://t.me/userinfobot) no Telegram.
2. Ele retornará seu `user_id`. Copie esse número e adicione ao arquivo `.env`.

## 📁 Exemplo de `commands.json`

```json
{
    "windows": {
        "📁 Listar Arquivos": "dir",
        "🌐 Info Rede": "ipconfig /all",
        "🖥️ Processos": "tasklist",
        "⚙️ Info Sistema": "systeminfo",
        "🔌 Desligar": "shutdown /s /t 60"        
    },
    "linux": {
        "📁 Listar Arquivos": "ls -la",
        "💾 Espaço Disco": "df -h",
        "📄 Memoria": "free -h",
        "🌐 Info Rede": "ifconfig || ip a",
        "🔌 Desligar": "sudo /sbin/shutdown -h now",
        "⚙️ Info Sistema": "uname -a"
    }   
}
```

## 🔐 `.env` (NÃO COMMITAR!)

Crie um arquivo `.env` com as seguintes variáveis:

```env
TELEGRAM_BOT_TOKEN=seu_token_aqui
ALLOWED_USER_ID=123456789
```

## 🚀 Executando o projeto

```bash
cargo run
```

> Certifique-se de que o arquivo `commands.json` está presente no diretório raiz.

## 🧾 Funcionalidades

- Teclado interativo para escolha de comandos.
- Verificação de usuário autorizado via `ALLOWED_USER_ID`.
- Execução segura de comandos com teclado de confirmação.
- Suporte multiplataforma (Windows/Linux).

## 🛡 Segurança

- Apenas um usuário autorizado pode interagir com o bot.
- Confirmação antes de comandos críticos como desligamento.
- Arquivo `.env` é ignorado pelo `.gitignore`.

## 📄 Licença

MIT
