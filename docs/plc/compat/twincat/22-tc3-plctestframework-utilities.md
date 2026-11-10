# Tc3_PlcTestFramework+Utilities

- **Manual:** `TF1140_Test_Framework_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** System · **Tier:** T1 · **API entries:** 28
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Test framework needs harness hooks into sim + runtime; utilities mostly T0.

---

#### (top level)

##### `Technical introduction` (FUNCTION_BLOCK) — §4

**Description:** The UnitTestFramework consists of a PLC library, which must be implemented into the test project, a test client with a user interface, and a test client without a user interface. The latter is used for command line calls and can be used to automatically run tests as part of a CI/CD pipeline. The following figure shows a schematic representation of the set-up.

#### PLC API › Tc3_PlcTestFramework › Function Blocks › FB_TestCaseBase

##### `Assert` (METHOD) — §7.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `condition` | `BOOL` | A BOOL value that indicates |

**Description:** The Assert method can be used to indicate that the current test case has failed. Unlike the AssertEqual method [} 18], which compares the result to the expected value on its own, the comparison here must be performed in the test case’s application code, and the result of the comparison must be passed to the method as a Boolean value. If you also want to pass an error message to the calling test client, use the AssertMSG method [} 19] .

##### `AssertAbsEqual` (METHOD) — §7.1.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `actual` | `LREAL` | Value determined by the test performed |
| `expected` | `LREAL` | Expected value for the test case. |
| `tolerance` | `LREAL` | Permissible tolerance for this test case between the expected value |

**Description:** Similar to the AssertEqual method, this method is also used to verify whether the expected value of a test case matches the actual result.

##### `AssertEqual` (METHOD) — §7.1.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `expected` | `ANY` | Expected value for the test case |
| `actual` | `ANY` | Value determined by executing the test case |
| `message` | `STRING(255)` | Error message if the test fails. |

**Description:** The AssertEqual method can be used to check the result of a test case and, if an error occurs, to display a custom error message. The comparison of the expected value with the result of the test case takes place within the AssertEqual method itself. The AssertEqual method is based on the ANY data type. Since the parameters must be passed as typed values for this reason, the expected value must first be assigned to a corresponding typed variable and cannot be assigned directly within the call to the AssertEqual method.

##### `AssertMsg` (METHOD) — §7.1.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `condition` | `BOOL` | A BOOL value that indicates |

**Description:** The AssertMsg method can be used to check the result of a test case and, if an error occurs, to display a custom error message. It thus extends the functionality of the Assert method to include an error message.

##### `AssertRelEqual` (METHOD) — §7.1.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `actual` | `LREAL` | Value determined by executing the |

**Description:** The method AssertRelEqual uses the method IsRelEqual to check whether two floating-point values (the test result and the expected value) fall within a defined relative or absolute tolerance. If this is not the case, the test case fails and returns the specified error message.

##### `AssertStrictLarger` (METHOD) — §7.1.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `actual` | `LREAL` | Value determined by executing the test case |
| `border` | `LREAL` | The threshold against which the |

**Description:** The AssertStrictlyLarger method tests whether a value (the result of a test) is strictly greater than a defined threshold.

##### `Cleanup` (METHOD) → `BOOL` — §7.1.1.1.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Cleanup` | `BOOL` | A return value that can be used to indicate that cleanup has been |

**Description:** Cleanup is a method that can be used to clean up the data for a test case. You must override and implement this method when it is needed.

##### `GetCurrentStep` (METHOD) → `UDINT` — §7.1.1.1.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCurrentStep` | `UDINT` | Returns the current state of the state machine if it is used in the |

**Description:** As described in the chapter FB_TestCaseBase [} 17], there are internal variables that can be used during the implementation of test cases. Among other things, the variable nCurStep is used as a status variable for a “Case” construct, which can be used to structure test cases in the form of a state machine. The method GetCurrentStep returns the current value of the status variable nCurStep. If you want to use a different status variable, you can override this method.

##### `GoToStep` (METHOD) — §7.1.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Step` | `UDINT` | Sets the current state of the state |

**Description:** As described in the chapter FB_TestCaseBase [} 17], there are internal variables that can be used during the implementation of test cases. Among other things, the variable nCurStep is used as a status variable for a “Case” construct, which can be used to structure test cases in the form of a state machine. The method

##### `IncStep` (METHOD) — §7.1.1.1.10

**Description:** As described in the chapter FB_TestCaseBase [} 17], there are internal variables that can be used during the implementation of test cases. Among other things, the variable nCurStep is used as a status variable for a “Case” construct, which can be used to structure test cases in the form of a state machine. The IncStep method increments the value of the variable nCurStep by 1. If a different status variable is to be used, this method must be overridden when it is to be used.

##### `IncStepIf` (METHOD) — §7.1.1.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `condition` | `BOOL` | If this value is TRUE, the current state of the state machine is |

**Description:** Like the IncStep method [} 22], this method increments the internal status variable of the “Case” construct by 1 when a condition is met. The condition itself is checked in the application code, and you must pass the Boolean result when calling the method. If a status variable other than nCurStep is to be used, this method must be overridden if it is to be applied.

##### `IsRelEqual` (METHOD) → `bool` — §7.1.1.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `actual` | `LREAL` | Value determined by executing the test case |
| `expected` | `LREAL` | Expected value for the test case |
| `precision` | `LREAL` | Relative tolerance for comparison |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsRelEqual` | `BOOL` | Returns the Boolean result of the |

**Description:** The IsRelEqual method checks whether two floating-point numbers of type “LREAL” fall within a specified tolerance. If both values are greater than the value specified in absCompScale, the relative deviation between the two values is checked against the tolerance. If any of the values fall below the tolerance, an absolute comparison is performed. Among other things, this method is used in the AssertRelEqual method [} 20].

##### `Succeeded` (METHOD) — §7.1.1.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `finalMessage` | `STRING(255)` | Final message when the “” |

**Description:** The succeeded method is called to confirm that the test case was executed successfully. The test case runs until either a Assert method detects that the test case has failed, the configured timeout stops the test case from running, or calling succeeded confirms that the test case has completed error-free.

#### PLC API › Tc3_PlcTestFramework › Function Blocks

##### `FB_TestCaseBasePrep` (FUNCTION_BLOCK) — §7.1.1.2

**Description:** FB_TestCaseBasePrep is an extension of FB_TestCaseBase [} 17]. When instantiating this function block, you can specify a function block of the type FB_TestPrepBase [} 24]. This makes it possible to move the steps into a separate function block in order to reach the operating point of a test case. It offers virtually the same methods for detecting errors to determine whether errors occur even before a test case reaches its operating point.

#### PLC API › Tc3_PlcTestFramework › Function Blocks › FB_TestPrepBase

##### `Assert` (METHOD) — §7.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `condition` | `BOOL` | A BOOL value that indicates |

**Description:** The Assert method can be used to indicate that one of the preparation steps for the following test case has failed. Unlike the AssertEqual method [} 26], which compares the result to the expected value on its own, the comparison here must be performed in the application code of the preparation step, and the result of the comparison must be passed to the method as a Boolean value. If an error message is also to be passed to the calling test client, the AssertMSG method [} 27] should be used.

##### `AssertAbsEqual` (METHOD) — §7.1.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `actual` | `LREAL` | Value determined by the test performed |
| `expected` | `LREAL` | Expected value for the test case. |
| `tolerance` | `LREAL` | Permissible tolerance for this test case between the expected value |

**Description:** Similar to the AssertEqual method, this method is also used to verify whether the expected value of a test preparation step matches the actual result.

##### `AssertEqual` (METHOD) — §7.1.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `expected` | `ANY` | Expected value for the test case |
| `actual` | `ANY` | Value determined by executing the test case |
| `message` | `STRING(255)` | Error message if the test fails. |

**Description:** The AssertEqual method can be used to verify the result of a test preparation step and, if an error occurs, to display a custom error message. The comparison of the expected value with the result of the preparation step takes place within the AssertEqual method itself. The AssertEqual method is based on the ANY data type. Since the parameters must be passed as typed values for this reason, the expected value must first be assigned to a corresponding typed variable and cannot be assigned directly within the call to the AssertEqual method.

##### `AssertMsg` (METHOD) — §7.1.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `condition` | `BOOL` | A BOOL value that indicates |

**Description:** The AssertMsg method can be used to verify the result of a test preparation step and, if an error occurs, to display a custom error message. It therefore extends the functionality of the “Assert” method to include an error message.

##### `AssertRelEqual` (METHOD) — §7.1.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `actual` | `LREAL` | Value determined by executing the |

**Description:** The AssertRelEqual method uses the IsRelEqual method to check whether two floating-point values (the result of the preparation step and the expected value) fall within a defined relative or absolute tolerance. If this is not the case, the test case fails and returns the specified error message.

##### `AssertStrictLarger` (METHOD) — §7.1.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `actual` | `LREAL` | Value determined by executing the test case |
| `border` | `LREAL` | The threshold against which the |

**Description:** The AssertStrictlyLarger method tests whether a value (the result of a test) is strictly greater than a defined threshold.

##### `Cleanup` (METHOD) → `BOOL` — §7.1.1.3.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Cleanup` | `BOOL` | A return value that can be used to indicate that cleanup has been |

**Description:** Cleanup is a method that can be used to “clean up” the data from a preparation step, provided that this data is not needed for the test case itself. You must override and implement this method when it is needed.

##### `GetCurrentStep` (METHOD) → `UDINT` — §7.1.1.3.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCurrentStep` | `UDINT` | Returns the current state of the state machine if it is used in the |

**Description:** As described in the FB_TestPrepBase [} 24] chapter, there are internal variables that can be used during the implementation of the preparation steps. Among other things, the variable nCurStep is used as a status variable for a “Case” construct, which can be used to structure the state machine so that it reaches the operating point of a test case. The method GetCurrentStep returns the current value of the status variable nCurStep. If you want to use a different status variable, you can override this method.

##### `GoToStep` (METHOD) — §7.1.1.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Step` | `UDINT` | Sets the current state of the state |

**Description:** As described in the chapter FB_TestPrepBase [} 24], there are internal variables that can be used during the implementation of the preparation step. Among other things, the variable nCurStep is used as a status variable for a “Case” construct, which can be used to structure the state machine so that it reaches the operating point of a test case. The GoToStep method sets the value of the status variable nCurStep to the value passed to it. This means that in the next iteration of the “Case” construct, the program jumps to exactly this step. If a different status variable is to be used, this method must be overridden when needed.

##### `IncStep` (METHOD) — §7.1.1.3.10

**Description:** As described in the FB_TestPrepBase [} 24] chapter, there are internal variables that can be used during the implementation of the preparation steps. Among other things, the variable nCurStep is used as a status variable for a “Case” construct, which can be used to structure the preparatory steps in the form of a state machine. The IncStep method increments the value of the nCurStep variable by 1. If a different status variable is to be used, this method must be overridden when it is to be used.

##### `IncStepIf` (METHOD) — §7.1.1.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `condition` | `BOOL` | If this value is TRUE, the current state of the state machine is |

**Description:** Like the IncStep method [} 29], this method increments the internal status variable of the “Case” construct by 1 when a condition is met. The condition itself is checked in the application code, and you must pass the Boolean result when calling the method. If a status variable other than nCurStep is to be used, this method must be overridden if it is to be applied.

##### `IsRelEqual` (METHOD) → `bool` — §7.1.1.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `actual` | `LREAL` | Value determined by executing the test case |
| `expected` | `LREAL` | Expected value for the test case |
| `precision` | `LREAL` | Relative tolerance for comparison |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsRelEqual` | `BOOL` | Returns the Boolean result of the |

**Description:** The IsRelEqual method checks whether two floating-point numbers of type “LREAL” fall within a specified tolerance.

##### `SucceededPreparation` (METHOD) — §7.1.1.3.13

**Description:** The method SucceededPreparation is called to confirm that test case preparation has been successfully completed. The preparation steps are executed until either an Assert method detects that a preparation step has failed, the configured timeout stops the test case preparation process, or a call to SucceededPreparation confirms that the test case preparation has completed error-free and the test case can now be executed.
