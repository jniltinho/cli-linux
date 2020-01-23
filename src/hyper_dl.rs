//!https://paulohrpinheiro.xyz/texts/rust/2016-11-21-download-arquivos.html

use hyper;
use std::fs::File; // para criar arquivos
use std::io::{Read, Write}; // para IO de arquivos
use std::path::Path; // configurar nome de arquivo
use std::thread; // concorrência

const ROBOT_NAME: &'static str = "nilton-curl";
const BUFFER_SIZE: usize = 512;

pub fn run_download(url: String) {
    // vetor para as threads que serão criadas
    let mut workers = vec![];

    workers.push(thread::spawn(move || match download_content(&url) {
        Err(error) => println!("{:?}", error),
        Ok(ok) => println!("{}", ok),
    }));

    // espera cada thread acabar
    for worker in workers {
        let _ = worker.join();
    }
}

fn download_content(url: &str) -> Result<String, String> {
    // Somos um respeitável e conhecido bot
    let mut headers = hyper::header::Headers::new();
    headers.set(hyper::header::UserAgent(ROBOT_NAME.to_string()));

    // Pega cabeçalhos (e possivelmente algum dado já)
    let client = hyper::Client::new();
    let mut response;

    match client.get(url).headers(headers).send() {
        Err(error) => return Err(format!("{:?}", error)),
        Ok(res) => response = res,
    }

    // Cria arquivo para salvar conteúdo
    let filename = Path::new(&url).file_name().unwrap();
    let mut localfile;

    match File::create(filename) {
        Err(error) => return Err(format!("{:?}", error)),
        Ok(filehandle) => localfile = filehandle,
    }

    // pega conteúdo e salva em arquivo
    loop {
        let mut buffer = [0; BUFFER_SIZE];

        match response.read(&mut buffer) {
            Err(read_error) => return Err(format!("{:?}", read_error)),
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // não tem mais o que ler
                    break;
                }
                // vamos tentar escrever o que pegamos
                match localfile.write(&buffer[0..bytes_read]) {
                    Err(write_error) => return Err(format!("{:?}", write_error)),
                    Ok(bytes_write) => {
                        if bytes_write != bytes_read {
                            return Err("Error in write.".to_string());
                        }
                    }
                }
            }
        }
    }

    return Ok(format!(
        "Download of '{}' has been completed.",
        String::from(filename.to_str().unwrap())
    ));
}
