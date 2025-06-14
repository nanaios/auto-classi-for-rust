use anyhow::{Ok, Result};
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::{sync::Arc, thread::sleep, time::Duration};

const LOGIN_URL: &str = "https://example.com";

pub fn login_classi() -> Result<()> {
    //browserの立ち上げ
    let mut launch_option: LaunchOptionsBuilder = LaunchOptionsBuilder::default();
    launch_option.headless(false);
    let browser: Browser = Browser::new(launch_option.build()?)?;

    //tabを生成
    let tab: Arc<Tab> = browser.new_tab()?;

    //classiのログインページに移動
    tab.navigate_to(LOGIN_URL)?.wait_until_navigated()?;

    let title = tab.get_title()?;

    sleep(Duration::from_secs(60));

    println!("{}", title);

    Ok(())
}
