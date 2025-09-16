# Story M7: Text Transfer via LAN

## Description
As a user, when devices are on the same local network, I want to synchronize captured text to paired devices.

## Priority
P0

## Acceptance Criteria
- Paired device A copying "World" results in device B receiving it within 1 second
- LAN discovery works without manual IP configuration
- Handle network changes and device roaming
- Maintain security on local network
- Support multiple devices on the same network

## Implementation Notes
- Implement mDNS/Bonjour for device discovery
- Use TCP/WebSocket for reliable data transfer
- Implement network change detection
- Handle firewall and router configurations
- Provide fallback mechanisms for network issues

## Dependencies
- M1: Device unique ID generation
- M2: First device pairing
- M3: Paired device list
- M4: Capture local text copy events
- M5: Text message structure definition
- Platform-specific networking APIs

## Testing Requirements
- Test LAN discovery and connection
- Verify text transfer timing (1-second target)
- Test network change handling
- Validate security measures
- Multi-device scenario testing

## Success Metrics
- 95%+ transfer success rate
- < 1 second transfer latency (80% of cases)
- 100% automatic device discovery
- 100% secure network communication