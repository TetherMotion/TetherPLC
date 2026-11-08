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
add_library(tplc_pkg STATIC runtime/pkg/src/Package.cpp)
target_include_directories(tplc_pkg PUBLIC
  runtime/pkg/include runtime/host/include)

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
endif()
