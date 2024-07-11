use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum AutopilotStakeAction {
    LiquidStake,
    RedeemStake,
}

#[cw_serde]
pub struct AutopilotStakeIbc {
    pub action: AutopilotStakeAction,
    pub ibc_receiver: String,
}

#[cw_serde]
#[serde(untagged)]
pub enum AutopilotAction {
    Stakeibc {
        receiver: String,
        stakeibc: AutopilotStakeIbc,
    },
    Claim {
        receiver: String,
        claim: Empty,
    },
}

#[cw_serde]
pub struct AutopilotMsg {
    pub autopilot: AutopilotAction,
}

pub enum Action {
    LiquidStake,
    RedeemStake,
    Claim,
}

const MISSING_IBC_RECEIVER_MSG: &str = "Action requires ibc receiver";

pub fn get_autopilot_msg(
    action: Action,
    receiver: &str,
    ibc_receiver: Option<String>,
) -> AutopilotMsg {
    let autopilot = match action {
        Action::LiquidStake => AutopilotAction::Stakeibc {
            receiver: receiver.to_string(),
            stakeibc: AutopilotStakeIbc {
                action: AutopilotStakeAction::LiquidStake,
                ibc_receiver: ibc_receiver.expect(MISSING_IBC_RECEIVER_MSG),
            },
        },
        Action::RedeemStake => AutopilotAction::Stakeibc {
            receiver: receiver.to_string(),
            stakeibc: AutopilotStakeIbc {
                action: AutopilotStakeAction::RedeemStake,
                ibc_receiver: ibc_receiver.expect(MISSING_IBC_RECEIVER_MSG),
            },
        },
        Action::Claim => AutopilotAction::Claim {
            receiver: receiver.to_string(),
            claim: Empty::default(),
        },
    };
    AutopilotMsg { autopilot }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECEIVER: &str = "stride123";
    const IBC_RECEIVER: &str = "osmo456";

    #[test]
    fn test_autopilot_redeem_stake_msg() {
        let msg = get_autopilot_msg(
            Action::RedeemStake,
            RECEIVER,
            Some(IBC_RECEIVER.to_string()),
        );

        let expected = format!("{{\"autopilot\":{{\"receiver\":\"{}\",\"stakeibc\":{{\"action\":\"RedeemStake\",\"ibc_receiver\":\"{}\"}}}}}}", RECEIVER, IBC_RECEIVER);
        assert_eq!(serde_json::to_string(&msg).unwrap(), expected);
    }

    #[test]
    fn test_autopilot_liquid_stake_msg() {
        let msg = get_autopilot_msg(
            Action::LiquidStake,
            RECEIVER,
            Some(IBC_RECEIVER.to_string()),
        );

        let expected = format!("{{\"autopilot\":{{\"receiver\":\"{}\",\"stakeibc\":{{\"action\":\"LiquidStake\",\"ibc_receiver\":\"{}\"}}}}}}", RECEIVER, IBC_RECEIVER);
        assert_eq!(serde_json::to_string(&msg).unwrap(), expected);
    }

    #[test]
    fn test_autopilot_claim_msg() {
        let msg = get_autopilot_msg(Action::Claim, RECEIVER, None);

        let expected = format!(
            "{{\"autopilot\":{{\"receiver\":\"{}\",\"claim\":{{}}}}}}",
            RECEIVER
        );
        assert_eq!(serde_json::to_string(&msg).unwrap(), expected);
    }
}
