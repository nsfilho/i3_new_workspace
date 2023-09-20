use std::process::Command;
use serde::{Deserialize, Serialize};

/**
* Este projeto consulta o i3wm via i3-msg para pegar todas as workspaces
* e gera uma nova workspace no monitor atual
*/

/**
* Estrutura de uma workspace
*/
#[derive(Serialize, Deserialize)]
struct Workspace {
    name: String,
    visible: bool,
    focused: bool,
    urgent: bool,
    rect: Rect,
    output: String,
}

/**
* Estrutura de um retângulo
*/
#[derive(Serialize, Deserialize)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn main() {
    // Pega todas as workspaces
    let output = Command::new("i3-msg")
        .arg("-t")
        .arg("get_workspaces")
        .output()
        .expect("failed to execute process");

    // Converte o output para string e faz o parsing
    let output = String::from_utf8(output.stdout).unwrap();
    let workspaces: Vec<Workspace> = serde_json::from_str(&output).unwrap();

    // Pega o numero da proxima workspace
    let next_workspace_number = workspaces.len() + 1;

    // Cria a nova workspace
    Command::new("i3-msg")
        .arg("workspace")
        .arg(next_workspace_number.to_string())
        .output()
        .expect("failed to execute process");
}
