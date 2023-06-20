use nom::{
    IResult,
    bytes::complete::{take_until, tag, take_while},
    sequence::{terminated, Tuple, preceded, delimited},
    multi::many0,
    character::complete::{char, multispace0},
    Parser, combinator::opt
};

use crate::SearchResult;

fn parse_bracket_balanced(input: &str) -> IResult<&str, &str> {
    let mut chars = input.chars().enumerate();
    let (mut depth, mut end) = match chars.next() {
        Some((_, '[')) => (1, None),
        _ => return Err(nom::Err::Error(
            nom::error::Error::new(input, nom::error::ErrorKind::Char)
        ))
    };

    for (i, c) in chars {
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(i);
                    break;
                }
            }
            _ => (),
        }
    }
    if let Some(end) = end {
        Ok((&input[end + 1..], &input[1..end]))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )))
    }
}

fn parse_string_vec(input: &str) -> IResult<&str, Vec<String>> {
    let (input, strings) = parse_bracket_balanced(input)?;
    Ok((
        input,
        strings
        .split(",")
        .map(str::trim)
        .map(Into::into)
        .collect()
    ))
}

fn gobble_around_string_vec(input: &str) -> IResult<&str, Vec<String>> {
    delimited(
        take_until("["),
        parse_string_vec,
        multispace0
    )(input)
}

fn gobble_around_true_map_map(input: &str) -> IResult<&str, Vec<String>> {
    delimited(
        take_until("True map path:")
        .and(tag("True map path:"))
        .and(multispace0),
        parse_string_vec,
        multispace0
    )(input)
}

fn gobble_around_events(input: &str) -> IResult<&str, Vec<String>> {
    delimited(
        take_until("Events:")
        .and(tag("Events:"))
        .and(multispace0),
        parse_string_vec,
        multispace0
    )(input)
}

fn parse_floor(input: &str) -> IResult<&str, (String, Vec<String>)> {
    (
        preceded(
            tag("Floor "),
            take_while(char::is_numeric)
            .map(Into::into)
        ),
        preceded(
            tag(": "),
            parse_string_vec
        )
    )
    .parse(input)
}

fn parse_card_choices(input: &str) -> IResult<&str, Vec<(String, Vec<String>)>> {
    let (input, card_choices) = preceded(
        tag("Card choices:").and(multispace0),
        take_until("Potions:")
    )(input)?;
    let (_, card_choices) = many0(
        terminated(
            parse_floor,
            multispace0
        )
    )(card_choices)?;
    Ok((input, card_choices))
}

fn parse_potions(input: &str) -> IResult<&str, Vec<(String, Vec<String>)>> {
    let (input, potions) = preceded(
        tag("Potions:").and(multispace0),
        take_until("Other cards:")
    )(input)?;
    let (_, potions) = many0(
        terminated(
            parse_floor,
            multispace0
        )
    )(potions)?;
    Ok((input, potions))
}

fn gobble_around_raw_relic_lists(input: &str) -> IResult<&str, [Vec<String>; 5]> {
    let (input, commons) = preceded(
        take_until("Raw common relic list:")
        .and(tag("Raw common relic list:"))
        .and(multispace0),
        parse_string_vec
    )(input)?;
    let (input, uncommons) = preceded(
        take_until("Raw uncommon relic list:")
        .and(tag("Raw uncommon relic list:"))
        .and(multispace0),
        parse_string_vec
    )(input)?;
    let (input, rares) = preceded(
        take_until("Raw rare relic list:")
        .and(tag("Raw rare relic list:"))
        .and(multispace0),
        parse_string_vec
    )(input)?;
    let (input, bosses) = preceded(
        take_until("Raw boss relic list:")
        .and(tag("Raw boss relic list:"))
        .and(multispace0),
        parse_string_vec
    )(input)?;
    let (input, shops) = preceded(
        take_until("Raw shop relic list:")
        .and(tag("Raw shop relic list:"))
        .and(multispace0),
        parse_string_vec
    )(input)?;
    Ok((input, [commons, uncommons, rares, bosses, shops]))

}

pub(super) fn parse_search_results(input: &str) -> IResult<&str, Vec<SearchResult>> {
    let (input, _) = take_until("Seed:")(input)?;
    let (input, (search_results, _)) = (
        many0(terminated(
            parse_search_result,
            multispace0
        )),
        opt(parse_seed_list)
    ).parse(input)?;
    Ok((input, search_results))
}

fn parse_search_result(input: &str) -> IResult<&str, SearchResult> {
    let (input, leftover) = terminated(
        take_until("#####################################"),
        tag("#####################################")
    )(input)?;
    
    let (leftover, (seed_string, seed)) = (
        preceded(
            tag("Seed: "),
            take_while(|c: char| !c.is_whitespace())
        ),
        delimited(
            tag(" ("),
            take_while(char::is_numeric),
            char(')').and(multispace0)
        )
    ).parse(leftover)?;

    let (leftover, neow_options) = parse_neow_options(leftover)?;
    let (leftover, combats) = gobble_around_string_vec(leftover)?;
    let (leftover, bosses) = gobble_around_string_vec(leftover)?;
    let (leftover, events) = gobble_around_events(leftover)?;
    let (leftover, true_map_path) = gobble_around_true_map_map(leftover)?;
    let (leftover, card_choices) = parse_card_choices(leftover)?;
    let (leftover, potions) = parse_potions(leftover)?;
    let (
        leftover,
        [
            common_relics,
            uncommon_relics,
            rare_relics,
            boss_relics,
            shop_relics,
        ]
    ) = gobble_around_raw_relic_lists(leftover)?;

    Ok((
        input,
        SearchResult {
            seed_string: seed_string.into(),
            seed: seed.into(),
            neow_options: neow_options,
            combats: combats,
            bosses: bosses,
            events: events,
            true_map_path,
            card_choices,
            potions,
            common_relics,
            uncommon_relics,
            rare_relics,
            boss_relics,
            shop_relics,
            leftover: leftover.into(),
        }
    ))
}

fn parse_neow_options(input: &str) -> IResult<&str, Vec<String>> {
    let (leftover, neow_options) = delimited(
        tag("Neow Options:").and(multispace0),
        take_until("[ Lose your starting Relic Obtain a random boss Relic ]"),
        tag("[ Lose your starting Relic Obtain a random boss Relic ]").and(multispace0)
    )(input)?;
    let neow_options = neow_options
    .split(']')
    .map(|s| {
        s.trim()
        .trim_start_matches('[')
        .trim()
    })
    .filter(|s| !s.is_empty())
    .map(Into::into)
    .collect();

    Ok((leftover, neow_options))
}

fn parse_seed_list(input: &str) -> IResult<&str, Vec<String>> {
    let (input, prefix) = (
        take_while(char::is_numeric),
        tag(" seeds found:"),
        multispace0
    ).parse(input)?;
    parse_string_vec(input)
}

#[cfg(test)]
mod test_parse_search_results {
    use super::parse_search_results;
    use crate::{_DEFAULT_HOME, GameHome};
    use std::path::PathBuf;

    #[test]
    fn parse_default_search_results() {
        let home: GameHome = PathBuf::from(_DEFAULT_HOME)
        .try_into().unwrap();
        let search = home.search().unwrap();
        let results = parse_search_results(&search).unwrap();
        dbg!(results);
    }
}