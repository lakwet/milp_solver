use super::super::forms::slack::{
    InitializationResult, SimplexResult, SlackFormLP,
};

pub fn simplex_lp_chvatal(
    slack_lp: &mut SlackFormLP,
) -> Result<SimplexResult, String> {
    let init_result = slack_lp.initialize_simplex()?;

    println!("{}", slack_lp);
    if init_result == InitializationResult::Unfeasible {
        return Ok(SimplexResult::Unfeasible);
    }

    slack_lp.find_optimal()
}
