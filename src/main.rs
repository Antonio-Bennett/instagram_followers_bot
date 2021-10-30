use std::{collections::HashSet, thread::sleep, time::Duration};

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).await?;

    driver.get("https://instagram.com").await?;

    let username = driver.query(By::Css("#loginForm > div.Igw0E.IwRSH.eGOV_._4EzTm.kEKum > div:nth-child(1) > div > label > input")).first().await?;

    username.send_keys("6479660493").await?;

    let password = driver.query(By::Css("#loginForm > div.Igw0E.IwRSH.eGOV_._4EzTm.kEKum > div:nth-child(2) > div > label > input")).first().await?;

    password.send_keys("!1932Seneca!!").await?;

    let login_button = driver
        .query(By::Css(
            "#loginForm > div.Igw0E.IwRSH.eGOV_._4EzTm.kEKum > div:nth-child(3)",
        ))
        .first()
        .await?;

    login_button.click().await?;

    let pop_up_close = driver
        .query(By::Css(
            "#react-root > section > main > div > div > div > div > button",
        ))
        .first()
        .await?;

    pop_up_close.click().await?;

    let notif_close = driver
        .query(By::Css(
            "body > div.RnEpo.Yx5HN > div > div > div > div.mt3GC > button.aOOlW.HoLwm",
        ))
        .first()
        .await?;

    notif_close.click().await?;

    let profile_menu = driver.query(By::Css("#react-root > section > nav > div._8MQSO.Cx7Bp > div > div > div.ctQZg > div > div:nth-child(6) > span > img")).first().await?;

    profile_menu.click().await?;

    let profile = driver.query(By::Css("#react-root > section > nav > div._8MQSO.Cx7Bp > div > div > div.ctQZg > div > div:nth-child(6) > div.poA5q > div.uo5MA._2ciX.tWgj8.XWrBI > div._01UL2 > a:nth-child(1) > div")).first().await?;

    profile.click().await?;

    let followers_amount = driver.query(By::Css("#react-root > section > main > div > header > section > ul > li:nth-child(2) > a > span")).first().await?;
    let followers_amount = followers_amount.inner_html().await?;
    let followers_amount = followers_amount.parse::<usize>().unwrap();

    let following_amount = driver.query(By::Css("#react-root > section > main > div > header > section > ul > li:nth-child(3) > a > span")).first().await?;
    let following_amount = following_amount.inner_html().await?;
    let following_amount = following_amount.parse::<usize>().unwrap();

    let mut followers: HashSet<String> = HashSet::new();
    let mut following: HashSet<String> = HashSet::new();

    let followers_menu = driver
        .query(By::Css(
            "#react-root > section > main > div > header > section > ul > li:nth-child(2) > a",
        ))
        .first()
        .await?;
    followers_menu.click().await?;

    let followers_list = driver
        .query(By::Css(
            "body > div.RnEpo.Yx5HN > div > div > div.isgrP > ul > div",
        ))
        .first()
        .await?;

    let mut followers_list_elems = followers_list.find_elements(By::Css("li")).await?;

    while followers_list_elems.len() < followers_amount {
        let last_li = followers_list
            .query(By::Css("li:last-child"))
            .first()
            .await?;

        last_li.scroll_into_view().await?;

        followers_list_elems = followers_list.find_elements(By::Css("li")).await?;

        sleep(Duration::from_secs(2));
    }

    for follower in followers_list_elems {
        let name = follower
            .query(By::Css(
                "div > div.t2ksc > div.enpQJ > div.d7ByH > span > a",
            ))
            .first()
            .await?;
        let name = name.inner_html().await?;
        followers.insert(name);
    }

    let close_list = driver.query(By::Css("body > div.RnEpo.Yx5HN > div > div > div:nth-child(1) > div > div:nth-child(3) > button")).first().await?;
    close_list.click().await?;

    let following_menu = driver
        .query(By::Css(
            "#react-root > section > main > div > header > section > ul > li:nth-child(3) > a",
        ))
        .first()
        .await?;
    following_menu.click().await?;

    let following_list = driver
        .query(By::Css(
            "body > div.RnEpo.Yx5HN > div > div > div.isgrP > ul > div",
        ))
        .first()
        .await?;

    let mut following_list_elems = following_list.find_elements(By::Css("li")).await?;

    while following_list_elems.len() < following_amount {
        let last_li = following_list
            .query(By::Css("li:last-child"))
            .first()
            .await?;

        last_li.scroll_into_view().await?;

        following_list_elems = following_list.find_elements(By::Css("li")).await?;

        sleep(Duration::from_secs(2));
    }

    for following_ in following_list_elems {
        let name = following_
            .query(By::Css(
                "div > div.t2ksc > div.enpQJ > div.d7ByH > span > a",
            ))
            .first()
            .await?;
        let name = name.inner_html().await?;
        following.insert(name);
    }

    let close_list = driver.query(By::Css("body > div.RnEpo.Yx5HN > div > div > div:nth-child(1) > div > div:nth-child(3) > button")).first().await?;
    close_list.click().await?;

    let diff = following.symmetric_difference(&followers);
    if followers_amount > following_amount {
        println!("Accounts you might not be following back");
        dbg!(&diff);
    } else {
        println!("Accounts that might not be following you back");
        dbg!(&diff);
    }

    driver.quit().await?;

    Ok(())
}
