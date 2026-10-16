//! Encodes/decodes the golden messages shared with the TypeScript package tests
//! (`testdata/agent_control_messages.txt`) to guarantee wire compatibility.

use std::collections::BTreeMap;

use prost::Message;
use qsp_proto_files::qsp::message::v1::{
    agent_control_message, agent_message, transceiver_message, AgentControlMessage, AgentMessage,
    Band, HelloCommand, RigFrequencyRangeMessage, TransceiverMessage, TrxAntenna, TrxBandMessage,
    TrxCapabilitiesMessage, TrxFrequencyMessage, TrxModeMessage, TrxVfoFlag, TrxVfoMode,
    TrxVfoOperation, TrxVfoOperationMessage,
};

const FIXTURES: &str = include_str!("../../../testdata/agent_control_messages.txt");

fn fixtures() -> BTreeMap<&'static str, Vec<u8>> {
    FIXTURES
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap();
            (name, decode_hex(parts.next().unwrap_or("")))
        })
        .collect()
}

fn decode_hex(hex: &str) -> Vec<u8> {
    assert!(hex.len() % 2 == 0, "odd hex length: {hex}");
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn transceiver(message: transceiver_message::TransceiverMessage) -> AgentControlMessage {
    AgentControlMessage {
        message: Some(agent_control_message::Message::Transceiver(
            TransceiverMessage {
                transceiver_message: Some(message),
            },
        )),
    }
}

fn messages() -> BTreeMap<&'static str, AgentControlMessage> {
    use transceiver_message::TransceiverMessage as Trx;

    BTreeMap::from([
        (
            "hello",
            AgentControlMessage {
                message: Some(agent_control_message::Message::Agent(AgentMessage {
                    agent_message: Some(agent_message::AgentMessage::Hello(HelloCommand {
                        protocol_version_major: 1,
                        protocol_version_minor: 2,
                    })),
                })),
            },
        ),
        (
            "frequency",
            transceiver(Trx::FrequencyMessage(TrxFrequencyMessage {
                vfo_id: 1,
                frequency: 14_074_000,
            })),
        ),
        (
            "mode",
            transceiver(Trx::ModeMessage(TrxModeMessage {
                vfo_id: 1,
                mode: TrxVfoMode::Usb.into(),
            })),
        ),
        (
            "band",
            transceiver(Trx::BandMessage(TrxBandMessage {
                vfo_id: 2,
                band: Band::Band20m.into(),
            })),
        ),
        (
            "vfo_operation",
            transceiver(Trx::TrxVfoMessage(TrxVfoOperationMessage {
                vfo_id: 1,
                operation: TrxVfoOperation::BandDown.into(),
            })),
        ),
        (
            "capabilities",
            transceiver(Trx::TrxCapabilitiesMessage(TrxCapabilitiesMessage {
                rig_model: 3073,
                model_name: "IC-7300".into(),
                manufacturer_name: "Icom".into(),
                rx_frequency_ranges: vec![RigFrequencyRangeMessage {
                    list_id: 1,
                    lower_frequency_hz: 30_000,
                    upper_frequency_hz: 74_800_000,
                    modes: vec![
                        TrxVfoMode::Am.into(),
                        TrxVfoMode::Cw.into(),
                        TrxVfoMode::Usb.into(),
                        TrxVfoMode::Lsb.into(),
                    ],
                    vfo: vec![TrxVfoFlag::A.into(), TrxVfoFlag::B.into()],
                    antenna: vec![TrxAntenna::TrxAntenna1.into()],
                    label: Some("HF/6m".into()),
                }],
                tx_frequency_ranges: vec![RigFrequencyRangeMessage {
                    list_id: 2,
                    lower_frequency_hz: 1_800_000,
                    upper_frequency_hz: 10_000_000_000,
                    modes: vec![TrxVfoMode::Fm.into()],
                    vfo: vec![TrxVfoFlag::Curr.into()],
                    antenna: vec![],
                    label: None,
                }],
            })),
        ),
        ("empty", AgentControlMessage::default()),
    ])
}

#[test]
fn fixtures_cover_every_message() {
    let fixture_names: Vec<_> = fixtures().into_keys().collect();
    let message_names: Vec<_> = messages().into_keys().collect();
    assert_eq!(fixture_names, message_names);
}

#[test]
fn messages_encode_to_golden_bytes() {
    let fixtures = fixtures();
    for (name, message) in messages() {
        assert_eq!(message.encode_to_vec(), fixtures[name], "{name}");
    }
}

#[test]
fn golden_bytes_decode_to_messages() {
    let fixtures = fixtures();
    for (name, message) in messages() {
        let decoded = AgentControlMessage::decode(fixtures[name].as_slice()).unwrap();
        assert_eq!(decoded, message, "{name}");
    }
}
