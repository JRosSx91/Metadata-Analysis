use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::io;

fn main() {
    // Crea un cliente HTTP
    let client = Client::builder()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537")
    .build()
    .expect("Failed to build client");

    // Hacer una solicitud GET a la página web que te interesa
    let stdin = io::stdin();
    let mut url = String::new();

    println!("Introduce the web you want to analyse")
    let res = client
        .get("https://www.neosalus.com")
        .send()
        .expect("Failed to send request");

    // Convertir la respuesta en texto
    let body = res.text().expect("Failed to read response");

    // Analizar el cuerpo de la respuesta en HTML
    let document = Html::parse_document(&body);

    // Crear selectores para encontrar los metadatos que te interesan
    let title_selector = Selector::parse("title").expect("Failed to create selector");
    let desc_selector =
        Selector::parse("meta[name=description]").expect("Failed to create selector");
    let keywords_selector =
        Selector::parse("meta[name=keywords]").expect("Failed to create selector");

    // Imprimir el título de la página
    if let Some(element) = document.select(&title_selector).next() {
        let title = element.inner_html();
        println!("Title: {}", title);
    }

    // Imprimir la descripción de la página
    if let Some(element) = document.select(&desc_selector).next() {
        if let Some(desc) = element.value().attr("content") {
            println!("Description: {}", desc);
        }
    }

    // Imprimir las palabras clave de la página
    if let Some(element) = document.select(&keywords_selector).next() {
        if let Some(keywords) = element.value().attr("content") {
            println!("Keywords: {}", keywords);
        }
    }
}
