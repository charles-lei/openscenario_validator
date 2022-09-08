Openscenario XML File Validator
=====================================

  This Program can validtate Openscenario File(xml) against xsd!

Getting started
===============

Install Rust on your PC, This Repo depends on [libxml2](https://github.com/GNOME/libxml2). 

```shell
git clone https://github.com/charles-lei/openscenario_validator.git
cd openscenario_validator
cargo run --bin scenario_validator -- --xml-path tests\openscenario1.0.xml --xsd-path tests\openscenario1.0.xsd
```

Tools Help
===============

```shell
scenario_validator --help

OpenScenario File Validator v0.1
Leili. <leili@guardstrike.com>
This Program can validtate Openscenario File(xml) against xsd!        

USAGE:
OPTIONS:
    -h, --help                   Print help information
    -V, --version                Print version information
        --xml-path <XML_PATH>    XML Path
        --xsd-path <XSD_PATH>    XSD Path
```




