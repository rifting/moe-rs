use poise::serenity_prelude::Colour;
use poise::serenity_prelude::builder::{CreateAttachment, CreateEmbedFooter};

use crate::{Context, Error, serenity};

/// happy
#[poise::command(slash_command, prefix_command)]
pub async fn happy(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let attachment = CreateAttachment::path("assets/happy.png").await.expect("Could not find image");
    let footer_attachment = CreateAttachment::path("assets/avatar.png").await.expect("Could not find image");
    
    let loading_message = ctx.channel_id().say(&ctx.http(), "wack wack wo...").await;

    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title("*laughs*")
            .image("attachment://happy.png")
            .color(Colour::from(0x250c3a).tuple())
            .footer(CreateEmbedFooter::new(
                "existing.moe - RUST - v1.0.0 -"
            ).icon_url("attachment://avatar.png")
          )
        )
        .attachment(attachment)
        .attachment(footer_attachment)


    // .ephemeral(true) // this one only applies in application commands though
).await?;
    let _ = loading_message.unwrap().delete(&ctx.http()).await;
    Ok(())
}