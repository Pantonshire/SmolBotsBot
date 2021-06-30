use goldcrest::data::{Tweet, tweet::TweetTextOptions};
use sbb_data::new::{NewRobotGroup, NewRobot};

use crate::parse::*;

pub fn parse_tweet<F, T>(tweet: &Tweet, handler: F) -> Option<T> where F: Fn(NewRobotGroup, Vec<Robot>) -> T {
    let text_opts = TweetTextOptions::all()
        .media(false)
        .urls(false);

    let text = tweet.text(text_opts);

    let (robots, body, cw) = parse_group(&text)?;

    let image = tweet.media
        .iter()
        .filter(|&media| {
            media.media_type == "photo" || media.media_type == "animated_gif" || media.media_type == "video"
        })
        .next()?;

    let image_url = image.media_url.as_str();

    let alt = {
        let alt = image.alt.trim();
        if alt.is_empty() {
            None
        } else {
            Some(alt)
        }
    };

    let group = NewRobotGroup{
        tweet_id: tweet.id as i64,
        tweet_time: tweet.created_at,
        image_url,
        body: body.trim_end(),
        alt,
        content_warning: cw,
    };
    
    Some(handler(group, robots))
}

pub fn new_robot<F, T>(robot: &Robot, group_id: i64, handler: F) -> T where F: Fn(NewRobot) -> T {
    let identifier = robot.name.identifier();
    handler(NewRobot{
        robot_group_id: group_id,
        robot_number: robot.number,
        prefix: robot.name.prefix,
        suffix: robot.name.suffix,
        plural: robot.name.plural,
        ident: &identifier,
    })
}
