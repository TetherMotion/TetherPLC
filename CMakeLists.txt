cmake_minimum_required(VERSION 3.16)
project(TetherPLC C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug)
endif()

add_compile_options(-Wall -Wextra)

include_directories(${CMAKE_SOURCE_DIR}/include)

enable_testing()

# ------------------------------------------------------------------ #
# Image library (SHM process image, both roles)
# ------------------------------------------------------------------ #
add_library(tplc_image STATIC runtime/image/tplc_image.c)
target_link_libraries(tplc_image PUBLIC rt)

# ------------------------------------------------------------------ #
# Host library (module loader + program + scheduler + services)
# ------------------------------------------------------------------ #
add_library(tplc_host STATIC
  runtime/host/src/Module.cpp
  runtime/host/src/ElfScan.cpp
  runtime/host/src/Sandbox.cpp
  runtime/host/src/AuditLog.cpp
  runtime/host/src/Program.cpp
  runtime/host/src/Scheduler.cpp
  runtime/host/src/RetainStore.cpp
  runtime/host/src/HostServices.cpp
  runtime/host/src/CtlServer.cpp)
target_include_directories(tplc_host PUBLIC runtime/host/include)
target_link_libraries(tplc_host PUBLIC tplc_image dl)

# ------------------------------------------------------------------ #
# Package library (.tplc C++ reader)
# ------------------------------------------------------------------ #
add_library(tplc_pkg STATIC
  runtime/pkg/src/Package.cpp
  runtime/pkg/src/Signature.cpp)
target_include_directories(tplc_pkg PUBLIC
  runtime/pkg/include runtime/host/include)

# ed25519 verification backend (doc 15 §4.1): optional — without it
# "require*"-policies fail closed on signed packages
find_library(TPLC_SODIUM_LIBSODIUM sodium)
if(TPLC_SODIUM_LIBSODIUM)
  target_compile_definitions(tplc_pkg PRIVATE TPLC_HAVE_SODIUM)
  target_link_libraries(tplc_pkg PUBLIC ${TPLC_SODIUM_LIBSODIUM})
  message(STATUS "tplc_pkg: signature verification via libsodium")
else()
  message(STATUS "tplc_pkg: no libsodium — signature policies fail closed")
endif()

# process sandbox syscall filter (doc 15 §3): optional — without
# libseccomp only rlimits apply
find_library(TPLC_SECCOMP_LIBSECCOMP seccomp)
if(TPLC_SECCOMP_LIBSECCOMP)
  target_compile_definitions(tplc_host PRIVATE TPLC_HAVE_SECCOMP)
  target_link_libraries(tplc_host PUBLIC ${TPLC_SECCOMP_LIBSECCOMP})
  message(STATUS "tplc_host: seccomp sandbox via libseccomp")
else()
  message(STATUS "tplc_host: no libseccomp — sandbox rlimits only")
endif()

# ------------------------------------------------------------------ #
# Runtime binary
# ------------------------------------------------------------------ #
add_executable(tether-plc runtime/tether-plc/main.cpp)
target_link_libraries(tether-plc PRIVATE tplc_host tplc_image tplc_pkg)
set_target_properties(tether-plc PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

# ------------------------------------------------------------------ #
# Tools
# ------------------------------------------------------------------ #
add_library(tplc_bridge STATIC
  runtime/bridge/src/ImageServer.cpp
  runtime/bridge/src/MapRunner.cpp
  runtime/bridge/src/Supervisor.cpp
  runtime/bridge/src/CatalogExporter.cpp)
target_include_directories(tplc_bridge PUBLIC runtime/bridge/include)
target_link_libraries(tplc_bridge PUBLIC tplc_image tplc_host)

add_executable(tether-plc-bridge runtime/tether-plc-bridge/main.cpp)
target_link_libraries(tether-plc-bridge PRIVATE tplc_bridge tplc_image tplc_pkg)
set_target_properties(tether-plc-bridge PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

add_executable(tether-plc-sim runtime/tether-plc-sim/main.cpp)
target_link_libraries(tether-plc-sim PRIVATE tplc_image)
set_target_properties(tether-plc-sim PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

add_executable(tplc-image tools/tplc_image_tool.c)
target_link_libraries(tplc-image PRIVATE tplc_image)

add_executable(tplc-ctl tools/tplc_ctl.c)
target_link_libraries(tplc-ctl PRIVATE tplc_image)
target_include_directories(tplc-ctl PRIVATE tools)

add_executable(tplc-ctl-proxy tools/tplc_ctl_proxy.cpp)

# ------------------------------------------------------------------ #
# Tests: stub modules (hand-written .so fixtures)
# ------------------------------------------------------------------ #
set(STUB_VERSION_SCRIPT ${CMAKE_SOURCE_DIR}/tests/abi/exports.map)

function(add_stub name)
  add_library(${name} MODULE tests/abi/stub_module.c)
  set_target_properties(${name} PROPERTIES
    PREFIX "" OUTPUT_NAME "${name}" SUFFIX ".so")
  target_include_directories(${name} PRIVATE tests/abi)
  target_compile_options(${name} PRIVATE
    -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables)
  target_link_options(${name} PRIVATE
    -Wl,--version-script=${STUB_VERSION_SCRIPT} -Wl,-z,defs)
  # stub .so fixtures model shipped modules: the post-link scanner
  # (ElfScan) rejects sanitizer-instrumented DT_NEEDED/imports, so
  # fixtures build un-instrumented even under sanitizer suite builds
  if(CMAKE_C_FLAGS MATCHES "sanitize")
    target_compile_options(${name} PRIVATE -fno-sanitize=all)
    target_link_options(${name} PRIVATE -fno-sanitize=all)
  endif()
endfunction()

add_stub(stub_v1)
target_compile_definitions(stub_v1 PRIVATE TPLC_STUB_V=1)
add_stub(stub_v2)
target_compile_definitions(stub_v2 PRIVATE TPLC_STUB_V=2)
add_stub(stub_bad_abi)
target_compile_definitions(stub_bad_abi PRIVATE TPLC_STUB_V=1 TPLC_STUB_BAD_ABI=1)

# ------------------------------------------------------------------ #
# Tests
# ------------------------------------------------------------------ #
add_executable(image_conformance tests/image/image_conformance.c)
target_include_directories(image_conformance PRIVATE tests)
target_link_libraries(image_conformance PRIVATE tplc_image)
add_test(NAME image_conformance COMMAND image_conformance)

add_executable(abi_conformance tests/abi/abi_conformance.cpp)
target_include_directories(abi_conformance PRIVATE tests tests/abi)
target_link_libraries(abi_conformance PRIVATE tplc_host tplc_image)
target_compile_definitions(abi_conformance PRIVATE
  STUB_V1_PATH="$<TARGET_FILE:stub_v1>"
  STUB_V2_PATH="$<TARGET_FILE:stub_v2>"
  STUB_BAD_ABI_PATH="$<TARGET_FILE:stub_bad_abi>")
add_dependencies(abi_conformance stub_v1 stub_v2 stub_bad_abi)
add_test(NAME abi_conformance COMMAND abi_conformance)

add_executable(ctl_smoke tests/runtime/ctl_smoke.cpp)
target_include_directories(ctl_smoke PRIVATE tests tools)
target_link_libraries(ctl_smoke PRIVATE tplc_host tplc_image)
target_compile_definitions(ctl_smoke PRIVATE
  TETHER_PLC_PATH="$<TARGET_FILE:tether-plc>"
  STUB_V1_PATH="$<TARGET_FILE:stub_v1>")
add_dependencies(ctl_smoke tether-plc stub_v1)
add_test(NAME ctl_smoke COMMAND ctl_smoke)

# post-link scanner fixtures (doc 15 §4.3): evil .so must be rejected,
# forbidden bytes inside immediates must NOT false-positive
function(add_scan_fixture name src)
  add_library(${name} MODULE tests/scan/${src})
  set_target_properties(${name} PROPERTIES
    PREFIX "" OUTPUT_NAME "${name}" SUFFIX ".so")
  target_compile_options(${name} PRIVATE
    -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables)
  if(CMAKE_C_FLAGS MATCHES "sanitize")
    target_compile_options(${name} PRIVATE -fno-sanitize=all)
    target_link_options(${name} PRIVATE -fno-sanitize=all)
  endif()
endfunction()

add_scan_fixture(scan_evil_syscall evil_syscall.c)
add_scan_fixture(scan_evil_int80   evil_int80.c)
add_scan_fixture(scan_evil_import  evil_import.c)
add_scan_fixture(scan_good_imm     good_immediates.c)

# sandbox-enforcement: a valid ABI module importing socket() — the
# import scanner must reject it (layer 1); and a probe binary that
# applies the sandbox then tests socket() families (layer 2)
add_library(scan_evil_inet MODULE tests/scan/evil_inet.c)
set_target_properties(scan_evil_inet PROPERTIES
  PREFIX "" OUTPUT_NAME "scan_evil_inet" SUFFIX ".so")
target_include_directories(scan_evil_inet PRIVATE include)
target_compile_options(scan_evil_inet PRIVATE
  -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables)
if(CMAKE_C_FLAGS MATCHES "sanitize")
  target_compile_options(scan_evil_inet PRIVATE -fno-sanitize=all)
  target_link_options(scan_evil_inet PRIVATE -fno-sanitize=all)
endif()

add_executable(sandbox_probe tests/scan/sandbox_probe.cpp)
target_link_libraries(sandbox_probe PRIVATE tplc_host)

add_executable(module_scan tests/scan/module_scan.cpp)
target_link_libraries(module_scan PRIVATE tplc_host)
add_dependencies(module_scan scan_evil_syscall scan_evil_int80
  scan_evil_import scan_evil_inet scan_good_imm stub_v1 stub_v2)
add_test(NAME sig_policy
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/scan/sig_policy.sh)
set_tests_properties(sig_policy PROPERTIES SKIP_RETURN_CODE 77)
add_test(NAME module_scan COMMAND module_scan
  $<TARGET_FILE:scan_evil_syscall> reject
  $<TARGET_FILE:scan_evil_int80>   reject
  $<TARGET_FILE:scan_evil_import>  reject
  $<TARGET_FILE:scan_evil_inet>    reject
  $<TARGET_FILE:scan_good_imm>     ok
  $<TARGET_FILE:stub_v1>           ok
  $<TARGET_FILE:stub_v2>           ok)

# doc 15 §3 acceptance: sandbox active, UDS-only socket gate enforced
add_test(NAME sandbox_enforce COMMAND bash
  ${CMAKE_SOURCE_DIR}/tests/scan/sandbox_enforce.sh
  $<TARGET_FILE:tether-plc> $<TARGET_FILE:tplc-image>
  $<TARGET_FILE:tplc-ctl>
  $<TARGET_FILE:sandbox_probe> $<TARGET_FILE:stub_v1>)
add_dependencies(sandbox_probe tether-plc tplc-image tplc-ctl)
set_tests_properties(sandbox_enforce PROPERTIES SKIP_RETURN_CODE 77)

# doc 15 §5 acceptance: role-gated ops, durable journal, rate limit
add_test(NAME authz_audit COMMAND bash
  ${CMAKE_SOURCE_DIR}/tests/scan/authz_audit.sh
  $<TARGET_FILE:tether-plc> $<TARGET_FILE:tplc-image>
  $<TARGET_FILE:tplc-ctl> $<TARGET_FILE:stub_v1>)
set_tests_properties(authz_audit PROPERTIES SKIP_RETURN_CODE 77)

# doc 15 §2 acceptance: safety channels deny force/write, reads stay open
add_test(NAME safety_boundary COMMAND bash
  ${CMAKE_SOURCE_DIR}/tests/scan/safety_boundary.sh)
set_tests_properties(safety_boundary PROPERTIES SKIP_RETURN_CODE 77)

# doc 16 §9: v2 status object, strict transitions, events, epoch
add_test(NAME mgmt_surface COMMAND bash
  ${CMAKE_SOURCE_DIR}/tests/scan/mgmt_surface.sh)
set_tests_properties(mgmt_surface PROPERTIES SKIP_RETURN_CODE 77)

# doc 15 §6: license manifest gate (skips when cargo metadata unusable)
add_test(NAME license_manifest COMMAND python3
  ${CMAKE_SOURCE_DIR}/tools/license_manifest.py --check
  --manifest /dev/null)
set_tests_properties(license_manifest PROPERTIES SKIP_RETURN_CODE 77)

add_executable(pkg_conformance tests/pkg/pkg_conformance.cpp)
target_link_libraries(pkg_conformance PRIVATE tplc_pkg)
target_compile_definitions(pkg_conformance PRIVATE
  PKG_FIXTURE_PATH="${CMAKE_SOURCE_DIR}/tests/pkg/fixtures/sample.tplc")
add_test(NAME pkg_conformance
  COMMAND pkg_conformance "${CMAKE_SOURCE_DIR}/tests/pkg/fixtures/sample.tplc")

add_executable(retain_store tests/runtime/retain_store.cpp)
target_include_directories(retain_store PRIVATE tests)
target_link_libraries(retain_store PRIVATE tplc_host)
add_test(NAME retain_store COMMAND retain_store)

add_executable(fault_injection tests/fault/fault_injection.cpp)
target_include_directories(fault_injection PRIVATE tests)
target_link_libraries(fault_injection PRIVATE tplc_host tplc_image)
target_compile_definitions(fault_injection PRIVATE
  TETHER_PLC_PATH="$<TARGET_FILE:tether-plc>"
  STUB_V1_PATH="$<TARGET_FILE:stub_v1>")
add_dependencies(fault_injection tether-plc stub_v1)
add_test(NAME fault_injection COMMAND fault_injection)

# online-change e2e: drives the real runtime + compiled packages;
# skips (rc 77) when tplcc/python3 are unavailable. Native-only: tplcc
# emits host-arch .so files that a cross-built tether-plc cannot load.
if(NOT CMAKE_CROSSCOMPILING)
  add_test(NAME online_change_e2e
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/online_change/e2e.sh)
  set_tests_properties(online_change_e2e PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120)

  # FBD gallery: ST/FBD pairs must produce identical %Q images
  add_test(NAME fbd_gallery_e2e
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/fbd_gallery/e2e.sh)
  set_tests_properties(fbd_gallery_e2e PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120)

  # debug-plane stress: boundary-consistent reads never tear a
  # multi-byte field mid-update (doc 12 §10.3)
  add_test(NAME debug_stress
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/debug/stress.sh)
  set_tests_properties(debug_stress PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120)

  # sim vectors (doc 14 §3.1): replay the oracle corpus through
  # tether-plc-sim incl. named field expectations via ctl DBG_READ
  add_test(NAME sim_vectors
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      python3 ${CMAKE_SOURCE_DIR}/tests/sim/vector_run.py)
  set_tests_properties(sim_vectors PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 180)

  # no-alloc property (doc 14 §5): LD_PRELOAD counter — steady-state
  # scans must not hit the heap
  add_test(NAME no_alloc
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/runtime/no_alloc.sh)
  set_tests_properties(no_alloc PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 60)

  # sanitizer passes (doc 14 §5): ASan+UBSan conformance, TSan on the
  # seqlock image protocol — builds dedicated build-san-* trees
  add_test(NAME sanitizers
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/sanitizers.sh)
  set_tests_properties(sanitizers PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 600)

  # differential fuzz vs the matiec oracle (doc 14 §3): downloads and
  # builds iec2c into a cache outside the repo on first run (subprocess
  # oracle only — never linked or vendored)
  add_test(NAME matiec_fuzz
    COMMAND bash -c
      "bash ${CMAKE_SOURCE_DIR}/tools/oracle-matiec/ensure_matiec.sh >/dev/null \
       || exit 77; \
       python3 ${CMAKE_SOURCE_DIR}/tools/oracle-matiec/diff_fuzz.py")
  set_tests_properties(matiec_fuzz PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 900)

  # determinism contract (doc 14 §2): same input sequence twice must
  # give identical traces at both legs
  add_test(NAME sim_determinism
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      python3 ${CMAKE_SOURCE_DIR}/tests/sim/vector_run.py --repeat 2)
  set_tests_properties(sim_determinism PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 300)

  # GPL quarantine scan (doc 14 §3/§4): no oracle coupling may reach
  # shipped paths
  add_test(NAME gpl_hygiene
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/gpl_hygiene.sh)
  set_tests_properties(gpl_hygiene PROPERTIES TIMEOUT 60)

  # ST frontend robustness fuzz (doc 14 §4/§5): mutated corpus must
  # produce diagnostics, never crashers
  add_test(NAME st_fuzz
    COMMAND python3 ${CMAKE_SOURCE_DIR}/tests/frontend/st_fuzz.py)
  set_tests_properties(st_fuzz PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 300)
endif()
