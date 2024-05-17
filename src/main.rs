use error_chain::error_chain;
use regex::Regex;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let base_url = "https://search.bilibili.com/all?keyword=%E5%BA%86%E4%BD%99%E5%B9%B42&from_source=webtop_search&spm_id_from=333.934&search_source=2&page=";
    let base_offset = "&o=";
    let re = Regex::new(r"//www\.bilibili\.com/video/[^?]*\?from=search").unwrap();
    let mut count = 0;
    let mut page = 1;
    let mut visited_links = HashSet::new();
    let mut output_file = File::create("庆余年2视频链接.txt")?; // 处理Result以获取File对象


    loop {
        let url = format!("{}{}{}{}", base_url, page, base_offset, (page - 1) * 36);
        let res = reqwest::get(&url).await?.text().await?;

        let links_count = Document::from(res.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .filter(|href| re.is_match(href))
            .map(|href| {
                let full_url = if href.starts_with("//") {
                    format!("https:{}", href)
                } else {
                    href.to_string()
                };

                if !visited_links.contains(&full_url) {
                    visited_links.insert(full_url.clone());
                    count += 1;
                    writeln!(&mut output_file, "Link {}: {}", count, full_url).unwrap();
                    //println!("数据已写入文件!");
                }
            })
            .count();

        if links_count == 0 {
            break; // No more links found on this page, exit the loop
        }

        page += 1;
    }
    println!("数据已写入文件");

    Ok(())
}
