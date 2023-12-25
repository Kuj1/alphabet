use soup::prelude::*;
use tokio::time::{sleep, Duration};
use thirtyfour::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use serde_json::to_writer_pretty as wp;
use clap::Parser;

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
struct Alphabeth {
    name: String,
    alphabeth: BTreeMap<String, String>
}

impl  Alphabeth  {
    fn new (name: String, alphabeth: BTreeMap<String, String>) -> Alphabeth  {
        Alphabeth { name: name.to_string(), alphabeth }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short,long)]
    #[clap(value_name = "IP:PORT")]
    ip_port: String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let cli_arg = format!("http://{}", Args::parse().ip_port);

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;

    let driver = WebDriver::new(&cli_arg, caps).await?;
    driver.goto("https://font-generators.com/ru/").await?;
    
    let alphabeth_list = "A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z";

    let elem_form_gen = driver.find(By::Id("generator-text")).await?;
    elem_form_gen.send_keys(alphabeth_list).await?;

    sleep(Duration::from_secs(2)).await;

    let html_fonts = driver.source().await?;
    let soup_fonts = Soup::new(&html_fonts);

    let mut struct_charlist = vec![];

    for i in soup_fonts.class("generator-font").find_all() {

        let each_font_name = i.class("font-name").find().expect("!!!").text();
        let each_font_char = i.class("generator-font__content").find().expect("!!").text();

        let each_alpha_charlist = alphabeth_list.split(',').map(|ts| ts.to_string());
        let each_font_charlist = each_font_char.split(',').map(|ts| ts.to_string());

        let tuple_char: Vec<_> = each_alpha_charlist.into_iter().zip(each_font_charlist).collect();

        let char_map: BTreeMap<_, _> = tuple_char.into_iter().collect();
        let char_struct = Alphabeth::new(each_font_name, char_map);

        struct_charlist.push(char_struct);
    }

    let mut file = std::fs::File::create("fonts.json")?;
    wp(&mut file, &struct_charlist)?;
    

    driver.quit().await?;

    Ok(())
}