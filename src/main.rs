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

    // cria um novo array com o numero das workspaces ordenados, como numero inteiro
    let mut workspaces: Vec<i32> = workspaces
        .iter()
        .filter_map(|workspace| workspace.name.parse::<i32>().ok())
        .collect();

    workspaces.sort_by(|a, b| a.cmp(b));

    // Calcula o numero da nova workspace com base nas que ja existem
    // exemplo 1: atualmente existem as workspaces 1, 3, 4, 5 -- então a nova workspace será a 2
    // exemplo 2: atualmente existem as workspaces 1, 2, 3, 4 -- então a nova workspace será a 5
    //
    // Precisa correr o vetor de workspaces de modo ordenado para pegar o menor numero que não existe
    let mut next_workspace_number = 1;
    for workspace in workspaces {
        if workspace != next_workspace_number {
            break;
        }
        next_workspace_number += 1;
    }

    // Cria a nova workspace
    Command::new("i3-msg")
        .arg("workspace")
        .arg(next_workspace_number.to_string())
        .output()
        .expect("failed to execute process");
}
