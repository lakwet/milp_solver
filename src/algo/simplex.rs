use super::super::forms::slack::{SimplexResult, SimplexRound, SlackFormLP};

pub fn simplex_lp_chvatal(slack_lp: &mut SlackFormLP) -> Result<SimplexResult, String> {
    let mut round_count = 0;
    loop {
        round_count += 1;
        match slack_lp.find_entering_and_leaving() {
            Ok(SimplexRound::Unbounded) => {
                return Ok(SimplexResult::Unbounded);
            },
            Ok(SimplexRound::Finished) => {
                break;
            },
            Ok(SimplexRound::Switch(col, row)) => {
                slack_lp.pivot(col, row);
            },
            Err(msg) => {
                return Err(msg);
            },
        }
    }

    println!("Round count: {}", round_count);

    Ok(slack_lp.compute_solution_vector())
}
