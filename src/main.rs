use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ParseMode};
use serde_json::Value;
use std::{fs, env};
use std::process::Command;

// Carrega comandos do JSON
fn load_commands() -> Value {
    let data = fs::read_to_string("commands.json").expect("Falha ao ler commands.json");
    serde_json::from_str(&data).unwrap()
}

// Executa comandos no sistema
fn run_command(cmd: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()
    } else {
        Command::new("sh")  // Mude de "sudo" para "sh"
            .arg("-c")
            .arg(cmd)
            .output()
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).into_owned()
            } else {
                format!("❌ Erro:\n{}", String::from_utf8_lossy(&output.stderr))
            }
        }
        Err(e) => format!("⚠️ Falha ao executar comando: {}", e),
    }
}

// Formata a saída para exibição
fn format_output(command: &str, output: &str) -> String {
    let mut formatted = String::new();
    formatted.push_str(&format!("🔹 *Comando executado:* `{}`\n", command));
    
    if output.is_empty() {
        formatted.push_str("✅ Executado com sucesso");
    } else {
        formatted.push_str(&format!("```\n{}\n```", output));
    }
    
    formatted
}

// Teclado principal
fn main_menu_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("🪟 Windows"),
            KeyboardButton::new("🐧 Linux"),
        ]
    ])
    .resize_keyboard()
}

// Teclado de comandos por SO
fn os_commands_keyboard(os: &str) -> KeyboardMarkup {
    let commands_data = load_commands();
    let commands = commands_data[os].as_object().unwrap();
    
    // Organiza em grupos de 2 botões por linha
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    
    for (display_name, _) in commands {
        current_row.push(KeyboardButton::new(display_name));
        
        if current_row.len() >= 2 {
            rows.push(current_row);
            current_row = Vec::new();
        }
    }
    
    // Adiciona a última linha se não estiver vazia
    if !current_row.is_empty() {
        rows.push(current_row);
    }
    
    // Linha divisória e botão de voltar
    rows.push(vec![KeyboardButton::new("──────────")]);
    rows.push(vec![KeyboardButton::new("⬅️ Voltar ao Menu")]);
    
    KeyboardMarkup::new(rows)
        .resize_keyboard()
        .one_time_keyboard()
}

// Teclado de confirmação para comandos críticos
fn confirmation_keyboard(command: &str) -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new(format!("✅ Sim, {}", command)),
            KeyboardButton::new("❌ Cancelar"),
        ]
    ])
    .resize_keyboard()
    .one_time_keyboard()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Falha ao carregar .env");
    let bot = Bot::new(env::var("TELEGRAM_BOT_TOKEN").expect("TOKEN não definido"));
    let user_id = env::var("ALLOWED_USER_ID").expect("USER_ID não definido").parse::<u64>().expect("USER_ID deve ser um número");

    teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
        // Verifica se o usuário é autorizado
        if msg.from.as_ref().map(|u| u.id.0) != Some(user_id) {
            bot.send_message(msg.chat.id, "🔒 *Acesso não autorizado!*")
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
            return Ok(());
        }

        if let Some(text) = msg.text() {
            match text {
                "/start" | "/menu" | "⬅️ Voltar ao Menu" => {
                    bot.send_message(msg.chat.id, "🖥️ *Selecione seu sistema operacional:*")
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(main_menu_keyboard())
                        .await?;
                }
                "🪟 Windows" | "🐧 Linux" => {
                    let os = if text == "🪟 Windows" { "windows" } else { "linux" };
                    bot.send_message(msg.chat.id, format!("🔧 *Comandos para {}:*", text))
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(os_commands_keyboard(os))
                        .await?;
                }
                "🔌 Desligar" => {
                    bot.send_message(msg.chat.id, "⚠️ *Tem certeza que deseja DESLIGAR o computador?*")
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(confirmation_keyboard("desligar"))
                        .await?;
                }
                confirm if confirm.starts_with("✅ Sim, ") => {
                    let command_name = confirm.replace("✅ Sim, ", "");
                    let os = if cfg!(target_os = "windows") { "windows" } else { "linux" };
                    let commands_data = load_commands();
                    
                    // Encontra o comando pelo nome exibido
                    let cmd = commands_data[os].as_object().unwrap()
                        .iter()
                        .find(|(display, _)| display.contains(&command_name))
                        .map(|(_, cmd)| cmd.as_str().unwrap())
                        .unwrap();
                    
                    let output = run_command(cmd);
                    let formatted = format_output(cmd, &output);
                    
                    bot.send_message(msg.chat.id, formatted)
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(main_menu_keyboard())
                        .await?;
                }
                "❌ Cancelar" => {
                    bot.send_message(msg.chat.id, "🔄 Operação cancelada")
                        .reply_markup(main_menu_keyboard())
                        .await?;
                }
                _ => {
                    let os = if cfg!(target_os = "windows") { "windows" } else { "linux" };
                    let commands_data = load_commands();
                    let os_commands = commands_data[os].as_object().unwrap();
                    
                    if let Some(cmd) = os_commands.get(text) {
                        let output = run_command(cmd.as_str().unwrap());
                        let formatted = format_output(text, &output);
                        
                        bot.send_message(msg.chat.id, formatted)
                            .parse_mode(ParseMode::MarkdownV2)
                            .await?;
                    }
                }
            }
        }
        Ok(())
    }).await;
}