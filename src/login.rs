use anyhow::Result;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab, protocol::cdp::Network::Cookie};
use std::{sync::Arc, time::Duration};

const LOGIN_URL: &str = "https://id.classi.jp/login/identifier";

pub fn login_classi() -> Result<Vec<Cookie>> {
    //browserの立ち上げ
    let mut launch_option: LaunchOptionsBuilder = LaunchOptionsBuilder::default();
    launch_option.headless(false);
    let browser: Browser = Browser::new(launch_option.build()?)?;

    //tabの生成とtimeoutの設定
    let tab: Arc<Tab> = browser.new_tab()?;
    tab.set_default_timeout(Duration::from_secs(86400));

    //classiのログインページに移動
    tab.navigate_to(LOGIN_URL)?.wait_until_navigated()?;

    println!("classi loading!");

    //classiのホームへの遷移待機
    tab.wait_for_element("a.logo")?;

    tab.get_cookies()
}

pub fn write_cookie(cookies: Vec<Cookie>) {
    for cookie in cookies {
        println!("{}", cookie.value);
    }
}
