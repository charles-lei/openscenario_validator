
use walkdir::WalkDir;
use libxml::error::StructuredError;
use libxml::schemas::SchemaParserContext;
use libxml::schemas::SchemaValidationContext;
use libxml::parser::Parser;
use clap::Parser as ClapParser;

/// Simple program to validate openscenario
#[derive(ClapParser)]
#[clap(name = "OpenScenario File Validator")]
#[clap(author = "Leili. <leili@guardstrike.com>")]
#[clap(version = "v0.1")]
#[clap(about = "This Program could validate Openscenario File(XML) against XSD!", long_about = None)]
struct Args {
   ///XML Path 
   #[clap(long, value_parser)]
   xml_path: String,

   ///XSD Path
   #[clap(long, value_parser)]
   xsd_path: String,
}

fn main() {
  let args = Args::parse();
  let result = validate(&args.xml_path.as_ref(), &args.xsd_path.as_ref());
  match result{
    Ok(msg) => {println!("{}", msg)},
    Err(errors) => {
      for err in &errors {
        println!("{}", err.message());
      }
    }
  }
}

fn validate(xml_path: &str, xsd_path: &str) -> Result<String, Vec<StructuredError>>
{
  let xml = Parser::default()
  .parse_file(xml_path)
  .expect("Expected to be able to parse XML Document from file");
  let mut xsdparser = SchemaParserContext::from_file(xsd_path);
  let mut xsd = SchemaValidationContext::from_parser(&mut xsdparser)?;
  xsd.validate_document(&xml)?;
  Ok(String::from(format!("Validate {} Success", xml_path)))
}

#[test]
fn test_openscenario1_0(){
  let result = validate("tests/openscenario1.0.xml", "tests/openscenario1.0.xsd");
  match result{
    Ok(msg) => {println!("{}", msg)},
    Err(_) => {
      panic!("Failed")
    }
  }
}

#[test]
fn test_openscenario1_2(){
  for entry in WalkDir::new("tests/OpenScenario1.2") {
    if let Ok(dir_entry) = entry {
      let e = dir_entry.clone();
      if let Some(ext) = dir_entry.into_path().extension(){
        if ext.to_str() == Some("xosc"){
          //println!("{:?}", e.into_path().to_str());
          if let Some(xml_path) = e.into_path().to_str() {
            let result = validate(xml_path, "tests/openscenario1.2.xsd");
            match result{
              Ok(msg) => {println!("{}", msg)},
              Err(_) => {
                panic!("Failed")
              }
            }
          }
        }
      }
    }
  }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}