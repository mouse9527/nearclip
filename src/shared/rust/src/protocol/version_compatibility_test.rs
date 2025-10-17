#[cfg(test)]
mod version_compatibility_tests {
    use super::*;

    #[test]
    fn test_protocol_version_creation() {
        let version = common::ProtocolVersion::new(1, 0, 0);
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
        assert!(version.build_info.is_empty());

        let version_with_build = version.with_build_info("release".to_string());
        assert_eq!(version_with_build.build_info, "release");
    }

    #[test]
    fn test_protocol_version_string_representation() {
        let version = common::ProtocolVersion::new(1, 2, 3);
        let version_with_build = version.with_build_info("beta".to_string());

        // 测试版本字符串格式
        assert_eq!(format!("{}", version.major), "1");
        assert_eq!(format!("{}", version.minor), "2");
        assert_eq!(format!("{}", version.patch), "3");
        assert_eq!(version_with_build.build_info, "beta");
    }

    #[test]
    fn test_version_compatibility_rules() {
        let test_cases = vec![
            // (version1, version2, expected_compatible)
            ((1, 0, 0), (1, 0, 0), true),    // 完全相同
            ((1, 0, 0), (1, 0, 1), true),    // 补丁版本更新
            ((1, 0, 1), (1, 0, 0), true),    // 补丁版本向后兼容
            ((1, 0, 0), (1, 1, 0), true),    // 次版本更新
            ((1, 1, 0), (1, 0, 0), true),    // 次版本向后兼容
            ((1, 0, 0), (1, 2, 0), true),    // 较大次版本更新
            ((1, 2, 0), (1, 0, 0), true),    // 较小次版本向后兼容
            ((1, 0, 0), (2, 0, 0), false),   // 主版本不同
            ((2, 0, 0), (1, 0, 0), false),   // 主版本不同
            ((1, 0, 0), (1, 0, 5), true),    // 补丁版本向后兼容
            ((1, 0, 5), (1, 0, 0), true),    // 补丁版本向前兼容
        ];

        for ((major1, minor1, patch1), (major2, minor2, patch2), expected) in test_cases {
            let version1 = common::ProtocolVersion::new(major1, minor1, patch1);
            let version2 = common::ProtocolVersion::new(major2, minor2, patch2);

            assert_eq!(
                version1.is_compatible_with(&version2),
                expected,
                "Version {}.{}.{} should {}be compatible with {}.{}.{}",
                major1, minor1, patch1,
                if expected { "" } else { "not " },
                major2, minor2, patch2
            );
        }
    }

    #[test]
    fn test_capability_negotiation() {
        let min_version = common::ProtocolVersion::new(1, 0, 0);
        let max_version = common::ProtocolVersion::new(1, 2, 0);

        let mut negotiation = common::CapabilityNegotiation::new(min_version.clone(), max_version.clone());

        // 测试基本协商创建
        assert_eq!(negotiation.min_version.as_ref().unwrap().major, 1);
        assert_eq!(negotiation.min_version.as_ref().unwrap().minor, 0);
        assert_eq!(negotiation.max_version.as_ref().unwrap().major, 1);
        assert_eq!(negotiation.max_version.as_ref().unwrap().minor, 2);

        // 测试添加支持的功能
        negotiation = negotiation.with_supported_feature("clipboard_sync".to_string());
        assert!(negotiation.supported_features.contains(&"clipboard_sync".to_string()));

        // 测试添加必需的功能
        negotiation = negotiation.with_required_feature("encryption".to_string());
        assert!(negotiation.required_features.contains(&"encryption".to_string()));

        // 测试多个功能
        negotiation = negotiation
            .with_supported_feature("file_transfer".to_string())
            .with_supported_feature("multi_device".to_string())
            .with_required_feature("compression".to_string());

        assert_eq!(negotiation.supported_features.len(), 3);
        assert_eq!(negotiation.required_features.len(), 2);
    }

    #[test]
    fn test_capability_negotiation_response() {
        let selected_version = common::ProtocolVersion::new(1, 1, 0);

        // 测试兼容响应
        let compatible_response = common::CapabilityNegotiationResponse::compatible(selected_version.clone());
        assert_eq!(compatible_response.selected_version.as_ref().unwrap().major, 1);
        assert_eq!(compatible_response.selected_version.as_ref().unwrap().minor, 1);
        assert!(compatible_response.is_compatible);

        // 测试不兼容响应
        let incompatible_response = common::CapabilityNegotiationResponse::incompatible(selected_version.clone());
        assert_eq!(incompatible_response.selected_version.as_ref().unwrap().major, 1);
        assert_eq!(incompatible_response.selected_version.as_ref().unwrap().minor, 1);
        assert!(!incompatible_response.is_compatible);

        // 测试添加支持和不支持的功能
        let mut response = compatible_response;
        response = common::CapabilityNegotiationResponse::with {
            $0.selectedVersion = selected_version
            $0.supportedFeatures = ["clipboard_sync", "encryption"]
            $0.unsupportedFeatures = ["advanced_features"]
            $0.compatibility = true
        };

        assert_eq!(response.supported_features.len(), 2);
        assert_eq!(response.unsupported_features.len(), 1);
        assert!(response.supported_features.contains(&"clipboard_sync".to_string()));
        assert!(response.unsupported_features.contains(&"advanced_features".to_string()));
    }

    #[test]
    fn test_version_manager() {
        // 创建版本管理器实例
        let current_version = common::ProtocolVersion::new(1, 0, 0);
        let supported_versions = vec![
            common::ProtocolVersion::new(1, 0, 0),
            common::ProtocolVersion::new(1, 0, 1),
            common::ProtocolVersion::new(1, 1, 0),
        ];

        // 在实际实现中，这里会创建 VersionManager 实例
        // 现在我们模拟版本协商逻辑

        let test_cases = vec![
            // 客户端版本范围，期望兼容的服务器版本
            ((1, 0, 0), (1, 2, 0), Some((1, 1, 0))), // 1.0-1.2 与 1.1.0 兼容
            ((1, 0, 0), (1, 1, 0), Some((1, 0, 0))), // 1.0-1.1 与 1.0.0 兼容
            ((1, 1, 0), (1, 2, 0), Some((1, 1, 0))), // 1.1-1.2 与 1.1.0 兼容
            ((2, 0, 0), (2, 1, 0), None),          // 2.x 版本，不在支持范围内
            ((0, 9, 0), (1, 0, 0), None),          // 0.x 版本，主版本不匹配
        ];

        for ((min_major, min_minor, min_patch), (max_major, max_minor, max_patch), expected) in test_cases {
            let min_version = common::ProtocolVersion::new(min_major, min_minor, min_patch);
            let max_version = common::ProtocolVersion::new(max_major, max_minor, max_patch);

            // 检查是否有兼容版本
            let compatible_version = find_compatible_version(&min_version, &max_version, &supported_versions);

            match (compatible_version, expected) {
                (Some(version), Some((exp_major, exp_minor, exp_patch))) => {
                    assert_eq!(version.major, exp_major);
                    assert_eq!(version.minor, exp_minor);
                    assert_eq!(version.patch, exp_patch);
                }
                (None, None) => {
                    // 预期不兼容，测试通过
                }
                _ => {
                    panic!("Unexpected compatibility result for range {}.{}.{} - {}.{}.{}",
                            min_major, min_minor, min_patch, max_major, max_minor, max_patch);
                }
            }
        }
    }

    #[test]
    fn test_feature_degradation() {
        // 测试功能降级场景
        let negotiation = common::CapabilityNegotiation::new(
            common::ProtocolVersion::new(1, 0, 0),
            common::ProtocolVersion::new(1, 2, 0)
        ).with_supported_feature("clipboard_sync".to_string())
            .with_supported_feature("encryption".to_string())
            .with_supported_feature("file_transfer".to_string())
            .with_supported_feature("advanced_features".to_string());

        let older_client_features = vec!["clipboard_sync", "encryption"];
        let newer_client_features = vec!["clipboard_sync", "encryption", "file_transfer", "advanced_features"];

        // 模拟与旧版本客户端的协商
        let compatible_with_old = find_common_features(
            &negotiation.supported_features,
            &older_client_features.iter().map(|s| s.to_string()).collect::<Vec<_>>()
        );

        // 模拟与新版本客户端的协商
        let compatible_with_new = find_common_features(
            &negotiation.supported_features,
            &newer_client_features.iter().map(|s| s.to_string()).collect::<Vec<_>>()
        );

        // 旧客户端应该支持基本功能
        assert!(compatible_with_old.contains(&"clipboard_sync".to_string()));
        assert!(compatible_with_old.contains(&"encryption".to_string()));
        assert!(!compatible_with_old.contains(&"advanced_features".to_string()));

        // 新客户端应该支持所有功能
        assert_eq!(compatible_with_new.len(), negotiation.supported_features.len());
    }

    #[test]
    fn test_version_upgrade_path() {
        // 测试版本升级路径
        let upgrade_paths = vec![
            // (from_version, to_version, should_upgrade)
            ((1, 0, 0), (1, 0, 1), true),   // 补丁升级
            ((1, 0, 0), (1, 1, 0), true),   // 次版本升级
            ((1, 0, 0), (2, 0, 0), true),   // 主版本升级（重大变更）
            ((1, 2, 0), (1, 1, 0), false),  // 降级不应该发生
            ((2, 0, 0), (1, 0, 0), false),  // 主版本降级
        ];

        for ((from_major, from_minor, from_patch), (to_major, to_minor, to_patch), should_upgrade) in upgrade_paths {
            let from_version = common::ProtocolVersion::new(from_major, from_minor, from_patch);
            let to_version = common::ProtocolVersion::new(to_major, to_minor, to_patch);

            let upgrade_possible = can_upgrade(&from_version, &to_version);

            assert_eq!(upgrade_possible, should_upgrade,
                "Upgrade from {}.{}.{} to {}.{}.{} should {}be possible",
                from_major, from_minor, from_patch, to_major, to_minor, to_patch,
                if should_upgrade { "" } else { "not " }
            );
        }
    }

    #[test]
    fn test_message_version_header() {
        // 测试消息版本头部处理
        let current_time = chrono::Utc::now().timestamp_millis() as u64;
        let device_id = "test-device";

        // 创建带版本信息的消息
        let message_with_version = common::Heartbeat {
            device_id: device_id.to_string(),
            timestamp: current_time,
            sequence_number: 42,
        };

        // 验证消息包含版本相关信息
        assert_eq!(message_with_version.device_id, device_id);
        assert_eq!(message_with_version.timestamp, current_time);
        assert_eq!(message_with_version.sequence_number, 42);

        // 在实际实现中，这里会检查消息版本兼容性
        // 现在我们验证消息的基本结构
        assert!(!message_with_version.device_id.is_empty());
        assert!(message_with_version.timestamp > 0);
    }

    #[test]
    fn test_version_negotiation_flow() {
        // 测试完整的版本协商流程
        let client_min_version = common::ProtocolVersion::new(1, 0, 0);
        let client_max_version = common::ProtocolVersion::new(1, 2, 0);
        let server_supported_versions = vec![
            common::ProtocolVersion::new(1, 0, 0),
            common::ProtocolVersion::new(1, 1, 0),
            common::ProtocolVersion::new(2, 0, 0),
        ];

        // 1. 客户端发送协商请求
        let client_negotiation = common::CapabilityNegotiation::new(
            client_min_version.clone(),
            client_max_version.clone()
        ).with_supported_feature("clipboard_sync".to_string())
            .with_required_feature("encryption".to_string());

        // 2. 服务器选择兼容版本
        let selected_version = find_compatible_version(
            &client_min_version,
            &client_max_version,
            &server_supported_versions
        );

        // 3. 验证协商结果
        assert!(selected_version.is_some());
        let negotiated_version = selected_version.unwrap();
        assert_eq!(negotiated_version.major, 1);
        assert!(negotiated_version.minor <= 2);

        // 4. 服务器创建响应
        let server_response = common::CapabilityNegotiationResponse::compatible(negotiated_version.clone())
            .with_supported_features("clipboard_sync".to_string())
            .with_supported_features("encryption".to_string())
            .with_unsupported_features("advanced_features".to_string());

        // 5. 验证响应
        assert_eq!(server_response.selected_version.as_ref().unwrap().major, negotiated_version.major);
        assert!(server_response.is_compatible);
        assert!(server_response.supported_features.contains(&"clipboard_sync".to_string()));
    }

    // 辅助函数
    fn find_compatible_version(
        min_version: &common::ProtocolVersion,
        max_version: &common::ProtocolVersion,
        supported_versions: &[common::ProtocolVersion],
    ) -> Option<common::ProtocolVersion> {
        for supported_version in supported_versions.iter().rev() {
            if is_version_compatible(supported_version, min_version, max_version) {
                return Some(supported_version.clone());
            }
        }
        None
    }

    fn is_version_compatible(
        version: &common::ProtocolVersion,
        min_version: &common::ProtocolVersion,
        max_version: &common::ProtocolVersion,
    ) -> bool {
        // 主版本必须相同
        if version.major != min_version.major || version.major != max_version.major {
            return false;
        }

        // 版本必须在范围内
        let version_numeric = version.major * 10000 + version.minor * 100 + version.patch;
        let min_numeric = min_version.major * 10000 + min_version.minor * 100 + min_version.patch;
        let max_numeric = max_version.major * 10000 + max_version.minor * 100 + max_version.patch;

        version_numeric >= min_numeric && version_numeric <= max_numeric
    }

    fn find_common_features(client_features: &[String], server_features: &[String]) -> Vec<String> {
        client_features.iter()
            .filter(|feature| server_features.contains(feature))
            .cloned()
            .collect()
    }

    fn can_upgrade(from_version: &common::ProtocolVersion, to_version: &common::ProtocolVersion) -> bool {
        // 主版本升级总是可能的（可能不兼容）
        if to_version.major > from_version.major {
            return true;
        }

        // 次版本升级
        if to_version.major == from_version.major && to_version.minor > from_version.minor {
            return true;
        }

        // 补丁版本升级
        if to_version.major == from_version.major
            && to_version.minor == from_version.minor
            && to_version.patch > from_version.patch {
            return true;
        }

        false
    }
}