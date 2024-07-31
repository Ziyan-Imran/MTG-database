use postgres::{error, Client, Config, GenericClient, NoTls};
use std::{
    convert::From,
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

pub struct MtgConfigs {
    // Config to hold .env values
    pub username: String,
    pub password: String,
    pub db_connection: String,
    pub ip_addr: String,
}

impl MtgConfigs {
    pub fn get_field(&self, key: String) -> &String {
        if key == "username" {
            return &self.username;
        } else if key == "password" {
            return &self.password;
        } else if key == "db_connection" {
            return &self.db_connection;
        } else if key == "ip_addr" {
            return &self.ip_addr;
        } else {
            panic!("Wrong Key");
        }
    }
}

pub fn initialize_db() -> Result<(), postgres::Error> {
    let mut client = Client::connect("postgresql://zimran:%mpEz3Q^@172.25.244.6/mtg", NoTls)?;

    client.batch_execute(
        "
      CREATE TABLE IF NOT EXISTS mtg_cards (
        name                                SERIAL PRIMARY KEY,
        set                                 VARCHAR NOT NULL,
        set_code                            VARCHAR NOT NULL,
        id                                  INTEGER,
        card_type                           VARCHAR NOT NULL,
        power                               INTEGER,
        toughness                           INTEGER,
        loyalty                             INTERGER,
        defense                             INTEGER,
        manacost                            INTEGER,
        converted_manacost                  INTEGER,
        artist                              VARCHAR NOT NULL,
        flavor                              VARCHAR,
        color                               VARCHAR,
        generated_mana                      VARCHAR,
        number                              INTEGER,
        rarity                              VARCHAR,
        rating                              INTEGER,
        ruling                              VARCHAR,
        variation                           VARCHAR,
        variation_local                     VARCHAR,
        ability                             VARCHAR,
        pricing_euro                        INTEGER,
        pricing_usd                         INTEGER,
        pricing_txt                         INTEGER,
        watermark                           VARCHAR,
        print_number                        INTEGER,
        is_original                         INTEGER,
        back_id                             INTEGER,
        number_int                          INTEGER,
        name_cn                             VARCHAR,
        name_tw                             VARCHAR,
        name_fr                             VARCHAR,
        name_de                             VARCHAR,
        name_it                             VARCHAR,
        name_jp                             VARCHAR,
        name_pt                             VARCHAR,
        name_ru                             VARCHAR,
        name_es                             VARCHAR,
        name_ko                             VARCHAR,
        type_cn                             VARCHAR,
        type_tw                             VARCHAR,
        type_fr                             VARCHAR,
        type_de                             VARCHAR,
        type_it                             VARCHAR,
        type_jp                             VARCHAR,
        type_pt                             VARCHAR,
        type_ru                             VARCHAR,
        type_es                             VARCHAR,
        type_ko                             VARCHAR,
        ability_cn                          VARCHAR,
        ability_tw                          VARCHAR,
        ability_fr                          VARCHAR,
        ability_de                          VARCHAR,
        ability_it                          VARCHAR,
        ability_jp                          VARCHAR,
        ability_pt                          VARCHAR,
        ability_ru                          VARCHAR,
        ability_es                          VARCHAR,
        ability_ko                          VARCHAR,
        flavor_cn                           VARCHAR,
        flavor_tw                           VARCHAR,
        flavor_fr                           VARCHAR,
        flavor_de                           VARCHAR,
        flavor_it                           VARCHAR,
        flavor_jp                           VARCHAR,
        flavor_pt                           VARCHAR,
        flavor_ru                           VARCHAR,
        flavor_es                           VARCHAR,
        flavor_ko                           VARCHAR,
        color_identify                      VARCHAR,
        fully_handled                       VARCHAR,
        custom_sort                         VARCHAR,
        is_special                          VARCHAR,
        card_status                         VARCHAR,
        associated_id                       VARCHAR,
        legality_standard                   VARCHAR,
        legality_pioneer                    VARCHAR,
        legality_modern                     VARCHAR,
        legality_legacy                     VARCHAR,
        legality_highlander                 VARCHAR,
        legality_duel_commander             VARCHAR,
        legality_tiny_leaders_commander     VARCHAR,
        legality_commander:                 VARCHAR,
        legality_peasant:                   VARCHAR,
        legality_pauper:                    VARCHAR
        )
    ",
    )?;

    Ok(())
}

pub fn get_env_values() -> Result<MtgConfigs, Box<dyn std::error::Error>> {
    // Get values from env variable
    dotenvy::dotenv()?;

    let user_configs = MtgConfigs {
        username: match dotenvy::var("USERNAME") {
            Ok(username) => username,
            Err(error) => {
                println!("{error}");
                String::from("not found")
            }
        },
        password: match dotenvy::var("PASSWORD") {
            Ok(password) => password,
            Err(error) => {
                println!("{error}");
                String::from("not found")
            }
        },
        db_connection: match dotenvy::var("DB_CONNECTION") {
            Ok(db_connection) => db_connection,
            Err(error) => {
                println!("{error}");
                String::from("not found")
            }
        },
        ip_addr: match dotenvy::var("IP_ADDR") {
            Ok(addresses) => addresses,
            Err(error) => {
                println!("{error}");
                String::from("not found")
            }
        },
    };
    Ok(user_configs)
}

pub fn check_connection() -> Result<bool, Box<dyn std::error::Error>> {
    // Checks if the connection to the database was already created

    // Get env values to build Client configs
    let user_configs = get_env_values()?;
    let mut db_config: Config = Config::new();

    db_config.user(user_configs.get_field("username".to_string()));
    db_config.password(user_configs.get_field("password".to_string()));
    db_config.dbname(user_configs.get_field("db_connection".to_string()));
    db_config.host("localhost");
    // Convert string of IP address to a Vector of IpAddr
    let mut connection = db_config.connect(NoTls)?;

    let timeout_duration = Duration::new(5, 0);
    let connection_result = connection.is_valid(timeout_duration)?;
    match connection_result {
        () => Ok(true),
        _ => Ok(false),
    }
}
