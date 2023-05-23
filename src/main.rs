use regex::Regex;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::io;

fn main() {
    loop {
        let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537")
        .build()
        .expect("Failed to build client");
        println!("Enter the web (type 'exit' to quit): ");
        let stdin = io::stdin();
        let mut url = String::new();

        stdin.read_line(&mut url).expect("Failed to read");

        if url == "exit" {
            break;
        }

        let res = client.get(url).send().expect("Failed to send request");

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
        let robots_selector =
            Selector::parse("meta[name=robots]").expect("Failed to create selector");
        let h1_selector = Selector::parse("h1").expect("Failed to create selector");
        let h2_selector = Selector::parse("h2").expect("Failed to create selector");
        let h3_selector = Selector::parse("h3").expect("Failed to create selector");
        let h4_selector = Selector::parse("h4").expect("Failed to create selector");
        let h5_selector = Selector::parse("h5").expect("Failed to create selector");
        let h6_selector = Selector::parse("h6").expect("Failed to create selector");
        let link_selector = Selector::parse("a[href]").expect("Failed to create selector");
        let email_regex = Regex::new(r"^mailto:(.+@.+\.com)$").expect("Failed to create regex");

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

        if let Some(element) = document.select(&robots_selector).next() {
            if let Some(robots) = element.value().attr("content") {
                println!("Robots: {}", robots);
            }
        }

        if let Some(element) = document.select(&h1_selector).next() {
            let h1 = element.inner_html();
            println!("H1: {}", h1);
        }
        if let Some(element) = document.select(&h2_selector).next() {
            let h2 = element.inner_html();
            println!("H2: {}", h2);
        }
        if let Some(element) = document.select(&h3_selector).next() {
            let h3 = element.inner_html();
            println!("H3: {}", h3);
        }
        if let Some(element) = document.select(&h4_selector).next() {
            let h4 = element.inner_html();
            println!("H4: {}", h4);
        }
        if let Some(element) = document.select(&h5_selector).next() {
            let h5 = element.inner_html();
            println!("H5: {}", h5);
        }
        if let Some(element) = document.select(&h6_selector).next() {
            let h6 = element.inner_html();
            println!("H6: {}", h6);
        }
        for link_element in document.select(&link_selector) {
            // Obtener el href del enlace
            if let Some(href) = link_element.value().attr("href") {
                // Comprobar si el href es un email
                if let Some(captures) = email_regex.captures(href) {
                    // Obtener el email de los grupos de captura
                    let email = captures.get(1).unwrap().as_str();
                    println!("Email: {}", email);
                }
            }
        }
    }
}
