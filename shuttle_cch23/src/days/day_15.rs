//!day_15.rs

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::fmt;

pub fn get_routes() -> Router {
    Router::new()
        .route("/15/nice", post(naughty_or_nice))
        .route("/15/game", post(game_of_the_year))
}

#[derive(Deserialize)]
struct NONInput {
    input: String,
}

impl NONInput {
    fn check_vowels(&self) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        self.input.matches(&vowels[..]).count() >= 3
    }

    fn check_one_letter_twice(&self) -> bool {
        let mut letters = self.input.chars();
        let mut prev_letter = match letters.next() {
            Some(c) => c,
            None => return false,
        };
        for c in letters {
            if c == prev_letter && c.is_ascii_alphabetic() {
                return true;
            }
            prev_letter = c;
        }
        false
    }

    fn check_substrings(&self) -> bool {
        let forbidden_substring = ["ab", "cd", "pq", "xy"];
        for pat in forbidden_substring.into_iter() {
            match self.input.matches(pat).next() {
                Some(_) => return false,
                None => (),
            }
        }
        true
    }

    fn rule_one_broken(&self) -> bool {
        self.input.chars().count() < 8
    }

    fn rule_two_broken(&self) -> bool {
        let mut has_oppercase = false;
        let mut has_lowercase = false;
        let mut has_digits = false;
        for c in self.input.chars() {
            has_oppercase = has_oppercase || c.is_ascii_uppercase();
            has_lowercase = has_lowercase || c.is_ascii_lowercase();
            has_digits = has_digits || c.is_ascii_digit();
        }
        !(has_oppercase && has_lowercase && has_digits)
    }

    fn rule_three_broken(&self) -> bool {
        self.input.chars().filter(|c| c.is_ascii_digit()).count() < 5
    }

    fn rule_four_broken(&self) -> bool {
        let mut digits: Vec<u32> = Vec::new();
        let mut current_digit = String::new();
        let mut last_is_digit = false;
        for c in self.input.chars() {
            if c.is_ascii_digit() {
                current_digit.push(c);
                last_is_digit = true;
            } else {
                if last_is_digit {
                    digits.push(current_digit.parse::<u32>().expect("parse error"));
                    current_digit = "".into();
                    last_is_digit = false;
                }
            }
        }
        digits.iter().sum::<u32>() != 2023
    }

    fn rule_five_broken(&self) -> bool {
        let mut chars = self.input.chars();
        let mut last_last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        let mut last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        let mut found_joy = false;
        for c in self.input.chars() {
            if c.is_alphabetic() {
                match (last_last_c, last_c, c) {
                    (_, 'j', 'o') => (),
                    (_, _, 'o') => return true,
                    ('j', 'o', 'y') => found_joy = true,
                    (_, _, 'y') | ('o', 'y', 'j') | ('y', _, 'j') => return true,
                    (_, _, 'j') => (),
                    (_, 'j', _) => return true,
                    _ => (),
                }
                last_last_c = last_c;
                last_c = c;
            }
        }
        !found_joy
    }

    fn rule_six_broken(&self) -> bool {
        let mut chars = self.input.chars();
        let mut last_last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        let mut last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        for c in chars {
            if c == last_last_c && c.is_alphabetic() && last_c.is_alphabetic() {
                return false;
            }
            last_last_c = last_c;
            last_c = c;
        }
        true
    }

    fn rule_seven_broken(&self) -> bool {
        for c in self.input.chars() {
            match c {
                '\u{2980}'..='\u{2BFF}' => return false,
                _ => (),
            }
        }
        true
    }

    fn rule_eight_broken(&self) -> bool {
        for c in self.input.chars() {
            match emojis::get(c.to_string().as_str()) {
                Some(_) => return false,
                None => (),
            }
        }
        true
    }

    fn rule_nine_broken(&self) -> bool {
        !digest(&self.input).ends_with('a')
    }
}

#[derive(Serialize)]
struct NONOutput {
    result: String,
}

impl NONOutput {
    fn nice() -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(Self {
                result: "nice".into(),
            }),
        )
    }

    fn naughty() -> (StatusCode, Json<Self>) {
        (
            StatusCode::BAD_REQUEST,
            Json(Self {
                result: "naughty".into(),
            }),
        )
    }
}

async fn naughty_or_nice(Json(input): Json<NONInput>) -> impl IntoResponse {
    if input.check_vowels() && input.check_one_letter_twice() && input.check_substrings() {
        NONOutput::nice()
    } else {
        NONOutput::naughty()
    }
}

enum Rules {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    None,
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rules::One => write!(f, "8 chars"),
            Rules::Two => write!(f, "more types of chars"),
            Rules::Three => write!(f, "55555"),
            Rules::Four => write!(f, "math is hard"),
            Rules::Five => write!(f, "not joyful enough"),
            Rules::Six => write!(f, "illegal: no sandwich"),
            Rules::Seven => write!(f, "outranged"),
            Rules::Eight => write!(f, "😳"),
            Rules::Nine => write!(f, "not a coffee brewer"),
            Rules::None => write!(f, "that's a nice password"),
        }
    }
}

impl Rules {
    fn statuscode(&self) -> StatusCode {
        match self {
            Rules::One => StatusCode::BAD_REQUEST,
            Rules::Two => StatusCode::BAD_REQUEST,
            Rules::Three => StatusCode::BAD_REQUEST,
            Rules::Four => StatusCode::BAD_REQUEST,
            Rules::Five => StatusCode::NOT_ACCEPTABLE,
            Rules::Six => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            Rules::Seven => StatusCode::RANGE_NOT_SATISFIABLE,
            Rules::Eight => StatusCode::UPGRADE_REQUIRED,
            Rules::Nine => StatusCode::IM_A_TEAPOT,
            Rules::None => StatusCode::OK,
        }
    }
}

#[derive(Serialize)]
struct NONOutputTask2 {
    result: String,
    reason: String,
}

impl NONOutputTask2 {
    fn nice() -> (StatusCode, Json<Self>) {
        (
            Rules::None.statuscode(),
            Json(Self {
                result: "nice".into(),
                reason: format!("{}", Rules::None),
            }),
        )
    }

    fn naughty(broken_rule: Rules) -> (StatusCode, Json<Self>) {
        (
            broken_rule.statuscode(),
            Json(Self {
                result: "naughty".into(),
                reason: format!("{}", broken_rule),
            }),
        )
    }
}

async fn game_of_the_year(Json(input): Json<NONInput>) -> impl IntoResponse {
    if input.rule_one_broken() {
        return NONOutputTask2::naughty(Rules::One);
    }
    if input.rule_two_broken() {
        return NONOutputTask2::naughty(Rules::Two);
    }
    if input.rule_three_broken() {
        return NONOutputTask2::naughty(Rules::Three);
    }
    if input.rule_four_broken() {
        return NONOutputTask2::naughty(Rules::Four);
    }
    if input.rule_five_broken() {
        return NONOutputTask2::naughty(Rules::Five);
    }
    if input.rule_six_broken() {
        return NONOutputTask2::naughty(Rules::Six);
    }
    if input.rule_seven_broken() {
        return NONOutputTask2::naughty(Rules::Seven);
    }
    if input.rule_eight_broken() {
        return NONOutputTask2::naughty(Rules::Eight);
    }
    if input.rule_nine_broken() {
        return NONOutputTask2::naughty(Rules::Nine);
    }

    NONOutputTask2::nice()
}
