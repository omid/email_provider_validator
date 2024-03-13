use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const MISSED_HS_EMAILS: &[&str] = &[
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
    // free email list
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("free.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let mut email_hs_list = reqwest::blocking::get("https://f.hubspotusercontent40.net/hubfs/2832391/Marketing/Lead-Capture/free-domains-2.csv")
        .unwrap()
        .text()
        .unwrap();
    email_hs_list.make_ascii_lowercase();

    let mut free_email_list = email_hs_list.split('\n').collect::<Vec<_>>();
    free_email_list.extend(MISSED_HS_EMAILS);

    // trim and lower case all
    let mut free_email_list = free_email_list
        .into_iter()
        .map(|email| email.trim().to_ascii_lowercase())
        .filter(|email| !email.is_empty())
        .collect::<Vec<_>>();

    // remove duplicates
    free_email_list.sort_unstable();
    free_email_list.dedup();

    let mut list = phf_codegen::Set::<&[u8]>::new();
    for email in &free_email_list {
        list.entry(email.as_bytes());
    }

    writeln!(
        &mut file,
        "static FREE_EMAILS: phf::Set<&'static [u8]> = \n{};\n",
        list.build()
    )
    .unwrap();

    // disposable email list
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("disposable.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let mut email_dis_list1 =
        reqwest::blocking::get("https://disposable.github.io/disposable-email-domains/domains.txt")
            .unwrap()
            .text()
            .unwrap();
    email_dis_list1.make_ascii_lowercase();

    let mut email_dis_list2 = reqwest::blocking::get(
        "https://raw.githubusercontent.com/FGRibreau/mailchecker/master/list.txt",
    )
    .unwrap()
    .text()
    .unwrap();
    email_dis_list2.make_ascii_lowercase();

    let mut disp_email_list = email_dis_list1.split('\n').collect::<Vec<_>>();
    disp_email_list.extend(email_dis_list2.split('\n'));

    // trim and lower case all
    let mut disp_email_list = disp_email_list
        .into_iter()
        .map(|email| email.trim().to_ascii_lowercase())
        .filter(|email| !email.is_empty())
        .collect::<Vec<_>>();

    // remove duplicates
    disp_email_list.sort_unstable();
    disp_email_list.dedup();

    let mut list = phf_codegen::Set::<&[u8]>::new();
    for email in &disp_email_list {
        list.entry(email.as_bytes());
    }

    writeln!(
        &mut file,
        "static DISPOSABLE_EMAILS: phf::Set<&'static [u8]> = \n{};\n",
        list.build()
    )
    .unwrap();
}
