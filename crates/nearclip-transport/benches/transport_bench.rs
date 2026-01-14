//! Transport layer performance benchmarks
//!
//! Measures performance of critical transport operations:
//! - Channel selection latency
//! - Encryption/decryption throughput
//! - Multi-device broadcast performance

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use nearclip_crypto::Aes256Gcm;
use nearclip_sync::{Channel, Message};
use nearclip_transport::{MockTransport, MockConfig, TransportManager, TransportManagerConfig};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Benchmark 4.1: Channel Selection Latency
///
/// Measures how quickly TransportManager can select the best transport
/// for a device from multiple available channels.
///
/// Acceptance criteria: < 1ms for 10 devices with 2 channels each
fn bench_channel_selection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("channel_selection_10_devices", |b| {
        b.iter(|| {
            rt.block_on(async {
                let manager = TransportManager::new();

                // Add 10 devices, each with WiFi + BLE
                for i in 0..10 {
                    let device_id = format!("device_{}", i);

                    let wifi_config = MockConfig::default().with_channel(Channel::Wifi);
                    let ble_config = MockConfig::default().with_channel(Channel::Ble);

                    let wifi_transport = Arc::new(MockTransport::new(
                        black_box(&device_id),
                        black_box(wifi_config),
                    ));
                    let ble_transport = Arc::new(MockTransport::new(
                        black_box(&device_id),
                        black_box(ble_config),
                    ));

                    manager.add_transport(&device_id, wifi_transport).await;
                    manager.add_transport(&device_id, ble_transport).await;
                }

                // Measure selection time
                let result = manager.get_best_transport(black_box("device_5")).await;
                black_box(result)
            })
        });
    });
}

/// Benchmark 4.2: Encryption/Decryption Throughput
///
/// Measures encryption and decryption performance for various message sizes.
///
/// Acceptance criteria:
/// - 1KB: < 100 μs
/// - 10KB: < 500 μs
/// - 100KB: < 3 ms
fn bench_encryption_throughput(c: &mut Criterion) {
    let shared_secret = [0u8; 32];
    let cipher = Aes256Gcm::new(&shared_secret).unwrap();

    let mut group = c.benchmark_group("encryption");

    for size in [1024, 10_240, 102_400].iter() {
        let data = vec![0u8; *size];
        let size_kb = size / 1024;

        // Benchmark encryption
        group.bench_with_input(
            BenchmarkId::new("encrypt", format!("{}KB", size_kb)),
            size,
            |b, _| {
                b.iter(|| {
                    cipher.encrypt(black_box(&data)).unwrap()
                });
            },
        );

        // Benchmark decryption
        let encrypted = cipher.encrypt(&data).unwrap();
        group.bench_with_input(
            BenchmarkId::new("decrypt", format!("{}KB", size_kb)),
            size,
            |b, _| {
                b.iter(|| {
                    cipher.decrypt(black_box(&encrypted)).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark 4.3: Multi-Device Broadcast Performance
///
/// Measures how quickly messages can be broadcast to multiple devices.
///
/// Acceptance criteria: 100 devices broadcast < 100ms
fn bench_broadcast_performance(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("broadcast");

    for device_count in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_devices", device_count)),
            device_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let manager = TransportManager::new();

                        // Add devices
                        for i in 0..count {
                            let device_id = format!("device_{}", i);
                            let transport = Arc::new(MockTransport::new(
                                &device_id,
                                MockConfig::default().with_channel(Channel::Wifi),
                            ));
                            manager.add_transport(&device_id, transport).await;
                        }

                        // Broadcast message
                        let msg = Message::heartbeat("test_device".to_string());
                        let results = manager.broadcast(black_box(&msg)).await;
                        black_box(results)
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark 4.4: Message Serialization/Deserialization
///
/// Measures message encoding and decoding performance
fn bench_message_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_serialization");

    for size in [100, 1000, 10_000].iter() {
        let content = vec![0u8; *size];
        let msg = Message::clipboard_sync(&content, "test_device".to_string());

        // Benchmark serialization
        group.bench_with_input(
            BenchmarkId::new("serialize", format!("{}_bytes", size)),
            size,
            |b, _| {
                b.iter(|| {
                    bincode::serialize(black_box(&msg)).unwrap()
                });
            },
        );

        // Benchmark deserialization
        let serialized = bincode::serialize(&msg).unwrap();
        group.bench_with_input(
            BenchmarkId::new("deserialize", format!("{}_bytes", size)),
            size,
            |b, _| {
                b.iter(|| {
                    bincode::deserialize::<Message>(black_box(&serialized)).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark 4.5: Send Message with Failover
///
/// Measures the overhead of failover logic when enabled vs disabled
fn bench_send_with_failover(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("failover");

    // Benchmark with failover enabled
    group.bench_function("enabled", |b| {
        b.iter(|| {
            rt.block_on(async {
                let manager = TransportManager::with_config(TransportManagerConfig {
                    failover_on_error: true,
                    ..Default::default()
                });

                let device_id = "test_device";
                let wifi = Arc::new(MockTransport::new(device_id, MockConfig::default().with_channel(Channel::Wifi)));
                let ble = Arc::new(MockTransport::new(device_id, MockConfig::default().with_channel(Channel::Ble)));

                manager.add_transport(device_id, wifi).await;
                manager.add_transport(device_id, ble).await;

                let msg = Message::heartbeat("sender".to_string());
                manager.send_to_device(black_box(device_id), black_box(&msg)).await
            })
        });
    });

    // Benchmark with failover disabled
    group.bench_function("disabled", |b| {
        b.iter(|| {
            rt.block_on(async {
                let manager = TransportManager::with_config(TransportManagerConfig {
                    failover_on_error: false,
                    ..Default::default()
                });

                let device_id = "test_device";
                let wifi = Arc::new(MockTransport::new(device_id, MockConfig::default().with_channel(Channel::Wifi)));
                let ble = Arc::new(MockTransport::new(device_id, MockConfig::default().with_channel(Channel::Ble)));

                manager.add_transport(device_id, wifi).await;
                manager.add_transport(device_id, ble).await;

                let msg = Message::heartbeat("sender".to_string());
                manager.send_to_device(black_box(device_id), black_box(&msg)).await
            })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_channel_selection,
    bench_encryption_throughput,
    bench_broadcast_performance,
    bench_message_serialization,
    bench_send_with_failover
);

criterion_main!(benches);
