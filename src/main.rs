use reqwest;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::io::{stdin,stdout,Write};

fn parse_node(node: scraper::element_ref::ElementRef) -> Option<HashMap<String, HashMap<&str, String>>> {
    let mut node_map: HashMap<&str, String> = HashMap::new();
    let selector_td = Selector::parse(r#"td"#).unwrap();
    let selector_a = Selector::parse(r#"a"#).unwrap();
    let selector_span = Selector::parse(r#"span"#).unwrap();
    let selector_div = Selector::parse(r#"div"#).unwrap();
    let selector_input = Selector::parse(r#"input"#).unwrap();
    let res = node.select(&selector_td);
    let mut bs_sdet: String = String::from("not_found");
    let mut bs_sknr: String = String::from("not_found");
    let mut bs_stag: String = String::from("not_found");
    let mut bs_szeit: String = String::from("not_found");
    let mut bs_skl: String = String::from("not_found");
    let mut bs_sort: String = String::from("not_found");
    let mut bs_sbuch: String = String::from("not_found");
    let mut bs_spreis: String = String::from("not_found");
    let mut bs_szr: String = String::from("not_found");
    for node in res {
        let key: &str =  node.value().attr("class").unwrap();
        let value: String = node.inner_html();
        match key {
            "bs_sdet" => bs_sdet = value,
            "bs_sknr" => bs_sknr = value,
            "bs_stag" => bs_stag = value,
            "bs_szeit" => bs_szeit = value,
            "bs_sort" => bs_sort = node.select(&selector_a).next().unwrap().inner_html(),
            "bs_szr" => bs_szr = node.select(&selector_a).next().unwrap().inner_html(),
            "bs_skl" => bs_skl = node.select(&selector_a).next().unwrap().inner_html(),
            "bs_spreis" => match node.select(&selector_div).next() {
                Some(inner) => bs_spreis = inner.inner_html()
                .split("€").next().unwrap().to_string()+"€",
                None => bs_spreis = node.select(&selector_span).next().unwrap().inner_html()
                .split("€").next().unwrap().to_string()
            },
            "bs_sbuch" => match node.select(&selector_input).next() {
                Some(inner) => bs_sbuch = String::from(inner.value().attr("value").unwrap()),
                None => bs_sbuch = node.select(&selector_span).next().unwrap().inner_html()
            },
            //_ => println!("something else!"),
            _ => (),
        }
    }
    if bs_sknr.contains("not_found") {
        return None;
    }    

    if bs_stag != String::from("not_found") && bs_szeit != String::from("not_found"){
        bs_stag = bs_stag + " " + &bs_szeit;
    }
    node_map.insert("bs_sbuch", bs_sbuch);
    node_map.insert("bs_sdet", bs_sdet);
    node_map.insert("bs_spreis", bs_spreis);
    node_map.insert("bs_szr", bs_szr);
    node_map.insert("bs_sort", bs_sort);
    node_map.insert("bs_skl", bs_skl);
    node_map.insert("bs_datum", bs_stag);
    let map: HashMap<String, HashMap<&str, String>> = HashMap::from([
        (bs_sknr, node_map)
    ]);
    return Some(map);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut map: HashMap<&str, HashMap<String, HashMap<&str, String>>> = HashMap::new();
        print!("Name des Sportkurses: ");
        let _=stdout().flush();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?.to_string();
        if let Some('\n')=buffer.chars().next_back() {
            buffer.pop();
        }
        if let Some('\r')=buffer.chars().next_back() {
            buffer.pop();
        }
        if buffer.contains("exit") | buffer.contains("quit") {
            break;
        }
        let url: String = String::from("https://www2.usz.uni-halle.de/angebote/aktueller_zeitraum/_");
        buffer = format!("{}{buffer}", buffer.remove(0).to_uppercase());
        let url = url+&buffer+".html";
        map.insert(buffer.as_str(), HashMap::new());
        let html = reqwest::get(url)
            .await?
            .text()
            .await?;
        let document = Html::parse_document(&html);
        let selector = Selector::parse(r#"tr"#).unwrap();
        let res = document.select(&selector);
        for node in res {
            match parse_node(node) {
                Some(inner) => map.get_mut(buffer.as_str()).unwrap().extend(inner),
                None => (),
            };
        }
        println!("{:#?}", map);
    }
    Ok(())
}