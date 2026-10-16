// Encodes/decodes the golden messages shared with the Rust crate tests
// (testdata/agent_control_messages.txt) to guarantee wire compatibility.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { create, equals, fromBinary, toBinary } from "@bufbuild/protobuf";
import {
  AgentControlMessageSchema,
  Band,
  TrxAntenna,
  TrxVfoFlag,
  TrxVfoMode,
  TrxVfoOperation,
} from "../dist/index.js";

const fixtureUrl = new URL("../../../testdata/agent_control_messages.txt", import.meta.url);

function loadFixtures() {
  const fixtures = new Map();
  for (const line of readFileSync(fixtureUrl, "utf8").split("\n")) {
    const trimmed = line.trim();
    if (trimmed === "" || trimmed.startsWith("#")) continue;
    const [name, hex = ""] = trimmed.split(/\s+/);
    fixtures.set(name, hex);
  }
  return fixtures;
}

const transceiver = (oneof) =>
  create(AgentControlMessageSchema, {
    message: { case: "transceiver", value: { transceiverMessage: oneof } },
  });

const messages = {
  hello: create(AgentControlMessageSchema, {
    message: {
      case: "agent",
      value: { agentMessage: { case: "hello", value: { protocolVersionMajor: 1, protocolVersionMinor: 2 } } },
    },
  }),
  frequency: transceiver({ case: "frequencyMessage", value: { vfoId: 1, frequency: 14_074_000n } }),
  mode: transceiver({ case: "modeMessage", value: { vfoId: 1, mode: TrxVfoMode.USB } }),
  band: transceiver({ case: "bandMessage", value: { vfoId: 2, band: Band.BAND_20M } }),
  vfo_operation: transceiver({
    case: "trxVfoMessage",
    value: { vfoId: 1, operation: TrxVfoOperation.BAND_DOWN },
  }),
  capabilities: transceiver({
    case: "trxCapabilitiesMessage",
    value: {
      rigModel: 3073,
      modelName: "IC-7300",
      manufacturerName: "Icom",
      rxFrequencyRanges: [
        {
          listId: 1,
          lowerFrequencyHz: 30_000n,
          upperFrequencyHz: 74_800_000n,
          modes: [TrxVfoMode.AM, TrxVfoMode.CW, TrxVfoMode.USB, TrxVfoMode.LSB],
          vfo: [TrxVfoFlag.A, TrxVfoFlag.B],
          antenna: [TrxAntenna.TRX_ANTENNA_1],
          label: "HF/6m",
        },
      ],
      txFrequencyRanges: [
        {
          listId: 2,
          lowerFrequencyHz: 1_800_000n,
          upperFrequencyHz: 10_000_000_000n,
          modes: [TrxVfoMode.FM],
          vfo: [TrxVfoFlag.CURR],
          antenna: [],
        },
      ],
    },
  }),
  empty: create(AgentControlMessageSchema),
};

const toHex = (bytes) => Buffer.from(bytes).toString("hex");

test("fixtures cover every message", () => {
  assert.deepEqual([...loadFixtures().keys()].sort(), Object.keys(messages).sort());
});

for (const [name, message] of Object.entries(messages)) {
  test(`${name}: encodes to the golden bytes`, () => {
    assert.equal(toHex(toBinary(AgentControlMessageSchema, message)), loadFixtures().get(name));
  });

  test(`${name}: decodes the golden bytes`, () => {
    const decoded = fromBinary(AgentControlMessageSchema, Buffer.from(loadFixtures().get(name), "hex"));
    assert.ok(equals(AgentControlMessageSchema, decoded, message));
  });
}
