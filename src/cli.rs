use std::collections::HashMap;
use inquire::{Select, Password, InquireError};
use inquire::ui::{RenderConfig, Color, Styled};

pub fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();
    render_config.highlighted_option_prefix = Styled::new("→");
    render_config.error_message = render_config
    .error_message
    .with_prefix(Styled::new("💀").with_fg(Color::LightRed));
    render_config
}

pub fn get_user_input() -> HashMap<String, String> {
    let base_url_options = vec![
        "http://localhost:50231", 
        "https://test.awardco.com", 
        "https://api.awardco.com"
    ];
    let report_name_options = vec![
        "Money Deposited",
        "Money Account",
        "Money Spent",
        "Points Redeemed",
        "Recognition Details",
        "Recognition Status",
        "Users",
        "Login and Engagement"
    ];
    let mut choices = HashMap::new();

    let base_url_response: Result<&str, InquireError> = Select::new("Select base URL:", base_url_options).prompt();
    if let Ok(choice) = base_url_response {
        choices.insert(String::from("base_url"), choice.to_string());
    } else {
        println!("Error while asking for base URL, please try again.");
    }

    let api_key_response = Password::new("API key:").prompt();
    if let Ok(choice) = api_key_response {
        choices.insert(String::from("api_key"), choice);
    } else {
        println!("Error while asking for API key, please try again.");
    }

    let report_name_response: Result<&str, InquireError> = Select::new("Select report name:", report_name_options).prompt();
    if let Ok(choice) = report_name_response {
        choices.insert(String::from("report_name"), choice.to_string());
    } else {
        println!("Error while asking for report name, please try again.");
    }

    return choices;
}