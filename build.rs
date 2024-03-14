use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const FREE_EMAILS: &[&str] = &[
    "pm.me",
    "proton.me",
    "protonmail.ch",
    "tutanota.com",
    "tutanota.de",
    "tutamail.com",
    "tuta.io",
    "keemail.me",
    "hey.com",
    "vk.com",
    "onmail.com",
    "cock.li",
    "doge-mail.net",
    "mailfence.com",
    "126.cn",
    "foxmail.cn",
    "foxmail.com.cn",
    "qq.com.cn",
    "sina.com.cn",
    "tom.cn",
    "tom.com.cn",
    "bvhrk.com",
    "bvhrs.com",
    "kvhrr.com",
    "kvhrs.com",
    "kvhrw.com",
    "nthrl.com",
    "nthrw.com",
    "nvhrw.com",
    "tmmbt.com",
    "tmmbt.net",
    "tmmcv.net",
    "tmmwj.com",
    "tmmwj.net",
    "neoistone.com",
    "hostingaro.com",
    "openmbox.net",
];

const DISPOSAL_EMAILS: &[&str] = &["nezid.com", "zbock.com", "zslsz.com", "omeie.com"];

// const EXAMPLE_DOMAINS: &[&str] = &[
//     "example.com",
//     "example.net",
//     "example.org",
//     "xn--kgbechtv",
//     "xn--hgbk6aj7f53bba",
//     "xn--0zwm56d",
//     "xn--g6w251d",
//     "xn--80akhbyknj4f",
//     "xn--11b5bs3a9aj6g",
//     "xn--jxalpdlp",
//     "xn--9t4b11yi5a",
//     "xn--deba0ad",
//     "xn--zckzah",
//     "xn--hlcj6aya9esc7a",
// ];

fn main() {
    if env::var("DOCS_RS").is_ok() {
        gen_static("free", &[]);
        gen_static("disposable", &[]);
        return;
    }

    // free email list
    let name = "free";
    let email_list = get_clean_web_result(&[
        "https://f.hubspotusercontent40.net/hubfs/2832391/Marketing/Lead-Capture/free-domains-2.csv"
    ], FREE_EMAILS);

    gen_static(name, &email_list);
    gen_output(name, &email_list);

    // disposable email list
    let name = "disposable";
    let email_list = get_clean_web_result(
        &[
            "https://disposable.github.io/disposable-email-domains/domains.txt",
            "https://raw.githubusercontent.com/FGRibreau/mailchecker/master/list.txt",
        ],
        DISPOSAL_EMAILS,
    );

    gen_static(name, &email_list);
    gen_output(name, &email_list);

    // example email list
    // let name = "example";
    // let email_list = get_clean_web_result(
    //     &[],
    //     EXAMPLE_DOMAINS,
    // );
    //
    // gen_static(name, &email_list);
    // gen_output(name, &email_list);
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

fn get_clean_web_result(urls: &[&str], extend: &[&str]) -> Vec<String> {
    let email_list = urls
        .iter()
        .map(|url| reqwest::blocking::get(*url).unwrap().text().unwrap())
        .map(|v| v.to_lowercase());

    let init_list = extend
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
