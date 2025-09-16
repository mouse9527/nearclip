# User Stories Documentation

## Overview

This directory contains the user stories for the Nearclip cross-platform clipboard synchronization tool. Each story addresses specific functionality required for the application.

## Story Index

| Story | Title | Priority | Status |
|-------|-------|----------|--------|
| M1 | [Generate Device Unique ID](M1_generate_device_unique_id.md) | P0 | 🟡 Not Started |
| M2 | [First Device Pairing](M2_first_device_pairing.md) | P0 | 🟡 Not Started |
| M3 | [Paired Device List](M3_paired_device_list.md) | P0 | 🟡 Not Started |
| M4 | [Capture Local Text Copy](M4_capture_local_text_copy.md) | P0 | 🟡 Not Started |
| M5 | [Text Message Structure](M5_text_message_structure.md) | P0 | 🟡 Not Started |
| M6 | [BLE Text Transfer](M6_ble_text_transfer.md) | P0 | 🟡 Not Started |
| M7 | [LAN Text Transfer](M7_lan_text_transfer.md) | P0 | 🟡 Not Started |
| M8 | [Inject Remote Text](M8_inject_remote_text.md) | P0 | 🟡 Not Started |

## Implementation Dependencies

### Phase 1: Foundation
- M1: Device unique ID generation
- M2: Device pairing mechanism
- M3: Device list management
- M4: Clipboard monitoring
- M5: Message structure definition

### Phase 2: Data Transmission
- M6: BLE text transfer
- M7: LAN text transfer
- M8: Clipboard injection

## Priority Levels

- **P0:** Must-have functionality for MVP
- **P1:** Important features for v1.0
- **P2:** Nice-to-have features

## Status Indicators

- 🟢 Completed
- 🟡 In Progress
- 🔴 Blocked
- ⚪ Not Started

## Related Documentation

- [Implementation Plan](../IMPLEMENTATION_PLAN.md)
- [Project Rules](../../PROJECT_RULES.md)
- [Architecture Structure](../../STRUCTURE.md)
- [Protocol Definitions](../../shared/proto/)