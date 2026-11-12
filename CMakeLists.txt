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
  runtime/host/src/NetService.cpp
  runtime/host/src/SerialService.cpp
  runtime/host/src/AdsService.cpp
  runtime/host/src/RtmonService.cpp
  runtime/host/src/ModbusService.cpp
  runtime/host/src/SacnService.cpp
  runtime/host/src/BacnetService.cpp
  runtime/host/src/PjlinkService.cpp
  runtime/host/src/S7Service.cpp
  runtime/host/src/FtpService.cpp
  runtime/host/src/SmtpService.cpp
  runtime/host/src/Iec870Service.cpp
  runtime/host/src/MotionService.cpp
  runtime/host/src/MotionIoService.cpp
  runtime/host/src/NciIoService.cpp
  runtime/host/src/OpcUaService.cpp
  runtime/host/src/EipService.cpp
  runtime/host/src/Iec61850Service.cpp
  runtime/host/src/MqttService.cpp
  runtime/host/src/OcppService.cpp
  runtime/host/src/Aes70Service.cpp
  runtime/host/src/DmxService.cpp
  runtime/host/src/CtlServer.cpp)
target_include_directories(tplc_host PUBLIC runtime/host/include)
target_link_libraries(tplc_host PUBLIC tplc_image dl)

# ------------------------------------------------------------------ #
# Package library (.tplc C++ reader)
# ------------------------------------------------------------------ #
add_library(tplc_pkg STATIC
  runtime/pkg/src/Package.cpp
  runtime/pkg/src/HiddenState.cpp
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

# OPC UA client service (tplc-opcua, C7): optional — without
# libopen62541 the service compiles inert and getService("tplc-opcua")
# returns nullptr.  -DTPLC_ENABLE_OPCUA=OFF forces it off.
option(TPLC_ENABLE_OPCUA
  "Enable the tplc-opcua OPC UA client service (needs libopen62541-dev)" ON)
if(TPLC_ENABLE_OPCUA)
  find_path(TPLC_OPEN62541_INCLUDE_DIR NAMES open62541/client.h)
  find_library(TPLC_OPEN62541_LIBRARY NAMES open62541)
  if(TPLC_OPEN62541_INCLUDE_DIR AND TPLC_OPEN62541_LIBRARY)
    target_compile_definitions(tplc_host PRIVATE TPLC_HAVE_OPCUA)
    target_include_directories(tplc_host PRIVATE ${TPLC_OPEN62541_INCLUDE_DIR})
    target_link_libraries(tplc_host PUBLIC ${TPLC_OPEN62541_LIBRARY})
    message(STATUS "tplc_host: OPC UA client via libopen62541")
  else()
    message(STATUS "tplc_host: no libopen62541 — tplc-opcua inactive")
  endif()
endif()

# IEC 61850 MMS client service (tplc-iec61850, C7): optional — without
# libiec61850 the service compiles inert and getService("tplc-iec61850")
# returns nullptr.  -DTPLC_ENABLE_IEC61850=OFF forces it off.  The
# library is auto-downloaded and built like the other optional deps.
option(TPLC_ENABLE_IEC61850
  "Enable the tplc-iec61850 IEC 61850 MMS client service (libiec61850)" ON)
if(TPLC_ENABLE_IEC61850)
  find_library(TPLC_IEC61850_LIBRARY NAMES iec61850)
  find_path(TPLC_IEC61850_INCLUDE_DIR NAMES iec61850_client.h)
  if(NOT TPLC_IEC61850_LIBRARY OR NOT TPLC_IEC61850_INCLUDE_DIR)
    # not installed — auto-download and build it
    include(FetchContent)
    FetchContent_Declare(libiec61850
      GIT_REPOSITORY https://github.com/mz-automation/libiec61850.git
      GIT_TAG        v1.5.3
      GIT_SHALLOW    TRUE)
    set(BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
    set(BUILD_TESTS OFF CACHE BOOL "" FORCE)
    FetchContent_MakeAvailable(libiec61850)
    if(TARGET iec61850)
      set(TPLC_IEC61850_BUILT TRUE)
      message(STATUS "tplc_host: IEC 61850 client via fetched libiec61850")
    endif()
  else()
    set(TPLC_IEC61850_BUILT TRUE)
    message(STATUS "tplc_host: IEC 61850 client via system libiec61850")
  endif()
  if(TPLC_IEC61850_BUILT)
    target_compile_definitions(tplc_host PRIVATE TPLC_HAVE_IEC61850)
    target_link_libraries(tplc_host PUBLIC iec61850)
    set(TPLC_IEC61850_INCDIRS
      ${libiec61850_SOURCE_DIR}/src/iec61850/inc
      ${libiec61850_SOURCE_DIR}/src/mms/inc
      ${libiec61850_SOURCE_DIR}/src/common/inc
      ${libiec61850_SOURCE_DIR}/src/logging
      ${libiec61850_SOURCE_DIR}/hal/inc)
    target_include_directories(tplc_host PRIVATE ${TPLC_IEC61850_INCDIRS})
    if(TPLC_IEC61850_INCLUDE_DIR)
      target_include_directories(tplc_host PRIVATE ${TPLC_IEC61850_INCLUDE_DIR})
    endif()
    # the fake IED server for the e2e reuses the same library
    add_executable(iec61850_ied_fake tests/compat/iec61850_ied_fake.cpp)
    target_link_libraries(iec61850_ied_fake PRIVATE iec61850)
    target_include_directories(iec61850_ied_fake PRIVATE ${TPLC_IEC61850_INCDIRS})
    set(TPLC_IEC61850_FAKE_TARGET iec61850_ied_fake)
  else()
    message(STATUS "tplc_host: no libiec61850 — tplc-iec61850 inactive")
  endif()
endif()

# MQTT client service (tplc-mqtt, C7): optional — without
# libpaho-mqtt3c the service compiles inert and getService("tplc-mqtt")
# returns nullptr.  -DTPLC_ENABLE_MQTT=OFF forces it off.
option(TPLC_ENABLE_MQTT
  "Enable the tplc-mqtt MQTT client service (libpaho-mqtt3c)" ON)
if(TPLC_ENABLE_MQTT)
  find_library(TPLC_PAHO_LIBRARY NAMES paho-mqtt3c paho-mqtt3cs)
  find_path(TPLC_PAHO_INCLUDE_DIR NAMES MQTTClient.h)
  if(NOT TPLC_PAHO_LIBRARY OR NOT TPLC_PAHO_INCLUDE_DIR)
    # not installed — auto-download and build paho.mqtt.c
    include(FetchContent)
    FetchContent_Declare(pahomqttc
      GIT_REPOSITORY https://github.com/eclipse/paho.mqtt.c.git
      GIT_TAG        v1.3.14
      GIT_SHALLOW    TRUE)
    set(PAHO_ENABLE_TESTING OFF CACHE BOOL "" FORCE)
    set(PAHO_BUILD_STATIC   ON  CACHE BOOL "" FORCE)
    set(PAHO_BUILD_SAMPLES  OFF CACHE BOOL "" FORCE)
    FetchContent_MakeAvailable(pahomqttc)
    if(TARGET paho-mqtt3c-static OR TARGET paho-mqtt3c)
      set(TPLC_PAHO_BUILT TRUE)
      set(TPLC_PAHO_FROM_FETCH TRUE)
      message(STATUS "tplc_host: MQTT client via fetched paho.mqtt.c")
    endif()
  else()
    set(TPLC_PAHO_BUILT TRUE)
    message(STATUS "tplc_host: MQTT client via system libpaho-mqtt3c")
  endif()
  if(TPLC_PAHO_BUILT)
    target_compile_definitions(tplc_host PRIVATE TPLC_HAVE_MQTT)
    if(TARGET paho-mqtt3c-static)
      target_link_libraries(tplc_host PUBLIC paho-mqtt3c-static)
    elseif(TARGET paho-mqtt3c)
      target_link_libraries(tplc_host PUBLIC paho-mqtt3c)
    else()
      target_link_libraries(tplc_host PUBLIC ${TPLC_PAHO_LIBRARY})
    endif()
    target_include_directories(tplc_host PRIVATE
      ${pahomqttc_SOURCE_DIR}/src)
    if(TPLC_PAHO_INCLUDE_DIR)
      target_include_directories(tplc_host PRIVATE ${TPLC_PAHO_INCLUDE_DIR})
    endif()
  else()
    message(STATUS "tplc_host: no libpaho-mqtt3c — tplc-mqtt inactive")
  endif()
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
# tplc-io: tether_io server exposing the ctl.sock surface.
# Compiles a sliced tether_io_protocol from a sibling Tether checkout
# (server + TCP + schema + streaming; no client/ws/serial). Skipped
# silently when Tether is not checked out next to this repo.
# ------------------------------------------------------------------ #
set(TETHER_DIR "" CACHE PATH "Tether source checkout (for tether_io)")
if(NOT TETHER_DIR)
  foreach(cand "${CMAKE_SOURCE_DIR}/../Tether" "$ENV{TETHER_DIR}")
    if(EXISTS "${cand}/include/tether/io/Server.hpp")
      set(TETHER_DIR "${cand}")
      break()
    endif()
  endforeach()
endif()

if(EXISTS "${TETHER_DIR}/include/tether/io/Server.hpp" AND
   EXISTS "${TETHER_DIR}/dependencies/libSLIPspeed/include" AND
   EXISTS "${TETHER_DIR}/dependencies/blake3/c/blake3.c")
  set(_io ${TETHER_DIR}/src/io)
  set(_slip ${TETHER_DIR}/dependencies/libSLIPspeed)
  set(_b3 ${TETHER_DIR}/dependencies/blake3/c)
  add_library(tplc_tether_io STATIC
    ${_io}/Protocol.cpp
    ${_io}/Registry.cpp
    ${_io}/SlipTransport.cpp
    ${_io}/SchemaDigest.cpp
    ${_io}/ThresholdFilter.cpp
    ${_io}/Datalogging.cpp
    ${_io}/Session.cpp
    ${_io}/Session_handlers.cpp
    ${_io}/Session_peer.cpp
    ${_io}/Session_streaming.cpp
    ${_io}/Server.cpp
    ${_io}/TcpTransport.cpp
    ${_io}/TetherIOClient.cpp
    ${TETHER_DIR}/src/logging/Logger.cpp
    ${TETHER_DIR}/src/logging/DeduplicatingLogger.cpp
    ${_b3}/blake3.c ${_b3}/blake3_dispatch.c ${_b3}/blake3_portable.c
    ${_slip}/src/Buffer.cpp ${_slip}/src/Decoder.cpp ${_slip}/src/Encoder.cpp)
  target_include_directories(tplc_tether_io PUBLIC
    ${TETHER_DIR}/include ${TETHER_DIR}/include/tether
    ${_b3} ${_slip}/include)
  target_include_directories(tplc_tether_io PRIVATE ${TETHER_DIR}/src)
  target_compile_definitions(tplc_tether_io PUBLIC
    TETHER_IO_TRANSPORT_WS=0 TETHER_IO_TRANSPORT_SERIAL=0)
  target_compile_definitions(tplc_tether_io PRIVATE
    BLAKE3_NO_SSE2 BLAKE3_NO_SSE41 BLAKE3_NO_AVX2 BLAKE3_NO_AVX512)
  set_target_properties(tplc_tether_io PROPERTIES
    CXX_STANDARD 23 CXX_STANDARD_REQUIRED ON)

  add_executable(tplc-io runtime/tplc-io/main.cpp)
  target_link_libraries(tplc-io PRIVATE tplc_tether_io)
  target_include_directories(tplc-io PRIVATE runtime/tplc-io)
  set_target_properties(tplc-io PROPERTIES
    CXX_STANDARD 23 CXX_STANDARD_REQUIRED ON
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

  # probe client: drives a live tplc-io for the ctest acceptance
  add_executable(tplc-io-probe runtime/tplc-io/probe.cpp)
  target_link_libraries(tplc-io-probe PRIVATE tplc_tether_io)
  set_target_properties(tplc-io-probe PROPERTIES
    CXX_STANDARD 23 CXX_STANDARD_REQUIRED ON
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

  # generic tether_io client: list/get/set catalog entries on any
  # tether_io TCP server (slave chain, ec-pump, tplc-io)
  add_executable(tplc-io-cli runtime/tplc-io/cli.cpp)
  target_link_libraries(tplc-io-cli PRIVATE tplc_tether_io)
  target_include_directories(tplc-io-cli PRIVATE runtime/tplc-io)
  set_target_properties(tplc-io-cli PROPERTIES
    CXX_STANDARD 23 CXX_STANDARD_REQUIRED ON
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
  message(STATUS "tplc-io: tether_io provider enabled (TETHER_DIR=${TETHER_DIR})")
else()
  message(STATUS "tplc-io: skipped (set TETHER_DIR to a Tether checkout)")
endif()

# tplc_host: enable the IO-backed tplc-motion backend (MotionIoService)
# when the sliced Tether IO static lib is available. Point it at a
# Tether IO endpoint with TPLC_MOTION_IO; without it the in-process
# MotionService remains the backend.
if(TARGET tplc_tether_io)
  set_source_files_properties(runtime/host/src/MotionIoService.cpp
                              runtime/host/src/NciIoService.cpp
    PROPERTIES COMPILE_OPTIONS "-std=c++23")
  target_compile_definitions(tplc_host PRIVATE TPLC_HAVE_TETHER_IO)
  target_link_libraries(tplc_host PUBLIC tplc_tether_io)
  message(STATUS "tplc_host: Tether IO motion backend enabled")
endif()

# ------------------------------------------------------------------ #
# tplc-ec-pump: EtherCAT master pump bridging the emulated Beckhoff
# fleet's PDO entries to the bridge simdir transport.  Includes the
# fleet/master helpers from the sibling Tether checkout's examples/
# and links its shared component libraries, so it needs a configured
# Tether build tree (build/include + build/lib).
# ------------------------------------------------------------------ #
set(TETHER_BUILD_DIR "${TETHER_DIR}/build" CACHE PATH
    "Tether build tree (for tplc-ec-pump)")
set(_ec_pump_ok FALSE)
if(TETHER_DIR AND IS_DIRECTORY "${TETHER_DIR}/examples/common" AND
   EXISTS "${TETHER_BUILD_DIR}/include/tether/TetherConfig.hpp" AND
   EXISTS "${TETHER_BUILD_DIR}/lib/libtether_ethercat_master_shared.so")
  set(_ec_pump_ok TRUE)
endif()
if(_ec_pump_ok AND TARGET tplc_tether_io)
  add_executable(tplc-ec-pump runtime/ecpump/main.cpp)
  target_include_directories(tplc-ec-pump PRIVATE
    ${TETHER_DIR}/include
    ${TETHER_DIR}/include/tether
    ${TETHER_DIR}/build/include
    ${TETHER_DIR}/examples
    ${TETHER_DIR}/dependencies/blake3/c
    ${TETHER_DIR}/dependencies/libSLIPspeed/include
    ${TETHER_DIR}/dependencies/atomic_queue/include
    runtime/tplc-io)
  target_link_libraries(tplc-ec-pump PRIVATE tplc_tether_io)
  foreach(lib ethercat_master ethercat_common hal common)
    target_link_libraries(tplc-ec-pump PRIVATE
      ${TETHER_BUILD_DIR}/lib/libtether_${lib}_shared.so)
  endforeach()
  set_target_properties(tplc-ec-pump PROPERTIES
    CXX_STANDARD 23 CXX_STANDARD_REQUIRED ON
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
    BUILD_RPATH "${TETHER_BUILD_DIR}/lib")
  message(STATUS "tplc-ec-pump: EtherCAT fleet pump enabled (TETHER_DIR=${TETHER_DIR})")
elseif(TETHER_DIR)
  message(STATUS "tplc-ec-pump: skipped (build Tether first or set TETHER_BUILD_DIR)")
endif()

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

# MCU static-link shim (plan/18-mcu-path.md §7 crit 3): both module
# images linked into one host binary — no dlopen, no ELF parsing, no
# files, no mmap, no threads. ABI symbols are macro-renamed per
# version so two modules coexist and resolve through a static table.
set(SHIM_ABI_SYMS abi_version get_descriptor init shutdown
    task_run migrate)
function(add_shim_module name ver)
  set(defs -DTPLC_STUB_V=${ver})
  foreach(s ${SHIM_ABI_SYMS})
    list(APPEND defs "-Dtplc_${s}=tplc_${name}_${s}")
  endforeach()
  add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${name}_shim.o
    COMMAND ${CMAKE_C_COMPILER} ${defs}
            -I${CMAKE_SOURCE_DIR}/include -I${CMAKE_SOURCE_DIR}/tests/abi
            -O2 -fno-exceptions -c ${CMAKE_SOURCE_DIR}/tests/abi/stub_module.c
            -o ${CMAKE_CURRENT_BINARY_DIR}/${name}_shim.o
    DEPENDS ${CMAKE_SOURCE_DIR}/tests/abi/stub_module.c)
endfunction()
add_shim_module(v1 1)
add_shim_module(v2 2)
add_executable(mcu_shim_host tests/abi/mcu_shim_host.c
  ${CMAKE_CURRENT_BINARY_DIR}/v1_shim.o
  ${CMAKE_CURRENT_BINARY_DIR}/v2_shim.o)
target_include_directories(mcu_shim_host PRIVATE include tests/abi)
add_test(NAME mcu_shim
  COMMAND bash ${CMAKE_SOURCE_DIR}/tests/abi/mcu_shim.sh
          $<TARGET_FILE:mcu_shim_host>)

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

# perf-evidence fixture: uncooperative spin module for watchdog
# latency measurement (tests/perf/evidence.sh)
add_library(hang_spin MODULE tests/perf/hang_spin.c)
set_target_properties(hang_spin PROPERTIES
  PREFIX "" OUTPUT_NAME "hang_spin" SUFFIX ".so")
target_include_directories(hang_spin PRIVATE include)
target_compile_options(hang_spin PRIVATE
  -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables)

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

# plan 20: update-auth strict + trust rotation/revocation
add_test(NAME update_auth
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/scan/update_auth.sh)
set_tests_properties(update_auth PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 90 RESOURCE_LOCK plc_scan)
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
set_tests_properties(mgmt_surface PROPERTIES SKIP_RETURN_CODE 77 RESOURCE_LOCK plc_scan)

# watchdog escalation: uncooperative task must fault the image flag
# and _exit at ~2x hang-timeout (needs hang_spin fixture)
add_dependencies(tether-plc hang_spin)
add_test(NAME watchdog_escalation COMMAND bash
  ${CMAKE_SOURCE_DIR}/tests/fault/watchdog_escalation.sh)
set_tests_properties(watchdog_escalation PROPERTIES SKIP_RETURN_CODE 77 RESOURCE_LOCK plc_scan)

# live attach over tether_io: tplc-io TCP bridge + probe client
if(TARGET tplc-io-probe)
  add_test(NAME tplc_io_attach COMMAND bash
    ${CMAKE_SOURCE_DIR}/tests/scan/tplc_io_attach.sh)
  set_tests_properties(tplc_io_attach PROPERTIES SKIP_RETURN_CODE 77 RESOURCE_LOCK plc_scan)
endif()

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
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # FBD gallery: ST/FBD pairs must produce identical %Q images
  add_test(NAME fbd_gallery_e2e
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/fbd_gallery/e2e.sh)
  set_tests_properties(fbd_gallery_e2e PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # shadow-run e2e (doc 19): candidate compare, isolation, promote
  add_test(NAME shadow_e2e
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/shadow/e2e.sh)
  set_tests_properties(shadow_e2e PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # debug-plane stress: boundary-consistent reads never tear a
  # multi-byte field mid-update (doc 12 §10.3)
  add_test(NAME debug_stress
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/debug/stress.sh)
  set_tests_properties(debug_stress PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # flight recorder: an induced task fault freezes the ring and the
  # pre-fault channel scans survive for TRACE_READ (doc 12 §10.5)
  # tplcc --target emits a monolithic object for a foreign triple
  add_test(NAME mcu_target
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/abi/mcu_target.sh)
  set_tests_properties(mcu_target PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 60)

  # ARM target_obj links into a bare-metal ELF (arm-none-eabi-gcc)
  add_test(NAME mcu_link
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/abi/mcu_link.sh)
  set_tests_properties(mcu_link PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 60)

  # ARM module executes init/task/io under qemu-arm user mode
  # (QEMU_ARM env or qemu-arm on PATH; skips without it)
  add_test(NAME mcu_exec
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/abi/mcu_exec.sh)
  set_tests_properties(mcu_exec PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120)

  # static-link shim cross-compiled for ARM and run under qemu-arm
  add_test(NAME mcu_shim_arm
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/abi/mcu_shim_arm.sh)
  set_tests_properties(mcu_shim_arm PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120)

  # TPLB PIC blob: tplcc --pic-blob → loader places + fixes up the
  # image with no target linker; module runs under qemu-arm
  add_test(NAME mcu_blob
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/abi/mcu_blob.sh)
  set_tests_properties(mcu_blob PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120)

  # library packages: tplcc build --library (kind=library manifest,
  # POU object, ST sources) + --lib source-inclusion consumption
  add_test(NAME library_pkg
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/pkg/library.sh)
  set_tests_properties(library_pkg PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120)

  # TwinCAT T0 compat libraries (plan 21 C1): compat/tc2/*.st as
  # --library packages consumed via --lib, replayed on the real runtime
  add_test(NAME tc2_compat
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2.sh)
  set_tests_properties(tc2_compat PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_TcpIp compat (plan 21 C2): tplc-net loopback e2e through the
  # real runtime (sockets, not mocks)
  add_test(NAME tc2_tcpip
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_tcpip.sh)
  set_tests_properties(tc2_tcpip PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_SerialCom compat (plan 21 C2): tplc-serial pty loopback e2e
  # through the real runtime (termios, not mocks)
  add_test(NAME tc2_serial
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_serial.sh)
  set_tests_properties(tc2_serial PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_System ADS compat (plan 21 C2): tplc-ads AMS/TCP exchange e2e
  # through the real runtime against a python ADS peer (wire protocol)
  add_test(NAME tc2_system_ads
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_system_ads.sh)
  set_tests_properties(tc2_system_ads PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc3_RealtimeMonitoring compat (plan 21 C3): tplc-rtmon mark ring
  # e2e through the real runtime (timestamped marks, count, indexed
  # reads, range error, clear)
  add_test(NAME tc3_rtmon
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc3_rtmon.sh)
  set_tests_properties(tc3_rtmon PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_ModbusSrv compat (plan 21 C4): tplc-modbus TCP exchange e2e
  # through the real runtime against a python Modbus peer (wire protocol)
  add_test(NAME tc2_modbussrv
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_modbussrv.sh)
  set_tests_properties(tc2_modbussrv PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_ModbusRTU compat (plan 21 C4): tplc-modbus RTU transport e2e
  # through the real runtime against a pty Modbus RTU slave peer
  add_test(NAME tc2_modbusrtu
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_modbusrtu.sh)
  set_tests_properties(tc2_modbusrtu PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc3_sACN compat (plan 21 C4): tplc-sacn E1.31 multicast source +
  # receiver e2e against a python sACN peer (priority preemption and
  # drop accounting)
  add_test(NAME tc3_sacn
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc3_sacn.sh)
  set_tests_properties(tc3_sacn PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc3_BACnet compat (plan 21 C4): tplc-bacnet BACnet/IP client e2e
  # (Who-Is/I-Am, ReadProperty, WriteProperty, Error) against a
  # python BACnet peer on UDP 47808
  add_test(NAME tc3_bacnet
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc3_bacnet.sh)
  set_tests_properties(tc3_bacnet PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc3_PJLink compat (plan 21 C4): tplc-pjlink PJLink projector
  # client e2e (POWR/INPT/AVMT, MD5 auth, ERRA, ERR1) against a
  # python PJLink peer on TCP 4352
  add_test(NAME tc3_pjlink
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc3_pjlink.sh)
  set_tests_properties(tc3_pjlink PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc3_S7Comm + Tc2_S5S7Com compat (plan 21 C4): tplc-s7
  # S7comm client e2e (connect/read/write/bit/s7err/connerr/
  # close) against a python S7 peer on TCP 1102
  add_test(NAME tc3_s7comm
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc3_s7comm.sh)
  set_tests_properties(tc3_s7comm PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc3_FTP + Tc2_FTP compat (plan 21 C4): tplc-ftp PASV client
  # e2e (connect/size/mkd/retr/list/stor/exists/rename/connerr/
  # close) against a python FTP peer on TCP 2121
  add_test(NAME tc3_ftp
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc3_ftp.sh)
  set_tests_properties(tc3_ftp PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_SMS + Tc2_SMTP compat (plan 21 C4): tplc-smtp RFC 5321
  # client + GSM AT SMS over tplc-serial e2e against python peers
  # (pty AT modem + TCP 2525 SMTP with AUTH LOGIN)
  add_test(NAME tc2_sms_sms
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_sms_sms.sh)
  set_tests_properties(tc2_sms_sms PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_IEC60870_5_10x compat (plan 21 C4): tplc-iec870
  # IEC 60870-5-104 client e2e (STARTDT/STOPDT/TESTFR, I/S/U
  # APCI, ASDU rx/tx, CP56Time2a, helpers) against a python
  # 104 slave peer on TCP 2480
  add_test(NAME tc3_iec870
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc3_iec870.sh)
  set_tests_properties(tc3_iec870 PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # tplc-motion / FB_MotionAxis e2e (plan 21 C5): simulated
  # CiA402 axis create, PP-position move, velocity mode, error
  # paths and destroy through tether-plc-sim; in-process, no peer
  add_test(NAME tc2_motion
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_motion.sh)
  set_tests_properties(tc2_motion PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_MC2 axis-management wrappers e2e (plan 21 C5): MC_Power /
  # MC_Reset / MC_SetPosition (abs+rel) / MC_Read* over the
  # simulated CiA402 axis; in-process, no peer
  add_test(NAME tc2_mc2
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_mc2.sh)
  set_tests_properties(tc2_mc2 PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_MC2 motion-command wrappers e2e (plan 21 C5): MC_Power /
  # MC_MoveAbsolute / MC_MoveRelative / MC_MoveVelocity / MC_Stop /
  # MC_Halt / MC_Home / MC_Jog over the simulated CiA402 axis;
  # in-process, no peer
  add_test(NAME tc2_mc2_motion
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_mc2_motion.sh)
  set_tests_properties(tc2_mc2_motion PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_MC2 tranche-4 wrappers e2e: MC_MoveModulo (shortest +
  # positive), MC_Write/ReadParameter, MC_Read/WriteBoolParameter,
  # MC_SetOverride, PLCopen operating-state machine, MC_Halt;
  # in-process, no peer
  add_test(NAME tc2_mc2_modulo
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_mc2_modulo.sh)
  set_tests_properties(tc2_mc2_modulo PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_MC2 tranche-5 wrappers e2e: two simulated axes (master +
  # slave), MC_TouchProbe position latch, MC_GearIn velocity-ratio
  # coupling, MC_GearOut, MC_Halt; in-process, no peer
  add_test(NAME tc2_mc2_gear
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_mc2_gear.sh)
  set_tests_properties(tc2_mc2_gear PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # Tc2_NCI channel-interpolation e2e: 3-axis group powered, a
  # coordinated FB_NciLine move streaming CSP setpoints, return move,
  # and a group E-stop; in-process, no peer
  add_test(NAME tc2_nci
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_nci.sh)
  set_tests_properties(tc2_nci PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # tplc-motion over the Tether IO protocol e2e: the same
  # FB_MotionAxis case replayed against tether_io_motion_server
  # (SimulatedCiA402Drive, --clock sim) with TPLC_MOTION_IO set;
  # skips when the Tether server binary is unavailable
  if(TARGET tplc_tether_io)
    add_test(NAME tc2_motion_io
      COMMAND ${CMAKE_COMMAND} -E env "TETHER_DIR=${TETHER_DIR}"
        bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_motion_io.sh)
    set_tests_properties(tc2_motion_io PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

    # the MC2 command wrappers driving the remote DS402 drive over
    # Tether IO; same case and mask as the in-process variant
    add_test(NAME tc2_mc2_motion_io
      COMMAND ${CMAKE_COMMAND} -E env "TETHER_DIR=${TETHER_DIR}"
        bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_mc2_motion_io.sh)
    set_tests_properties(tc2_mc2_motion_io PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

    # the tranche-4 MC2 wrappers (modulo/override/params/state)
    # driving the remote DS402 drive over Tether IO
    add_test(NAME tc2_mc2_modulo_io
      COMMAND ${CMAKE_COMMAND} -E env "TETHER_DIR=${TETHER_DIR}"
        bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_mc2_modulo_io.sh)
    set_tests_properties(tc2_mc2_modulo_io PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

    # the tranche-5 MC2 wrappers (2-axis gear coupling + touch probe)
    # driving the remote DS402 drive fleet over Tether IO
    add_test(NAME tc2_mc2_gear_io
      COMMAND ${CMAKE_COMMAND} -E env "TETHER_DIR=${TETHER_DIR}"
        bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_mc2_gear_io.sh)
    set_tests_properties(tc2_mc2_gear_io PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

    # the Tc2_NCI channel-interpolation surface (3-axis CSP line
    # move) driving the remote DS402 drive fleet over Tether IO
    add_test(NAME tc2_nci_io
      COMMAND ${CMAKE_COMMAND} -E env "TETHER_DIR=${TETHER_DIR}"
        bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_nci_io.sh)
    set_tests_properties(tc2_nci_io PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

    # the Tc2_NCI G-code channel over Tether IO: FB_Itp* wrappers
    # drive the server's nc.* channel (Tether-owned interpreter +
    # motion planner streaming CSP to the drive fleet)
    add_test(NAME tc2_nci_gcode_io
      COMMAND ${CMAKE_COMMAND} -E env "TETHER_DIR=${TETHER_DIR}"
        bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_nci_gcode_io.sh)
    set_tests_properties(tc2_nci_gcode_io PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 180 RESOURCE_LOCK plc_scan)

    # the OPC UA client surface (C7): FB_UA_* wrappers drive a real
    # open62541 session against the server's --opcua endpoint —
    # connect/status/read/write/ns-index/disconnect
    add_test(NAME tc2_opcua_io
      COMMAND ${CMAKE_COMMAND} -E env "TETHER_DIR=${TETHER_DIR}"
        bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_opcua_io.sh)
    set_tests_properties(tc2_opcua_io PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)
  endif()

  # EtherNet/IP client surface (C7): FB_Eip* wrappers drive a real
  # explicit-messaging session against tests/compat/eip_adapter_fake.py
  # (encapsulation + CIP Get/Set_Attribute_Single) — register,
  # identity, attribute r/w, disconnect
  add_test(NAME tc2_eip_io
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_eip_io.sh)
  set_tests_properties(tc2_eip_io PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # IEC 61850 client surface (C7): FB_Iec61850* wrappers drive a real
  # MMS session against iec61850_ied_fake (a libiec61850 IedServer with
  # a tetherIEDtetherTestLD/LLN0 model) — associate, status, read,
  # write-error path, name-list, disconnect.  The test needs the fake
  # target, which only exists when libiec61850 is on — otherwise the
  # script's own exit-77 skip handles the missing-binary case.
  if(TARGET iec61850_ied_fake)
    add_test(NAME tc2_iec61850_io
      COMMAND ${CMAKE_COMMAND} -E env
        "IEC61850_FAKE=$<TARGET_FILE:iec61850_ied_fake>"
        "BUILD=$<TARGET_FILE_DIR:tether-plc>"
        bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_iec61850_io.sh)
    set_tests_properties(tc2_iec61850_io PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)
  endif()

  # MQTT client surface (C7): FB_IotMqtt* wrappers drive a real
  # MQTT 3.1.1 session against the python fake broker
  # (mqtt_broker_fake.py — CONNECT/CONNACK/SUBACK/PUBLISH-echo) —
  # connect/subscribe/publish/echo/disconnect
  add_test(NAME tc2_mqtt_io
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_mqtt_io.sh)
  set_tests_properties(tc2_mqtt_io PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # OCPP-J client surface (C7): FB_Ocpp* wrappers drive a real
  # WebSocket+JSON-RPC session against the python CSMS fake
  # (ocpp_csms_fake.py — WS upgrade + OCPP-J echo) —
  # connect/call/result/disconnect
  add_test(NAME tc2_ocpp_io
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_ocpp_io.sh)
  set_tests_properties(tc2_ocpp_io PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # AES70/OCA client surface (C7): FB_Aes70* wrappers drive a real
  # OSCP+OCP session against the python device fake
  # (aes70_device_fake.py — OCP1 frames + OcaResponse echo) —
  # connect/keepalive/call/response/disconnect
  add_test(NAME tc2_aes70_io
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/compat/tc2_aes70_io.sh)
  set_tests_properties(tc2_aes70_io PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # REFERENCE TO: online change must warn and reset, never copy
  add_test(NAME ref_migration
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/online_change/ref_migration.sh)
  set_tests_properties(ref_migration PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 60 RESOURCE_LOCK plc_scan)

  add_test(NAME flight_recorder
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/fault/flight_recorder.sh)
  set_tests_properties(flight_recorder PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 60 RESOURCE_LOCK plc_scan)

  # error-path conformance (Appendix L): induced TPLC_E_* codes across
  # protocol frames, ctl ops, module stubs, package + image mutations
  add_test(NAME error_paths
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      bash ${CMAKE_SOURCE_DIR}/tests/scan/error_paths.sh)
  set_tests_properties(error_paths PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 60 RESOURCE_LOCK plc_scan)

  # object dictionary (doc 16 §7): LIST/READ/WRITE + entry authz
  add_test(NAME od_surface COMMAND bash
    ${CMAKE_SOURCE_DIR}/tests/scan/od_surface.sh
    $<TARGET_FILE:tether-plc> $<TARGET_FILE:tplc-image>
    $<TARGET_FILE:tplc-ctl> $<TARGET_FILE:stub_v1>)
  set_tests_properties(od_surface PROPERTIES
    SKIP_RETURN_CODE 77 RESOURCE_LOCK plc_scan)

  # sim vectors (doc 14 §3.1): replay the oracle corpus through
  # tether-plc-sim incl. named field expectations via ctl DBG_READ
  add_test(NAME sim_vectors
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      python3 ${CMAKE_SOURCE_DIR}/tests/sim/vector_run.py)
  set_tests_properties(sim_vectors PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 180 RESOURCE_LOCK plc_scan)

  # shadow-run vectors: load/compare/promote through the real runtime
  # (--scan) with the L1 interpreter as the oracle leg
  add_test(NAME sim_shadow
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      python3 ${CMAKE_SOURCE_DIR}/tests/sim/vector_run.py
      --vectors ${CMAKE_SOURCE_DIR}/tools/oracle/corpus/vectors_shadow.jsonl)
  set_tests_properties(sim_shadow PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 180 RESOURCE_LOCK plc_scan)

  # doc 19 §7.1: a shadowed active produces byte-identical %Q to a
  # solo run on the same tape
  add_test(NAME shadow_bitidentical
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/shadow/bitidentical.sh)
  set_tests_properties(shadow_bitidentical PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 120 RESOURCE_LOCK plc_scan)

  # no-alloc property (doc 14 §5): LD_PRELOAD counter — steady-state
  # scans must not hit the heap
  add_test(NAME no_alloc
    COMMAND bash ${CMAKE_SOURCE_DIR}/tests/runtime/no_alloc.sh)
  set_tests_properties(no_alloc PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 60 RESOURCE_LOCK plc_scan)

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

  # EtherCAT fleet e2e: TetherPLC master + PLC against the emulated
  # Beckhoff chain (sibling Tether beckhoff_slave_chain over UDP or a
  # veth pair); Tether IO stimuli/observations on both endpoints
  if(TARGET tplc-ec-pump AND TARGET tplc-io-cli)
    add_test(NAME ec_fleet_e2e
      COMMAND ${CMAKE_COMMAND} -E env
        "BUILD=$<TARGET_FILE_DIR:tether-plc>"
        bash ${CMAKE_SOURCE_DIR}/tests/ec/fleet_e2e.sh)
    set_tests_properties(ec_fleet_e2e PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 240 RESOURCE_LOCK plc_scan)
    # Same scenario over raw L2 frames on a veth pair (EC_IFACE mode);
    # skips when runec/ip or CAP_NET_ADMIN is unavailable
    add_test(NAME ec_fleet_e2e_veth
      COMMAND ${CMAKE_COMMAND} -E env
        "BUILD=$<TARGET_FILE_DIR:tether-plc>"
        "EC_IFACE=veth-tplc-ec"
        bash ${CMAKE_SOURCE_DIR}/tests/ec/fleet_e2e.sh)
    set_tests_properties(ec_fleet_e2e_veth PROPERTIES
      SKIP_RETURN_CODE 77 TIMEOUT 240 RESOURCE_LOCK plc_scan)
  endif()

  # determinism contract (doc 14 §2): same input sequence twice must
  # give identical traces at both legs
  add_test(NAME sim_determinism
    COMMAND ${CMAKE_COMMAND} -E env
      "BUILD=$<TARGET_FILE_DIR:tether-plc>"
      python3 ${CMAKE_SOURCE_DIR}/tests/sim/vector_run.py --repeat 2)
  set_tests_properties(sim_determinism PROPERTIES
    SKIP_RETURN_CODE 77 TIMEOUT 300 RESOURCE_LOCK plc_scan)

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
