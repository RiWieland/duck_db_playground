use serde_yaml;

use serde_yaml::Value;
use serde::de::Error;


#[derive(serde::Deserialize)]
struct configCrawl {
    CompanyName : Option<String>,
    url : Option<String>,
    carrerSuffix: Option<String>,
    }


fn readConfigEntry (path:String, position:usize) -> Result<configCrawl, Box<dyn std::error::Error>>  {
    let file = std::fs::File::open(path)?;
    let e: Value = serde_yaml::from_reader(file)?;
    
    let company_ =  e["Company"][position]["name"].as_str().map(str::to_string);
    let url_: Option<String> =  e["Company"][position]["url"].as_str().map(str::to_string);
    let carrer_suffix = e["Company"][position]["carrerSuffix"].as_str().map(str::to_string);
    
    let crawl = configCrawl{
        CompanyName :company_,
        url : url_,
        carrerSuffix :carrer_suffix,
    };

    return Ok(crawl)

}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // how to read out index?
    let path =String::from("src/config/configParser.yml"); 
    //let crawl = readConfigEntry(path, 0);

    let file: std::fs::File = std::fs::File::open(path)?;
    let e: Value = serde_yaml::from_reader(file)?;
    
    
    let company_: Option<String> =  e["Company"][0]["name"].as_str().map(str::to_string);



    Ok(())

}
