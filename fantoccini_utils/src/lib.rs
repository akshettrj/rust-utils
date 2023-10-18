use error_utils::BaseError;

use fantoccini::{
    elements::{Element, Form},
    Client, ClientBuilder, Locator,
};
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Clone, Debug, Deserialize, EnumString, Serialize)]
pub enum BrowserType {
    #[strum(to_string = "chrome")]
    #[serde(rename = "chrome")]
    Chrome,
    #[strum(to_string = "firefox")]
    #[serde(rename = "firefox")]
    Firefox,
}

pub async fn get_client<E: BaseError>(
    headless: bool,
    browser: &BrowserType,
    url: &str,
) -> Result<Client, E> {
    let mut capabilities = serde_json::map::Map::new();
    let mut args = match browser {
        BrowserType::Chrome => {
            vec![
                "--disable-background-networking",
                "--disable-background-timer-throttling",
                "--disable-backgrounding-occluded-windows",
                "--disable-breakpad",
                "--disable-client-side-phishing-detection",
                "--disable-default-apps",
                "--disable-dev-shm-usage",
                "--disable-extensions",
                "--disable-hang-monitor",
                "--disable-ipc-flooding-protection",
                "--disable-popup-blocking",
                "--disable-prompt-on-repost",
                "--disable-renderer-backgrounding",
                "--disable-sync",
                "--metrics-recording-only",
                "--safebrowsing-disable-auto-update",
                "--disable-gpu",
                "--enable-automation=false",
                "--use-mock-keychain",
                "--disable-crash-reporting",
                "--disable-features=site-per-process,Translate,BlinkGenPropertyTrees",
                "--enable-features=NetworkService,NetworkServiceInProcess",
                "--force-color-profile=srgb",
                "--password-store=basic",
                "--no-first-run",
                "--no-default-browser-check",
                "--disk-cache-dir=/tmp",
                "--user-data-dir=/tmp",
                "--crash-dumps-dir=/tmp",
            ]
        }
        BrowserType::Firefox => {
            vec![]
        }
    };
    if headless {
        args.push("--headless");
    };

    capabilities.insert(
        match browser {
            BrowserType::Chrome => "goog:chromeOptions",
            BrowserType::Firefox => "moz:firefoxOptions",
        }
        .to_string(),
        serde_json::json!({
            "args": args,
        }),
    );

    ClientBuilder::native()
        .capabilities(capabilities)
        .connect(url)
        .await
        .map_err(|err| {
            E::with_description_and_error("failed to build fatoccini client", Some(err.to_string()))
        })
}

pub async fn navigate<E: BaseError>(client: &Client, url: &str, name: &str) -> Result<(), E> {
    client.goto(url).await.map_err(|err| {
        E::with_description_and_error(
            format!("failed to navigate to {}", name),
            Some(err.to_string()),
        )
    })
}

pub async fn get_element<'a, E: BaseError>(
    client: &Client,
    locator: Locator<'a>,
    name: &str,
) -> Result<Element, E> {
    client.find(locator).await.map_err(|err| {
        E::with_description_and_error(
            format!("failed to get element: {}", name),
            Some(err.to_string()),
        )
    })
}

pub async fn take_screenshot<'a, E: BaseError>(
    client: &Client,
    locator: Locator<'a>,
    name: &str,
) -> Result<Vec<u8>, E> {
    let element = get_element(client, locator, name).await?;
    element.screenshot().await.map_err(|err| {
        E::with_description_and_error(
            format!("failed to take screenshot of {}", name),
            Some(err.to_string()),
        )
    })
}

pub async fn wait_until_element<'a, E: BaseError>(
    client: &Client,
    locator: Locator<'a>,
    name: &str,
) -> Result<(), E> {
    client.wait().for_element(locator).await.map_err(|err| {
        E::with_description_and_error(
            format!("failed to wait for {}", name),
            Some(err.to_string()),
        )
    })?;
    Ok(())
}

pub async fn wait_until_navigated<E: BaseError>(
    client: &Client,
    target_url: &str,
) -> Result<(), E> {
    let target_url = url::Url::parse(target_url).map_err(|err| {
        E::with_description_and_error("failed to parse target_url", Some(err.to_string()))
    })?;
    client.wait().for_url(target_url).await.map_err(|err| {
        E::with_description_and_error("failed to wait for target_url", Some(err.to_string()))
    })?;
    Ok(())
}

pub async fn get_form<'a, E: BaseError>(
    client: &Client,
    locator: Locator<'a>,
    name: &str,
) -> Result<Form, E> {
    client.form(locator).await.map_err(|err| {
        E::with_description_and_error(
            format!("failed to get form: {}", name),
            Some(err.to_string()),
        )
    })
}

pub async fn set_in_form<'a, E: BaseError>(
    form: &Form,
    locator: Locator<'a>,
    value: &str,
    name: &str,
) -> Result<(), E> {
    form.set(locator, value).await.map_err(|err| {
        E::with_description_and_error(
            format!("failed to set value for {} in form", name),
            Some(err.to_string()),
        )
    })?;
    Ok(())
}

pub async fn set_in_form_by_name<'a, E: BaseError>(
    form: &Form,
    input_name: &str,
    value: &str,
    name: &str,
) -> Result<(), E> {
    form.set_by_name(input_name, value).await.map_err(|err| {
        E::with_description_and_error(
            format!("failed to set value for {} in form", name),
            Some(err.to_string()),
        )
    })?;
    Ok(())
}

pub async fn submit_form<E: BaseError>(form: &Form, name: &str) -> Result<(), E> {
    form.submit().await.map_err(|err| {
        E::with_description_and_error(
            format!("failed to submit {} form", name),
            Some(err.to_string()),
        )
    })
}

pub async fn get_attribute_of_element<E: BaseError>(
    element: &Element,
    attribute: &str,
    name: &str,
) -> Result<Option<String>, E> {
    element.attr(attribute).await.map_err(|err| {
        E::with_description_and_error(
            format!("failed to get attribute '{}' for {}", attribute, name),
            Some(err.to_string()),
        )
    })
}

pub async fn get_inner_text_of_element<E: BaseError>(
    element: &Element,
    name: &str,
) -> Result<Option<String>, E> {
    let inner_text = element
        .text()
        .await
        .map_err(|err| {
            E::with_description_and_error(
                format!("failed to get inner text of {}", name),
                Some(err.to_string()),
            )
        })?
        .trim()
        .to_string();

    if inner_text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(inner_text))
    }
}

pub async fn get_inner_text<'a, E: BaseError>(
    client: &Client,
    locator: Locator<'a>,
    name: &str,
) -> Result<Option<String>, E> {
    let element = get_element(client, locator, name).await?;
    get_inner_text_of_element(&element, name).await
}
