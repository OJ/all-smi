// Copyright 2025 Lablup Inc. and Jeongkyu Shin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Canonical key names for [`GpuInfo::detail`](crate::device::types::GpuInfo).
//!
//! `detail` is a `HashMap<String, String>` shared by every vendor reader, and
//! its keys are a contract rather than display text: the UI looks each one up
//! by name, the Prometheus exporter turns the whole map into labels, and the
//! remote-metrics parser rebuilds it from a scrape. Readers historically spelled
//! the same datum two ways -- Intel wrote `Architecture` while the dashboard
//! read `architecture` -- so the datum silently went missing for that vendor.
//!
//! Every key therefore lives here as a constant, and every reader and consumer
//! refers to the constant instead of a literal. Renaming a key becomes a
//! compiler-checked change, and a typo stops being a silent `None`.
//!
//! # Spelling rule
//!
//! Keys are `snake_case`, matching what
//! [`sanitize_label_name`](crate::parsing::common::sanitize_label_name) produces.
//! That function lowercases and rewrites every non-alphanumeric byte to `_`, so
//! a `snake_case` key survives the round trip through the exporter and back
//! through the parser unchanged, while a `Title Case` key does not: the label
//! for `"Driver Version"` was always `driver_version`, and a remote node's
//! `detail` came back keyed that way while the local one was still `Title Case`.
//! `keys_round_trip_through_the_exporter` below pins the rule.
//!
//! Human-readable text belongs in the renderer next to the value it labels, not
//! in the key.

// Readers are platform-gated -- the Windows, macOS, and vendor-SDK backends do
// not compile on every target -- so a constant that is live on one target looks
// dead on another. Keeping the table complete on every target is what lets a
// key be renamed in one place.
#![allow(dead_code)]

// ---- Shared ----
pub const DEVICE_INDEX: &str = "device_index";
pub const TOTAL_MEMORY: &str = "total_memory";

pub const ARCHITECTURE: &str = "architecture";
pub const CLOCK_GRAPHICS_MAX: &str = "clock_graphics_max";
pub const CLOCK_MEMORY_MAX: &str = "clock_memory_max";
pub const COMBINED_POWER_MW: &str = "combined_power_mw";
pub const CORE_COUNT: &str = "core_count";
pub const CUDA_VERSION: &str = "cuda_version";
pub const CURRENT_POWER: &str = "current_power";
pub const DAC_TYPE: &str = "dac_type";
pub const DEVICE_ID: &str = "device_id";
pub const DEVICE_NAME: &str = "device_name";
pub const DRIVER_VERSION: &str = "driver_version";
pub const ENGINE_3D: &str = "engine_3d";
pub const FAN_SPEED: &str = "fan_speed";
pub const FIRMWARE: &str = "firmware";
pub const FIRMWARE_VERSION: &str = "firmware_version";
pub const GPU_TEMP_LIMIT: &str = "gpu_temp_limit";
pub const HLO_QUEUE_SIZE: &str = "hlo_queue_size";
pub const INDEX: &str = "index";
pub const INSTANCE_NAME: &str = "instance_name";
pub const LIB_NAME: &str = "lib_name";
pub const LIB_VERSION: &str = "lib_version";
pub const MAX_POWER: &str = "max_power";
pub const MEMORY_TYPE: &str = "memory_type";
pub const METRICS_AVAILABLE: &str = "metrics_available";
pub const METRICS_SOURCE: &str = "metrics_source";
pub const NAME: &str = "name";
pub const NATIVE_METRICS: &str = "native_metrics";
pub const NOTE: &str = "note";
pub const PCI_BUS_ID: &str = "pci_bus_id";
pub const PCIE_GENERATION: &str = "pcie_generation";
pub const PCIE_LINK_WIDTH: &str = "pcie_link_width";
pub const PERFORMANCE_STATE: &str = "performance_state";
pub const POWER: &str = "power";
pub const POWER_DRAW: &str = "power_draw";
pub const POWER_LIMIT_CURRENT: &str = "power_limit_current";
pub const POWER_LIMIT_DEFAULT: &str = "power_limit_default";
pub const POWER_LIMIT_MAX: &str = "power_limit_max";
pub const ROCM_VERSION: &str = "rocm_version";
pub const SOURCE_FAN: &str = "source_fan";
pub const SOURCE_FREQUENCY: &str = "source_frequency";
pub const SOURCE_MEMORY: &str = "source_memory";
pub const SOURCE_MEMORY_USED: &str = "source_memory_used";
pub const SOURCE_POWER: &str = "source_power";
pub const SOURCE_TEMPERATURE: &str = "source_temperature";
pub const SOURCE_UTILIZATION: &str = "source_utilization";
pub const STATUS: &str = "status";
pub const TEMPERATURE: &str = "temperature";
pub const THERMAL_PRESSURE: &str = "thermal_pressure";
pub const TRANSPORT: &str = "transport";
pub const USED_MEMORY: &str = "used_memory";
pub const UTILIZATION: &str = "utilization";
pub const UUID: &str = "uuid";
pub const VIDEO_PROCESSOR: &str = "video_processor";
pub const VRAM_BUDGET_PROCESS: &str = "vram_budget_process";
pub const VRAM_USAGE_PROCESS: &str = "vram_usage_process";

// ---- NVIDIA ----
pub const BRAND: &str = "brand";
pub const COMPUTE_MODE: &str = "compute_mode";
pub const PCIE_GEN_MAX: &str = "pcie_gen_max";
pub const PCIE_WIDTH_MAX: &str = "pcie_width_max";

pub const ARR_SUPPORTED: &str = "arr_supported";
pub const ECC_MODE_CURRENT: &str = "ecc_mode_current";
pub const ECC_MODE_PENDING: &str = "ecc_mode_pending";
pub const GPU_TYPE: &str = "gpu_type";
pub const INTERCONNECT: &str = "interconnect";
pub const JETPACK_VERSION: &str = "jetpack_version";
pub const L4T_VERSION: &str = "l4t_version";
pub const MIG_MODE_CURRENT: &str = "mig_mode_current";
pub const MIG_MODE_PENDING: &str = "mig_mode_pending";
pub const POWER_LIMIT_MIN: &str = "power_limit_min";
pub const VGPU_CAPABLE: &str = "vgpu_capable";

// ---- AMD ----

pub const ASIC_NAME: &str = "asic_name";
pub const CURRENT_LINK: &str = "current_link";
pub const HOTSPOT_TEMPERATURE: &str = "hotspot_temperature";
pub const MAX_DPM_LINK: &str = "max_dpm_link";
pub const MAX_GPU_LINK: &str = "max_gpu_link";
pub const MAX_SYSTEM_LINK: &str = "max_system_link";
pub const MEMORY_CLOCK: &str = "memory_clock";
pub const MEMORY_CONTROLLER_ACTIVITY: &str = "memory_controller_activity";
pub const MEMORY_TEMPERATURE: &str = "memory_temperature";
pub const MIN_DPM_LINK: &str = "min_dpm_link";
pub const POWER_CAP: &str = "power_cap";
pub const POWER_CAP_MAX: &str = "power_cap_max";
pub const POWER_CAP_MIN: &str = "power_cap_min";
pub const REVISION_ID: &str = "revision_id";
pub const VBIOS_DATE: &str = "vbios_date";
pub const VBIOS_VERSION: &str = "vbios_version";

// ---- Intel ----

pub const DRIVER: &str = "driver";
pub const MEMORY: &str = "memory";
pub const MEMORY_L0: &str = "memory_l0";
pub const PCI_BUS: &str = "pci_bus";
pub const POWER_L0: &str = "power_l0";
pub const SYCL_CAPABLE: &str = "sycl_capable";
pub const VARIANT: &str = "variant";
pub const VRAM_DEDICATED_L0: &str = "vram_dedicated_l0";
pub const VRAM_TOTAL: &str = "vram_total";

// ---- Google TPU ----
pub const TPU_RUNTIME_VERSION: &str = "tpu_runtime_version";

pub const ACCELERATOR_TYPE: &str = "accelerator_type";
pub const CHIP_VERSION: &str = "chip_version";
pub const HLO_EXEC_MEAN: &str = "hlo_exec_mean";
pub const HLO_EXEC_P50: &str = "hlo_exec_p50";
pub const HLO_EXEC_P90: &str = "hlo_exec_p90";
pub const HLO_EXEC_P95: &str = "hlo_exec_p95";
pub const HLO_EXEC_P99_9: &str = "hlo_exec_p99_9";
pub const TENSORCORE_COUNT: &str = "tensorcore_count";

// ---- Tenstorrent ----
pub const BOARD_ID: &str = "board_id";
pub const BOARD_TYPE: &str = "board_type";
pub const ETH_FW_VERSION: &str = "eth_fw_version";
pub const FW_DATE: &str = "fw_date";
pub const PCIE_DEVICE_ID: &str = "pcie_device_id";
pub const PCIE_VENDOR_ID: &str = "pcie_vendor_id";

pub const AI_CLOCK: &str = "ai_clock";
pub const AICLK_MHZ: &str = "aiclk_mhz";
pub const ARC0_HEALTH: &str = "arc0_health";
pub const ARC3_HEALTH: &str = "arc3_health";
pub const ARC_CLOCK: &str = "arc_clock";
pub const ARC_FW_VERSION: &str = "arc_fw_version";
pub const ARCCLK_MHZ: &str = "arcclk_mhz";
pub const ASIC_TEMPERATURE: &str = "asic_temperature";
pub const AXI_CLOCK: &str = "axi_clock";
pub const AXICLK_MHZ: &str = "axiclk_mhz";
pub const COLLECTION_METHOD: &str = "collection_method";
pub const CURRENT: &str = "current";
pub const DDR_FW_VERSION: &str = "ddr_fw_version";
pub const DDR_STATUS: &str = "ddr_status";
pub const DRAM_SPEED: &str = "dram_speed";
pub const ETH_STATUS0: &str = "eth_status0";
pub const ETH_STATUS1: &str = "eth_status1";
pub const FAN_RPM: &str = "fan_rpm";
pub const FAULTS: &str = "faults";
pub const HEARTBEAT: &str = "heartbeat";
pub const INLET_TEMPERATURE: &str = "inlet_temperature";
pub const OUTLET_TEMPERATURE1: &str = "outlet_temperature1";
pub const OUTLET_TEMPERATURE2: &str = "outlet_temperature2";
pub const PCIE_ADDRESS: &str = "pcie_address";
pub const PCIE_LINK_GEN: &str = "pcie_link_gen";
pub const PCIE_STATUS: &str = "pcie_status";
pub const POWER_LIMIT_TDC: &str = "power_limit_tdc";
pub const POWER_LIMIT_TDP: &str = "power_limit_tdp";
pub const POWER_WATTS: &str = "power_watts";
pub const SPIBOOTROM_FW_VERSION: &str = "spibootrom_fw_version";
pub const TDC_LIMIT: &str = "tdc_limit";
pub const TDP_LIMIT: &str = "tdp_limit";
pub const THERMAL_LIMIT: &str = "thermal_limit";
pub const THROTTLER: &str = "throttler";
pub const VDD_VOLTAGE: &str = "vdd_voltage";
pub const VOLTAGE: &str = "voltage";
pub const VR_TEMPERATURE: &str = "vr_temperature";
pub const VREG_TEMPERATURE: &str = "vreg_temperature";

// ---- Furiosa ----
pub const MEMORY_BANDWIDTH: &str = "memory_bandwidth";
pub const ON_CHIP_SRAM: &str = "on_chip_sram";
pub const PCI_BDF: &str = "pci_bdf";
pub const PCI_DEV: &str = "pci_dev";
pub const PE_COUNT: &str = "pe_count";

pub const BDF: &str = "bdf";
pub const CHIP_NAME: &str = "chip_name";
pub const CLOCK_MHZ: &str = "clock_mhz";
pub const COMPUTE_UTILIZATION: &str = "compute_utilization";
pub const FREQUENCY: &str = "frequency";
pub const GOVERNOR: &str = "governor";
pub const MEMORY_CLOCK_MHZ: &str = "memory_clock_mhz";
pub const MEMORY_TOTAL: &str = "memory_total";
pub const MEMORY_USED: &str = "memory_used";
pub const NUMA_NODE: &str = "numa_node";
pub const PERT_VERSION: &str = "pert_version";
pub const SERIAL_NUMBER: &str = "serial_number";

// ---- Rebellions ----
pub const BOARD_INFO: &str = "board_info";
pub const DEVICE_PATH: &str = "device_path";
pub const PCI_LINK_SPEED: &str = "pci_link_speed";
pub const PCI_NUMA_NODE: &str = "pci_numa_node";
pub const SERIAL_ID: &str = "serial_id";

pub const KMD_VERSION: &str = "kmd_version";

// ---- Habana Gaudi ----

pub const FREE_MEMORY: &str = "free_memory";
pub const INTERNAL_NAME: &str = "internal_name";

// ---- Apple Silicon / chassis ----

pub const ANE_POWER_WATTS: &str = "ane_power_watts";
pub const API: &str = "api";
pub const BIOS_VERSION: &str = "bios_version";
pub const BOARD: &str = "board";
pub const CPU_POWER_WATTS: &str = "cpu_power_watts";
pub const CPU_TEMPERATURE: &str = "cpu_temperature";
pub const E_CLUSTER_FREQ_MHZ: &str = "e_cluster_freq_mhz";
pub const GPU_CORE_COUNT: &str = "gpu_core_count";
pub const GPU_POWER_WATTS: &str = "gpu_power_watts";
pub const GPU_TEMPERATURE: &str = "gpu_temperature";
pub const P_CLUSTER_FREQ_MHZ: &str = "p_cluster_freq_mhz";
pub const PLATFORM: &str = "platform";
pub const POWER_SOURCE: &str = "power_source";
pub const PRODUCT_NAME: &str = "product_name";
pub const VENDOR: &str = "vendor";
pub const VERSION: &str = "version";

/// Every canonical key, for the round-trip test below.
#[cfg(test)]
const ALL: &[&str] = &[
    ACCELERATOR_TYPE,
    AICLK_MHZ,
    AI_CLOCK,
    ANE_POWER_WATTS,
    API,
    ARC0_HEALTH,
    ARC3_HEALTH,
    ARCCLK_MHZ,
    ARCHITECTURE,
    ARC_CLOCK,
    ARC_FW_VERSION,
    ARR_SUPPORTED,
    ASIC_NAME,
    ASIC_TEMPERATURE,
    AXICLK_MHZ,
    AXI_CLOCK,
    BDF,
    BIOS_VERSION,
    BOARD,
    BOARD_ID,
    BOARD_INFO,
    BOARD_TYPE,
    BRAND,
    CHIP_NAME,
    CHIP_VERSION,
    CLOCK_GRAPHICS_MAX,
    CLOCK_MEMORY_MAX,
    CLOCK_MHZ,
    COLLECTION_METHOD,
    COMBINED_POWER_MW,
    COMPUTE_MODE,
    COMPUTE_UTILIZATION,
    CORE_COUNT,
    CPU_POWER_WATTS,
    CPU_TEMPERATURE,
    CUDA_VERSION,
    CURRENT,
    CURRENT_LINK,
    CURRENT_POWER,
    DAC_TYPE,
    DDR_FW_VERSION,
    DDR_STATUS,
    DEVICE_ID,
    DEVICE_INDEX,
    DEVICE_NAME,
    DEVICE_PATH,
    DRAM_SPEED,
    DRIVER,
    DRIVER_VERSION,
    ECC_MODE_CURRENT,
    ECC_MODE_PENDING,
    ENGINE_3D,
    ETH_FW_VERSION,
    ETH_STATUS0,
    ETH_STATUS1,
    E_CLUSTER_FREQ_MHZ,
    FAN_RPM,
    FAN_SPEED,
    FAULTS,
    FIRMWARE,
    FIRMWARE_VERSION,
    FREE_MEMORY,
    FREQUENCY,
    FW_DATE,
    GOVERNOR,
    GPU_CORE_COUNT,
    GPU_POWER_WATTS,
    GPU_TEMPERATURE,
    GPU_TEMP_LIMIT,
    GPU_TYPE,
    HEARTBEAT,
    HLO_EXEC_MEAN,
    HLO_EXEC_P50,
    HLO_EXEC_P90,
    HLO_EXEC_P95,
    HLO_EXEC_P99_9,
    HLO_QUEUE_SIZE,
    HOTSPOT_TEMPERATURE,
    INDEX,
    INLET_TEMPERATURE,
    INSTANCE_NAME,
    INTERCONNECT,
    INTERNAL_NAME,
    JETPACK_VERSION,
    KMD_VERSION,
    L4T_VERSION,
    LIB_NAME,
    LIB_VERSION,
    MAX_DPM_LINK,
    MAX_GPU_LINK,
    MAX_POWER,
    MAX_SYSTEM_LINK,
    MEMORY,
    MEMORY_BANDWIDTH,
    MEMORY_CLOCK,
    MEMORY_CLOCK_MHZ,
    MEMORY_CONTROLLER_ACTIVITY,
    MEMORY_L0,
    MEMORY_TEMPERATURE,
    MEMORY_TOTAL,
    MEMORY_TYPE,
    MEMORY_USED,
    METRICS_AVAILABLE,
    METRICS_SOURCE,
    MIG_MODE_CURRENT,
    MIG_MODE_PENDING,
    MIN_DPM_LINK,
    NAME,
    NATIVE_METRICS,
    NOTE,
    NUMA_NODE,
    ON_CHIP_SRAM,
    OUTLET_TEMPERATURE1,
    OUTLET_TEMPERATURE2,
    PCIE_ADDRESS,
    PCIE_DEVICE_ID,
    PCIE_GENERATION,
    PCIE_GEN_MAX,
    PCIE_LINK_GEN,
    PCIE_LINK_WIDTH,
    PCIE_STATUS,
    PCIE_VENDOR_ID,
    PCIE_WIDTH_MAX,
    PCI_BDF,
    PCI_BUS,
    PCI_BUS_ID,
    PCI_DEV,
    PCI_LINK_SPEED,
    PCI_NUMA_NODE,
    PERFORMANCE_STATE,
    PERT_VERSION,
    PE_COUNT,
    PLATFORM,
    POWER,
    POWER_CAP,
    POWER_CAP_MAX,
    POWER_CAP_MIN,
    POWER_DRAW,
    POWER_L0,
    POWER_LIMIT_CURRENT,
    POWER_LIMIT_DEFAULT,
    POWER_LIMIT_MAX,
    POWER_LIMIT_MIN,
    POWER_LIMIT_TDC,
    POWER_LIMIT_TDP,
    POWER_SOURCE,
    POWER_WATTS,
    PRODUCT_NAME,
    P_CLUSTER_FREQ_MHZ,
    REVISION_ID,
    ROCM_VERSION,
    SERIAL_ID,
    SERIAL_NUMBER,
    SOURCE_FAN,
    SOURCE_FREQUENCY,
    SOURCE_MEMORY,
    SOURCE_MEMORY_USED,
    SOURCE_POWER,
    SOURCE_TEMPERATURE,
    SOURCE_UTILIZATION,
    SPIBOOTROM_FW_VERSION,
    STATUS,
    SYCL_CAPABLE,
    TDC_LIMIT,
    TDP_LIMIT,
    TEMPERATURE,
    TENSORCORE_COUNT,
    THERMAL_LIMIT,
    THERMAL_PRESSURE,
    THROTTLER,
    TOTAL_MEMORY,
    TPU_RUNTIME_VERSION,
    TRANSPORT,
    USED_MEMORY,
    UTILIZATION,
    UUID,
    VARIANT,
    VBIOS_DATE,
    VBIOS_VERSION,
    VDD_VOLTAGE,
    VENDOR,
    VERSION,
    VGPU_CAPABLE,
    VIDEO_PROCESSOR,
    VOLTAGE,
    VRAM_BUDGET_PROCESS,
    VRAM_DEDICATED_L0,
    VRAM_TOTAL,
    VRAM_USAGE_PROCESS,
    VREG_TEMPERATURE,
    VR_TEMPERATURE,
];

/// Key for a per-engine utilization reading, e.g. `engine_render`.
pub fn engine(class: &str) -> String {
    format!("engine_{}", slug(class))
}

/// Key for a per-engine utilization reading sampled through Level Zero.
pub fn engine_level_zero(label: &str) -> String {
    format!("engine_{}_l0", slug(label))
}

/// Key carrying the version of the vendor library named `name`, e.g.
/// `cuda_version` for `"CUDA"`.
pub fn library_version(name: &str) -> String {
    format!("{}_version", slug(name))
}

/// Key recording which backend supplied `field`, e.g. `source_fan`.
pub fn source(field: &str) -> String {
    format!("source_{}", slug(field))
}

/// Lowercase `s` and collapse every run of non-alphanumeric bytes to a single
/// `_`, so a dynamically built key obeys the same spelling rule as a constant.
fn slug(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::common::sanitize_label_name;
    use regex::Regex;

    /// The exporter derives a Prometheus label from every `detail` key by
    /// sanitizing it, and the remote parser feeds those labels back into
    /// `detail`. A key that does not survive that round trip is spelled one way
    /// locally and another way on a viewer, which is the class of bug this
    /// module exists to prevent -- so every constant must be its own sanitized
    /// form.
    #[test]
    fn keys_round_trip_through_the_exporter() {
        for key in ALL {
            assert_eq!(
                &sanitize_label_name(key),
                key,
                "detail key {key:?} is not stable under sanitize_label_name"
            );
        }
    }

    /// Walk `src/` and collect every string literal sitting in a `detail`-key
    /// position, so the check does not depend on a list of keys that is itself
    /// derived from the code being checked -- the circularity that let
    /// `"PCIe Width Max"` and fifteen chained `DetailBuilder` keys survive an
    /// earlier normalization pass unnoticed.
    fn literal_keys_in(source: &str) -> Vec<String> {
        // Comment lines legitimately name keys in prose (`detail["memory"]`).
        let code: String = source
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");

        let mut found = Vec::new();
        let mut push = |k: &str| found.push(k.to_string());

        // A named `detail` / `builder` receiver, and `detail["key"]` indexing.
        for pat in [
            r#"(?s)(?:detail\w*|\.detail|builder\w*)\s*\.\s*(?:insert|insert_optional|get|contains_key|entry|remove)\s*\(\s*"([^"]*)""#,
            r#"(?s)detail\w*\s*\[\s*"([^"]*)""#,
        ] {
            for c in Regex::new(pat).unwrap().captures_iter(&code) {
                push(&c[1]);
            }
        }

        // A fluent chain, where the receiver of `.insert` is the previous call
        // rather than anything named `detail`.
        let chain_key =
            Regex::new(r#"(?s)\.\s*(?:insert|insert_optional)\s*\(\s*"([^"]*)""#).unwrap();
        for m in Regex::new(r"DetailBuilder::new\(\)")
            .unwrap()
            .find_iter(&code)
        {
            let rest = &code[m.end()..];
            let span = &rest[..rest.find(';').unwrap_or(rest.len())];
            for c in chain_key.captures_iter(span) {
                push(&c[1]);
            }
        }

        // The `add_detail!` family takes its key as a bare macro argument,
        // which none of the receiver-shaped patterns above can see.
        let string_lit = Regex::new(r#""([^"]*)""#).unwrap();
        for (macro_name, key_is_last) in [("add_detail_fmt!(", false), ("add_detail!(", true)] {
            let mut from = 0;
            while let Some(at) = code[from..].find(macro_name) {
                let open = from + at + macro_name.len() - 1;
                let mut depth = 0;
                let mut close = open;
                for (i, ch) in code[open..].char_indices() {
                    match ch {
                        '(' => depth += 1,
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                close = open + i;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                let span = &code[open..close];
                let mut lits: Vec<&str> = string_lit
                    .captures_iter(span)
                    .map(|c| c.get(1).unwrap().as_str())
                    .collect();
                // `add_detail_fmt!`'s trailing argument is the value format
                // string (`"x{}"`), not a key.
                if !key_is_last {
                    lits.pop();
                }
                for l in lits {
                    push(l);
                }
                from = close.max(from + at + macro_name.len());
            }
        }
        found
    }

    /// Every `detail` key must come from a constant in this module.
    ///
    /// The keys are a cross-vendor contract that nothing type-checks: a reader
    /// writing `"Board ID"` while its exporter reads `board_id` compiles
    /// cleanly and silently drops the metric, which is what several readers
    /// were doing. Constants make a rename compiler-checked; this test is what
    /// keeps a raw literal from quietly opting back out.
    #[test]
    fn no_reader_or_consumer_spells_a_key_by_hand() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut offenders: Vec<String> = Vec::new();
        let mut seen = std::collections::HashSet::new();
        let mut stack = vec![root.clone()];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read src/") {
                let path = entry.expect("dir entry").path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                // The registry itself is the one place a key may be spelled
                // out. Everywhere else, including the reader-side helpers,
                // has to go through it -- that is where the last one hid.
                if path.extension().is_none_or(|e| e != "rs") || path.ends_with("device/keys.rs") {
                    continue;
                }
                let source = std::fs::read_to_string(&path).expect("read source");
                for key in literal_keys_in(&source) {
                    let rel = path.strip_prefix(&root).unwrap_or(&path).display();
                    let offence = format!("{rel}: {key:?}");
                    if seen.insert(offence.clone()) {
                        offenders.push(offence);
                    }
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "detail keys spelled as literals instead of `device::keys` constants:\n  {}\n\n\
             Add a constant to `device::keys` and use it. If the literal is \
             deliberately arbitrary (a fixture, or a deliberately malformed key), bind \
             it to a local first so the intent is explicit.",
            offenders.join("\n  ")
        );
    }

    #[test]
    fn dynamic_keys_obey_the_same_rule() {
        for key in [
            engine("Render/3D"),
            engine_level_zero("Compute 0"),
            source("Memory Used"),
            library_version("ROCm"),
        ] {
            assert_eq!(&sanitize_label_name(&key), &key, "dynamic key {key:?}");
        }
        assert_eq!(engine("Render/3D"), "engine_render_3d");
        assert_eq!(engine_level_zero("Compute 0"), "engine_compute_0_l0");
        assert_eq!(source("Memory Used"), "source_memory_used");
        assert_eq!(library_version("CUDA"), "cuda_version");
        assert_eq!(library_version("ROCm"), "rocm_version");
    }
}
