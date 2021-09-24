use std::{thread::sleep, time::Duration};

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

     let login_button = driver.query(By::Css("#loginForm > div.Igw0E.IwRSH.eGOV_._4EzTm.kEKum > div:nth-child(3)")).first().await?;

     login_button.click().await?;

     let pop_up_close = driver.query(By::Css("#react-root > section > main > div > div > div > div > button")).first().await?;

     pop_up_close.click().await?;

     let notif_close = driver.query(By::Css("body > div.RnEpo.Yx5HN > div > div > div > div.mt3GC > button.aOOlW.HoLwm")).first().await?;

     notif_close.click().await?;

     let profile_menu = driver.query(By::Css("#react-root > section > nav > div._8MQSO.Cx7Bp > div > div > div.ctQZg > div > div:nth-child(5) > span > img")).first().await?;

     profile_menu.click().await?;

     let profile = driver.query(By::Css("#react-root > section > nav > div._8MQSO.Cx7Bp > div > div > div.ctQZg > div > div:nth-child(5) > div.poA5q > div.uo5MA._2ciX.tWgj8.XWrBI > div._01UL2 > a:nth-child(1) > div")).first().await?;

     profile.click().await?;

     let followers_menu = driver.query(By::Css("#react-root > section > main > div > header > section > ul > li:nth-child(2) > a")).first().await?;
     followers_menu.click().await?;

     sleep(Duration::from_secs(5));
    
     driver.quit().await?;

     Ok(())
}
