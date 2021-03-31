use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "abc")]
struct Test {
    age: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerConf {
    age: Test,
    workers: u64,
    ignore: bool,
    auth_server: Option<String>,
}

fn main() {
    let conf = ServerConf {
        age: Test { age: 30 },
        workers: 100,
        ignore: true,
        auth_server: Some(String::from("zyd")),
    };

    {
        println!("json:");
        let str_json = serde_json::to_string(&conf).unwrap();
        println!("json: {}", str_json);
        let de_conf: ServerConf = serde_json::from_str(&str_json).unwrap();
        println!("struct: {:#?}", de_conf);
    }
    println!("");
    {
        println!("yaml:");
        let str_yaml = serde_yaml::to_string(&conf).unwrap();
        println!("yaml: {}", str_yaml);
        let de_conf: ServerConf = serde_yaml::from_str(&str_yaml).unwrap();
        println!("struct: {:#?}", de_conf);
    }
}
