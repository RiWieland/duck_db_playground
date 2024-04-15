use serde_yaml;

use serde_yaml::Value;
use serde::de::Error;


#[derive(serde::Deserialize)]
struct configCrawl {
    CompanyName : Option<String>,
    url : Option<String>,
    carrerSuffix: Option<String>,
    }




fn main() -> Result<(), Box<dyn std::error::Error>> {


}
