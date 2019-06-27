use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use reqwest::multipart::Form;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{override_if_present, override_option_with_option_none, serde_url};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Expiry {
    #[serde(rename = "1day")]
    OneDay,
    #[serde(rename = "1week")]
    OneWeek,
}

impl Default for Expiry {
    fn default() -> Self {
        Expiry::OneDay
    }
}

impl Display for Expiry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expiry::OneDay => "1day",
                Expiry::OneWeek => "1week",
            }
        )
    }
}

impl FromStr for Expiry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1day" => Ok(Expiry::OneDay),
            "1week" => Ok(Expiry::OneWeek),
            x => Err(format!("Invalid value for --expires: {}. Valid values are 1day, 1week, or NONE (to use default)", x)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(with = "serde_url")]
    pub url: Url,
    pub syntax: Option<String>,
    #[serde(default)]
    pub expires: Expiry,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "bpaste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Overrides url set in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// Filetype for syntax highlighting
    #[structopt(short = "s", long = "syntax", value_name = "filetype|NONE")]
    syntax: Option<String>,
    /// Time to live
    #[structopt(short = "e", long = "expires", value_name = "1day|1week|NONE")]
    expires: Option<String>,
}

pub const NAME: &str = "bpaste";

pub const INFO: &str = r#"Bpaste backend. Supports <https://bpaste.net/>.

Example config block:

    [servers.bpaste]
    backend = "bpaste"
    url = "https://bpaste.net/"

    # Optional values

    # Filetype for syntax highlighting. Default is set by the server.
    syntax = "python"

    # Paste lifetime. Supports "1day" and "1week". Default is 1 day.
    expires = "1week"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.syntax, opt.syntax);

        if let Some(new) = opt.expires {
            if new == "NONE" {
                self.expires = Expiry::default();
            } else {
                self.expires = new.parse().map_err(|x| clap::Error {
                    message: x,
                    kind: clap::ErrorKind::InvalidValue,
                    info: None,
                })?;
            }
        }

        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new()
            .text("code", data)
            .text("expiry", self.expires.to_string());

        let form = match self.syntax {
            Some(ref syntax) => form.text("lexer", syntax.to_owned()),
            None => form.text("lexer", "text".to_owned()),
        };

        let res = Client::new()
            .post(self.url.clone())
            .multipart(form)
            .send()?
            .error_for_status()?;

        if res.url() == &self.url {
            Err("Paste failed.\nCheck parameters, it is possible that the syntax name provided wasn't recognized.".to_owned().into())
        } else {
            Ok(res.url().to_owned())
        }
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "dpaste | {}", self.url)
    }
}
