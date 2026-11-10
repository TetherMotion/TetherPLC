# Tc3_Speech

- **Manual:** `TF4500_TC3_Speech_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Control · **Tier:** T4 · **API entries:** 8
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Speech recognition/synthesis needs an ASR/TTS engine; no such facility exists.

---

#### SPS API › Function Blocks

##### `FB_Play` (FUNCTION_BLOCK) — §9.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPlay` | `BOOL` | Rising edge starts the playback. |
| `sPath` | `STRING` | File path to the audio file. 16 kHz sample rate, 16 bit sample |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Function block is busy. |
| `bError` | `BOOL` | Function block is in error state. |
| `nErrorId` | `ETcsSpeechCommandExit ErrorId, which indicates the exact reason for the error. Code [} 56]` | — |
| `nLastComman UINT` | `Return value of the last call. dExitCode` | — |
| `nPlaybackPosit ULINT` | `Current position of the speech output. ion` | — |
| `nPlaybackTotal ULINT` | `Total length of the speech output. SPS API Methods` | — |
| `SetAmsNetAddr [} 59]` | `Method to specify a different AmsNetAdr for the TcSpeech service. Only` | — |

**Description:** Playback function block

#### SPS API › Function Blocks › FB_Play

##### `SetAmsNetAddr` (METHOD) → `HRESULT` — §9.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAmsNetId` | `STRING` | AmsNetId of destination TcSpeech service. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Method to be called once to specifiy the AmsNetId of destination TcSpeech service.

#### SPS API › Function Blocks

##### `FB_SetRuleState` (FUNCTION_BLOCK) — §9.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | Rising edge sets the given RuleId. |
| `nConfigurationI UINT` | `ConfigurationID to be used (see TcSpeech Configurator). d` | — |
| `sGrammarFile STRING` | `Path to the SRGS file on the target that is to be used. If not` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Function block is busy. |
| `bError` | `BOOL` | Function block is in error state. |
| `nErrorId` | `ETcsSpeechCommandExit ErrorId, which indicates the exact reason for the error. Code [} 56]` | — |
| `nLastComman UINT` | `Return value of the last call. dExitCode Methods` | — |
| `SetAmsNetAddr [} 60]` | `Method to specify a different AmsNetAdr for the TcSpeech service. Only` | — |

**Description:** bGrammarActivate BOOL

#### SPS API › Function Blocks › FB_SetRuleState

##### `SetAmsNetAddr` (METHOD) → `HRESULT` — §9.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAmsNetId` | `STRING SPS API` | AmsNetId of destination TcSpeech service. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Method to be called once to specifiy the AmsNetId of destination TcSpeech service.

#### SPS API › Function Blocks

##### `FB_SpeechRecognition` (FUNCTION_BLOCK) — §9.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bLists` | `BOOL` | Rising edge starts speech recognition. |
| `nConfigurationI UINT` | `ConfigurationID to be used for speech recognition. d` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Function block is busy. |
| `bError` | `BOOL` | Function block is in error state. |
| `nErrorId` | `ETcsSpeechCommandExit ErrorId, which indicates the exact reason for the error. Code [} 56]` | — |

**Description:** Automatic Speech Recognition function block

#### SPS API › Function Blocks › FB_SpeechRecognition

##### `SetAmsNetAddr` (METHOD) → `HRESULT` — §9.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAmsNetId` | `STRING` | AmsNetId of destination TcSpeech service. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT SPS API` | `` | — |

**Description:** Method to be called once to specifiy the AmsNetId of destination TcSpeech service.

#### SPS API › Function Blocks

##### `FB_TextToSpeech` (FUNCTION_BLOCK) — §9.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSpeak` | `BOOL` | Rising edge starts a speech command. |
| `sUtterance` | `STRING` | Utterance for the speech command |
| `nConfigurationI UINT` | `ConfigurationID to be used for output (see TcSpeech` | — |
| `d` | `Configurator)` | — |
| `nLanguageId UINT` | `LanguageId which is to be used. '0' for configured default` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Function block is busy. |
| `bError` | `BOOL` | Function block is in error state. |
| `nErrorId` | `ETcsSpeechCommandExit ErrorId, which indicates the exact reason for the error. Code [} 56]` | — |
| `nLastComman UINT` | `Return value of the last call. dExitCode` | — |
| `nPlaybackPosit ULINT` | `Current position of the speech output. ion` | — |
| `nPlaybackTotal ULINT` | `Total length of the speech output. Methods` | — |
| `SetAmsNetAddr [} 64]` | `Method to specify a different AmsNetAdr for the TcSpeech service. Only` | — |

**Description:** Text to Speech function block.

#### SPS API › Function Blocks › FB_TextToSpeech

##### `SetAmsNetAddr` (METHOD) → `HRESULT` — §9.2.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAmsNetId` | `STRING` | AmsNetId of destination TcSpeech service. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT Samples` | `` | — |

**Description:** Method to be called once to specifiy the AmsNetId of destination TcSpeech service.
