Openscenario XML File Validator
=====================================

  This Program can validtate Openscenario File(xml) against xsd!

Getting started
===============
You can download binary from [THIS LINK](https://fuzzx.oss-cn-beijing.aliyuncs.com/OSCValidator.zip), but olny Linux version there, windows installer will be provided later. 

You can also build it from source:

Install Rust on your PC, This Repo depends on [libxml2](https://github.com/GNOME/libxml2). 

```shell
git clone https://github.com/charles-lei/openscenario_validator.git
cd openscenario_validator
cargo run --bin scenario_validator -- --xml tests\openscenario1.0.xml --xsd tests\openscenario1.0.xsd
```

Tools Help
===============

```shell
./scenario_validator --help

OpenScenario File Validator v0.1
Leili. <leili@guardstrike.com>
This Program can validtate Openscenario File(xml) against xsd!        

USAGE:
OPTIONS:
    -h, --help                   Print help information
    -V, --version                Print version information
        --xml <XML>              XML Path
        --xsd <XSD>              XSD Path
```

Tools Example
===============
```shell
./scenario_validator --xml ./openscenario1.0.xml --xsd ./openscenario1.0.xsd
Validate ./openscenario1.0.xml Success

./scenario_validator --xml ./openscenario1.0.xml --xsd ./openscenario1.2.xsd
Validate ./openscenario1.0.xml Success
```
