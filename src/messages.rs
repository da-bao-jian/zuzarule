use regex::Regex;
pub fn get_welcome_message() -> String {
    "ZuzaRule: Crowdsourced Governance at Your Fingertips\nOur platform is where community consensus builds the foundation of collaboration\nStay updated, propose changes, and have a direct hand in sculpting the environment you participate in, all within your Telegram group\nEmbrace the power of collective decision making with ZuzaRule"
        .to_string()
}

pub fn get_new_proposal_message() -> String {
    let template =
        "Create your proposal below:\nTitle: \nDescription: \nStarting Date: \nExpiration Date: \n";
    template.to_string()
}

struct MessageFields<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub starting_date: Option<&'a str>,
    pub expiration_date: Option<&'a str>,
}

fn fill_in_message_template(template: &str, fields: MessageFields) -> String {
    let mut message = template.to_string();

    let replace_placeholder = |message: &mut String, field_name: &str, value: Option<&str>| {
        if let Some(val) = value {
            let pattern = format!(r"{}: \n", field_name);
            let re = Regex::new(&pattern).unwrap();
            *message = re
                .replace(message, format!("{}: {}\n", field_name, val))
                .to_string();
        }
    };

    replace_placeholder(&mut message, "Title", fields.title);
    replace_placeholder(&mut message, "Description", fields.description);
    replace_placeholder(&mut message, "Starting Date", fields.starting_date);
    replace_placeholder(&mut message, "Expiration Date", fields.expiration_date);

    message
}

pub fn parse_message(
    msg: &str,
    title: Option<&str>,
    description: Option<&str>,
    starting_date: Option<&str>,
    expiration_date: Option<&str>,
) -> String {
    let message_field = MessageFields {
        title,
        description,
        starting_date,
        expiration_date,
    };
    let filled_in_message = fill_in_message_template(msg, message_field);
    filled_in_message
}
