use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let lists = [(
            "free",
            vec![
                "https://f.hubspotusercontent40.net/hubfs/2832391/Marketing/Lead-Capture/free-domains-2.csv"
            ]
        ),
        (
            "disposable",
            vec![
                "https://disposable.github.io/disposable-email-domains/domains.txt",
                "https://raw.githubusercontent.com/FGRibreau/mailchecker/master/list.txt"
            ]
        )];

    for (name, download_list) in lists.iter() {
        if env::var("DOCS_RS").is_ok() {
            gen_static(name, &[]);
        } else {
            create_list(name, download_list)
        }
    }
}

fn create_list(name: &str, download_list: &[&str]) {
    let extra = read_to_string(format!("input/{}_emails.csv", name)).unwrap();

    let email_list = get_clean_web_result(download_list, &extra.lines().collect::<Vec<_>>());

    gen_static(name, &email_list);
    gen_output(name, &email_list);
}

fn get_clean_web_result(urls: &[&str], extra: &[&str]) -> Vec<String> {
    let email_list = urls
        .iter()
        .map(|url| reqwest::blocking::get(*url).unwrap().text().unwrap())
        .map(|v| v.to_lowercase());

    let init_list = extra
        .iter()
        .map(|&v| v.to_string())
        .collect::<Vec<String>>();

    let mut email_list = email_list.fold(init_list, |mut acc: Vec<String>, text| {
        let lines = text.lines().map(String::from).collect::<Vec<String>>();
        acc.extend(lines);
        acc
    });

    email_list.sort_unstable();
    email_list.dedup();

    email_list
}

fn gen_static(name: &str, email_list: &[String]) {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(format!("{name}.rs"));
    let mut file = BufWriter::new(File::create(path).unwrap());

    let mut list = phf_codegen::Set::<&[u8]>::new();
    for email in email_list {
        list.entry(email.as_bytes());
    }

    writeln!(
        &mut file,
        "static {}_EMAILS: phf::Set<&'static [u8]> = \n{};\n",
        name.to_uppercase(),
        list.build()
    )
    .unwrap();
}

fn gen_output(name: &str, email_list: &[String]) {
    let path = Path::new("output").join(format!("{name}.csv"));
    let mut file = BufWriter::new(File::create(path).unwrap());

    writeln!(&mut file, "{}", email_list.join("\n")).unwrap();

    println!(
        "cargo:warning=Number of {} emails: {}",
        name,
        email_list.len()
    );
}
