use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct DiscordWebhook<'a> {
    #[serde(flatten)]
    pub inner: DiscordInner<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<DiscordMention<'a>>,
}

#[serde(rename_all = "lowercase")]
#[derive(Serialize, Debug)]
pub enum DiscordInner<'a> {
    Content(&'a str),
    // File(&[u8]), // with multipart/form-data
    Embeds(Vec<DiscordEmbed<'a>>),
}

#[derive(Serialize, Debug)]
pub struct DiscordMention<'a> {
    pub parse: &'a [MentionType],
    pub roles: &'a [&'a str],
    pub users: &'a [&'a str],
}

#[serde(rename_all = "lowercase")]
#[derive(Serialize, Debug)]
pub enum MentionType {
    Roles,
    Users,
    Everyone,
}

#[derive(Serialize, Debug)]
pub struct DiscordEmbed<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub timestamp: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<DiscordAuthor<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<DiscordFooter<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<DiscordField<'a>>>, // @TODO https://discordapp.com/developers/docs/resources/channel#embed-object
}

#[derive(Serialize, Debug)]
pub struct DiscordFooter<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<&'a str>,
}

#[derive(Serialize, Debug)]
pub struct DiscordAuthor<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<&'a str>,
}

#[derive(Serialize, Debug)]
pub struct DiscordField<'a> {
    pub name: &'a str,
    pub value: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}
