use crate::{Context, Error, serenity};

/// Displays aagey your or another user's account creation LOL date
#[poise::command(slash_command, prefix_command)]
pub async fn agey(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s agey account was created at {}", u.name, u.created_at());
    ctx.send(poise::CreateReply::default()
    .content(format!("Works for slash and preASFHUSDHUHSDUIFHUSDIHFUISDHUISDFHYUifix commands {}", response))
    .embed(serenity::CreateEmbed::new()
        .title("Much versatile, very wow")
        .description("I need more documentation ok?")
    )
    .ephemeral(true) // this one only applies in application commands though
).await?;
    Ok(())
}