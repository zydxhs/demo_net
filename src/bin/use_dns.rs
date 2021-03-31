use std::env;
use trust_dns_proto::rr::record_type::RecordType;
// use trust_dns::rr::record_type::RecordType;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please input a name to query!");
        std::process::exit(1);
    }
    let query = format!("{}.", args[1]);
    // let query = "www.baidu.com".to_string();

    println!("使用默认配置:");
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = resolver.lookup_ip(query.as_str()).unwrap();
    for ans in response.iter() {
        println!("{:?}", ans);
    }

    println!("使用系统配置:");
    let resolver = Resolver::from_system_conf().unwrap();
    let response = resolver.lookup_ip(query.as_str()).unwrap();
    for ans in response.iter() {
        println!("{:?}", ans);
    }

    println!("使用 ns:");
    let ns = resolver.lookup(query.as_str(), RecordType::NS);
    for ans in ns.iter() {
        println!("{:?}", ans);
    }
}
