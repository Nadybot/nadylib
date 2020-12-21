use std::env::var;

#[derive(Clone)]
pub struct AccountData {
    pub username: String,
    pub password: String,
    pub character: String,
}

#[derive(Clone)]
pub struct Config {
    pub port_number: u32,
    pub accounts: Vec<AccountData>,
    pub server_address: String,
    pub spam_bot_support: bool,
    pub send_tells_over_main: bool,
    pub relay_slave_tells: bool,
}

pub fn load_config() -> Option<Config> {
    let mut next_number = 1;
    let mut account_data = Vec::new();

    loop {
        let username = var(format!("SLAVE{}_USERNAME", next_number)).ok();
        let password = var(format!("SLAVE{}_PASSWORD", next_number)).ok();
        let character = var(format!("SLAVE{}_CHARACTERNAME", next_number)).ok();

        if username.is_none() || password.is_none() || character.is_none() {
            break;
        }
        let account = AccountData {
            username: username.unwrap(),
            password: password.unwrap(),
            character: character.unwrap(),
        };
        account_data.push(account);
        next_number += 1;
    }

    let server_address =
        var("SERVER_ADDRESS").unwrap_or_else(|_| String::from("chat.d1.funcom.com:7105"));
    let port_number: u32 = var("PROXY_PORT_NUMBER")
        .unwrap_or_else(|_| String::from("9993"))
        .parse()
        .ok()?;
    let spam_bot_support: bool = var("SPAM_BOT_SUPPORT")
        .unwrap_or_else(|_| String::from("false"))
        .parse()
        .ok()?;
    let send_tells_over_main: bool = var("SEND_TELLS_OVER_MAIN")
        .unwrap_or_else(|_| {
            if account_data.is_empty() {
                String::from("true")
            } else {
                String::from("false")
            }
        })
        .parse()
        .ok()?;
    let relay_slave_tells: bool = var("RELAY_SLAVE_TELLS")
        .unwrap_or_else(|_| String::from("false"))
        .parse()
        .ok()?;

    // We cannot send tells in this case
    if !send_tells_over_main && account_data.is_empty() {
        return None;
    }

    Some(Config {
        port_number,
        accounts: account_data,
        server_address,
        spam_bot_support,
        send_tells_over_main,
        relay_slave_tells,
    })
}
