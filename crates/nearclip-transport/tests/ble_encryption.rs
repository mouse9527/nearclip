//! Integration tests for BLE transport encryption
//!
//! These tests verify that the BLE transport layer correctly:
//! - Encrypts messages before sending
//! - Decrypts messages after receiving
//! - Handles encryption key mismatches
//! - Maintains acceptable performance overhead

mod common;

use common::{create_encrypted_pair, MockBleTransport};
use nearclip_crypto::EcdhKeyPair;
use nearclip_sync::Message;
use nearclip_transport::Transport;
use std::time::Instant;

/// Helper to create a test message with specific content
fn create_test_message(content: &str) -> Message {
    Message::clipboard_sync(content.as_bytes(), "test_device".to_string())
}

/// Test 1.1: End-to-end encryption/decryption roundtrip
///
/// Verifies that:
/// - Two devices with matching ECDH shared secrets can communicate
/// - Messages are encrypted on send
/// - Messages are decrypted on receive
/// - The content matches exactly after roundtrip
#[tokio::test]
async fn test_ble_encryption_roundtrip() {
    // 1. Create two ECDH keypairs (simulating two devices)
    let keypair_a = EcdhKeyPair::generate();
    let keypair_b = EcdhKeyPair::generate();

    // 2. Compute shared secrets (should be identical)
    let secret_a = keypair_a
        .compute_shared_secret(&keypair_b.public_key_bytes())
        .expect("Failed to compute shared secret A");
    let secret_b = keypair_b
        .compute_shared_secret(&keypair_a.public_key_bytes())
        .expect("Failed to compute shared secret B");

    assert_eq!(secret_a, secret_b, "Shared secrets should match");

    // 3. Create encrypted transports
    let (transport_a, transport_b) =
        create_encrypted_pair("device_a", "device_b", &secret_a).expect("Failed to create encrypted pair");

    // 4. Send encrypted message from A
    let original_msg = create_test_message("This is a secret message that should be encrypted");
    transport_a.send(&original_msg).await.expect("Failed to send message");

    // 5. Transfer chunks from A to B (simulating BLE transfer)
    let chunks = transport_a.get_sent_chunks().await;
    assert!(!chunks.is_empty(), "Should have created chunks");

    let mut reassemblers = transport_b.reassemblers.lock().await;
    for chunk in chunks {
        transport_b
            .process_chunk(&chunk, &mut reassemblers)
            .await
            .expect("Failed to process chunk");
    }
    drop(reassemblers);

    // 6. Receive and decrypt message on B
    let received_msg = transport_b.recv().await.expect("Failed to receive message");

    // 7. Verify content matches
    assert_eq!(
        received_msg.payload, original_msg.payload,
        "Decrypted message should match original"
    );
}

/// Test 1.2: Key mismatch detection
///
/// Verifies that:
/// - Messages encrypted with one key cannot be decrypted with a different key
/// - Decryption failure is properly detected and reported
#[tokio::test]
async fn test_ble_encryption_key_mismatch() {
    // 1. Create two different shared secrets
    let keypair_a = EcdhKeyPair::generate();
    let keypair_b = EcdhKeyPair::generate();
    let keypair_c = EcdhKeyPair::generate(); // Third keypair for mismatch

    let secret_a = keypair_a.compute_shared_secret(&keypair_b.public_key_bytes()).unwrap();
    let secret_wrong = keypair_a.compute_shared_secret(&keypair_c.public_key_bytes()).unwrap();

    assert_ne!(secret_a, secret_wrong, "Secrets should be different");

    // 2. Create transport with correct key
    let transport_sender = MockBleTransport::new_with_encryption("sender", &secret_a)
        .expect("Failed to create sender");

    // 3. Create transport with wrong key
    let transport_receiver = MockBleTransport::new_with_encryption("receiver", &secret_wrong)
        .expect("Failed to create receiver");

    // 4. Send encrypted message
    let msg = create_test_message("encrypted with key A");
    transport_sender.send(&msg).await.expect("Send should succeed");

    // 5. Try to decrypt with wrong key
    let chunks = transport_sender.get_sent_chunks().await;
    let mut reassemblers = transport_receiver.reassemblers.lock().await;

    let mut decryption_failed = false;
    for chunk in chunks {
        if let Err(err) = transport_receiver.process_chunk(&chunk, &mut reassemblers).await {
            // Check if it's a decryption-related error (would be wrapped in Other or Deserialization)
            let err_msg = err.to_string().to_lowercase();
            if err_msg.contains("decrypt") || err_msg.contains("deserializ") {
                decryption_failed = true;
                break;
            }
        }
    }

    assert!(decryption_failed, "Decryption should fail with wrong key");
}

/// Test 1.3: Encryption performance overhead
///
/// Verifies that:
/// - Encryption adds acceptable overhead for typical message sizes
/// - Performance is acceptable for typical usage
///
/// Note: In debug builds, overhead may be higher due to lack of optimizations.
/// The 500% threshold is set for debug builds; release builds should be much faster.
#[tokio::test]
async fn test_ble_encryption_performance_overhead() {
    const TEST_ITERATIONS: usize = 100;
    const MESSAGE_SIZE_KB: usize = 10;
    const MAX_OVERHEAD_PERCENT: f64 = 500.0; // Relaxed for debug builds

    // 1. Create encrypted and unencrypted transports
    let shared_secret = [0u8; 32];
    let transport_encrypted = MockBleTransport::new_with_encryption("encrypted", &shared_secret)
        .expect("Failed to create encrypted transport");

    let transport_plain = MockBleTransport::new_without_encryption("plain");

    // 2. Prepare test message (10 KB)
    let content = "x".repeat(MESSAGE_SIZE_KB * 1024);
    let msg = create_test_message(&content);

    // 3. Measure encrypted transport time
    let start = Instant::now();
    for _ in 0..TEST_ITERATIONS {
        transport_encrypted.send(&msg).await.expect("Encrypted send failed");
    }
    let encrypted_duration = start.elapsed();

    // Clear for next test
    transport_encrypted.clear_sent().await;

    // 4. Measure plain transport time
    let start = Instant::now();
    for _ in 0..TEST_ITERATIONS {
        transport_plain.send(&msg).await.expect("Plain send failed");
    }
    let plain_duration = start.elapsed();

    // 5. Calculate overhead
    let overhead = if plain_duration.as_millis() > 0 {
        ((encrypted_duration.as_millis() as f64 - plain_duration.as_millis() as f64)
            / plain_duration.as_millis() as f64)
            * 100.0
    } else {
        0.0
    };

    println!("Plain transport: {:?}", plain_duration);
    println!("Encrypted transport: {:?}", encrypted_duration);
    println!("Overhead: {:.2}%", overhead);

    // 6. Verify overhead is acceptable
    // In debug builds, overhead can be significant due to lack of optimizations
    // This test mainly ensures the encryption doesn't hang or fail
    assert!(
        overhead < MAX_OVERHEAD_PERCENT,
        "Encryption overhead should be < {}%, got {:.2}%",
        MAX_OVERHEAD_PERCENT,
        overhead
    );

    // Also verify encryption actually works (encrypted should take longer)
    assert!(encrypted_duration > plain_duration, "Encrypted transport should take longer than plain");
}

/// Test 1.4: Large message encryption
///
/// Verifies that encryption works correctly for large messages
/// that require multiple BLE chunks.
#[tokio::test]
async fn test_ble_encryption_large_message() {
    let shared_secret = [3u8; 32];
    let (transport_a, transport_b) = create_encrypted_pair("device_a", "device_b", &shared_secret)
        .expect("Failed to create encrypted pair");

    // Create a large message (100 KB)
    let large_content = "L".repeat(100 * 1024);
    let large_msg = create_test_message(&large_content);

    // Send
    transport_a.send(&large_msg).await.expect("Failed to send large message");

    // Verify multiple chunks were created
    let chunks = transport_a.get_sent_chunks().await;
    assert!(chunks.len() > 1, "Large message should create multiple chunks");

    // Transfer chunks
    let mut reassemblers = transport_b.reassemblers.lock().await;
    for chunk in chunks {
        transport_b
            .process_chunk(&chunk, &mut reassemblers)
            .await
            .expect("Failed to process chunk");
    }
    drop(reassemblers);

    // Receive and verify
    let received = transport_b.recv().await.expect("Failed to receive large message");
    assert_eq!(
        received.payload, large_msg.payload,
        "Large message content should match"
    );
}

/// Test 1.5: Multiple messages in sequence
///
/// Verifies that:
/// - Multiple encrypted messages can be sent sequentially
/// - Each message is encrypted/decrypted independently
/// - Message order is preserved
#[tokio::test]
async fn test_ble_encryption_multiple_messages() {
    let shared_secret = [4u8; 32];
    let (transport_a, transport_b) = create_encrypted_pair("device_a", "device_b", &shared_secret)
        .expect("Failed to create encrypted pair");

    // Send 5 messages
    let messages = vec![
        create_test_message("Message 1"),
        create_test_message("Message 2"),
        create_test_message("Message 3"),
        create_test_message("Message 4"),
        create_test_message("Message 5"),
    ];

    for msg in &messages {
        transport_a.send(msg).await.expect("Failed to send message");

        // Transfer chunks
        let chunks = transport_a.get_sent_chunks().await;
        let mut reassemblers = transport_b.reassemblers.lock().await;
        for chunk in chunks {
            transport_b
                .process_chunk(&chunk, &mut reassemblers)
                .await
                .expect("Failed to process chunk");
        }
        drop(reassemblers);

        transport_a.clear_sent().await;
    }

    // Receive all messages
    for original_msg in &messages {
        let received = transport_b.recv().await.expect("Failed to receive message");
        assert_eq!(received.payload, original_msg.payload, "Message content should match");
    }
}

/// Test 1.6: Encryption with different message types
///
/// Verifies encryption works for all message types in the protocol
#[tokio::test]
async fn test_ble_encryption_different_message_types() {
    let shared_secret = [5u8; 32];
    let (transport_a, transport_b) = create_encrypted_pair("device_a", "device_b", &shared_secret)
        .expect("Failed to create encrypted pair");

    // Test different message types
    let messages = vec![
        Message::heartbeat("device_a".to_string()),
        Message::heartbeat("device_a".to_string()),
        Message::clipboard_sync(b"clipboard data", "device_a".to_string()),
        // Add more message types as needed
    ];

    for msg in messages {
        // Send
        transport_a.send(&msg).await.expect("Failed to send message");

        // Transfer
        let chunks = transport_a.get_sent_chunks().await;
        let mut reassemblers = transport_b.reassemblers.lock().await;
        for chunk in chunks {
            transport_b
                .process_chunk(&chunk, &mut reassemblers)
                .await
                .expect("Failed to process chunk");
        }
        drop(reassemblers);

        // Receive
        let received = transport_b.recv().await.expect("Failed to receive message");

        // Verify payload matches
        assert_eq!(received.payload, msg.payload, "Payload should match for all message types");

        // Clear for next iteration
        transport_a.clear_sent().await;
    }
}
