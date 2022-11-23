use crate::{scoresaber, Context, Error};
use poise::serenity_prelude::CacheHttp;
use prettytable::{format, row, Cell, Row, Table};

#[poise::command(slash_command)]
pub async fn progress_report(
    ctx: Context<'_>,
    #[description = "Number of Users to Include - Default: 50"] users: Option<i32>,
    #[description = "List of countries separated by commas"] countries: Option<String>,
) -> Result<(), Error> {
    let loading_msg = ctx.send(|m| m.content("Loading...").ephemeral(true)).await;

    let countries_string =
        countries.unwrap_or(std::env::var("COUNTRIES").unwrap_or("".to_string()));
    let mut filter_countries: Option<String> = None;
    if countries_string != "" {
        filter_countries = Some(countries_string);
    }
    let users_count = users.unwrap_or(50);
    let ss_data = scoresaber::get_users(users_count, filter_countries).await?;
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row![c->"Rank", c->"Name", c->"PP", c->"Global"]);

    let mut i = 1;
    for user in ss_data {
        table.add_row(Row::new(vec![
            Cell::new(format!("#{}", i).as_str()),
            Cell::new(&user.name),
            Cell::new(&user.pp.to_string()),
            Cell::new(format!("#{}", &user.rank).as_str()),
        ]));
        i += 1;
    }

    let users_per_page = 25;
    let msg_count = (users_count + (users_per_page - 1)) / users_per_page;
    let table_string = table.to_string();
    let table_rows = table_string.split("\n").collect::<Vec<&str>>();

    let _ = loading_msg.unwrap().delete(ctx).await;

    for i in 0..msg_count {
        let lower_range = if i == 0 {
            0 as usize
        } else {
            (i * users_per_page + 3) as usize
        };
        let upper_range = if i == msg_count - 1 && users_count % users_per_page != 0 {
            (users_count % users_per_page + 3 + (i * users_per_page)) as usize
        } else {
            ((i + 1) * users_per_page + 3) as usize
        };

        let mut out_str = "".to_owned();
        for j in lower_range..upper_range {
            out_str.push_str(table_rows[j]);
            out_str.push_str("\n");
        }

        ctx.channel_id()
            .send_message(ctx.http(), |m| m.content(format!("```{}```", out_str)))
            .await?;
    }
    Ok(())
}
