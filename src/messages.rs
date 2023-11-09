pub fn get_welcome_message() -> String {
    "ZuzaRule: Crowdsourced Governance at Your Fingertips\nOur platform is where community consensus builds the foundation of collaboration\nStay updated, propose changes, and have a direct hand in sculpting the environment you participate in, all within your Telegram group\nEmbrace the power of collective decision making with ZuzaRule"
        .to_string()
}

pub fn get_new_proposal_message(
    title: &str,
    description: &str,
    starting_date: &str,
    expiration_date: &str,
) -> String {
    let text = format!(
        "Create your proporsal below:\nTitle:{}\nDescription:{}\nStarting Date:{}\nExpiration date:{}\n",
        title, description, starting_date, expiration_date
    );
    text
}
