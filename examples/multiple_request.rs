use log::debug;
use sloppy_podcast_tool::parser::{quick::Client, Parser};
use std::io::BufReader;
use ureq::{Agent, AgentBuilder};

const TEST_UTL: &str = "http://rss.lizhi.fm/rss/14093.xml";

fn init_log() {
    let _lg = flexi_logger::Logger::try_with_env_or_str("debug")
        .unwrap()
        .log_to_stdout()
        .start()
        .unwrap();
}

/**
 * Send multiple request, fetch rss by chunks
 */
fn main() {
    init_log();
    let mut start = 3000usize;
    let win_size = 7000usize;
    let agent: Agent = AgentBuilder::new().build();

    let mut count = 0;

    while count < 3 {
        let end = start + win_size;
        let rd = agent
            .get(TEST_UTL)
            .set("Range", &format!("bytes={}-{}", start, end))
            .call()
            .expect("http failed")
            .into_reader();
        let bufrd = BufReader::new(rd);
        let client = Client {};
        let (items, last_item_postion) = client.de_valid(bufrd).expect("items failed");
        for i in items {
            debug!("{:?}", i.image);
        }

        // next start byte index
        start += last_item_postion;
        count += 1;
        debug!("next start {}", start);
    }
}
